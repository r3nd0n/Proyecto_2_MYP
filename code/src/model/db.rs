use std::path::Path;
use rusqlite::{params, Connection, OptionalExtension, Result};
use super::songtag::SongTag;

#[derive(Clone, Debug)]
pub struct AlbumSummary {
    pub id_album: i64,
    pub name: String,
    pub path: String,
    pub songs: i64,
    pub artist: String,
}

#[derive(Clone, Debug)]
pub struct SongSummary {
    pub title: String,
    pub track: Option<i64>,
    pub genre: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AlbumWithSongs {
    pub id_album: i64,
    pub name: String,
    pub path: String,
    pub songs: i64,
    pub artist: String,
    pub song_list: Vec<SongSummary>,
}

/// Inicializa la BD ejecutando schema.sql
/// 
/// Activa las foreign keys de SQLite y ejecuta el esquema definido en
/// schema.sql para asegurar que las tablas y relaciones existan antes
/// de usar la BD.
pub fn create_db(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    let schema = include_str!("schema.sql");
    conn.execute_batch(schema)?;
    Ok(())
}


/// Inserta o actualiza una cancion en la base de datos.
/// 
/// Primero obtiene o crea el artista y el album asociados. Luego busca si ya
/// existe una rola con el mismo path. Si existe, actualiza sus datos; si no,
/// inserta una nueva fila.
pub fn upsert_song(conn: &Connection, song: &SongTag) -> Result<()> {
    let performer_id = get_or_create_performer(conn, &song.artist)?;
    let album_id = get_or_create_album(conn, song)?;

    let existing_song_id: Option<i64> = conn
        .query_row(
            "SELECT id_rola FROM rolas WHERE path = ?1 LIMIT 1",
            [&song.file_path],
            |row| row.get(0),
        )
        .optional()?;

    if let Some(song_id) = existing_song_id {
        conn.execute(
            "UPDATE rolas
             SET id_performer = ?1,
                 id_album = ?2,
                 title = ?3,
                 track = ?4,
                 year = ?5,
                 genre = ?6
             WHERE id_rola = ?7",
            params![
                performer_id,
                album_id,
                song.title,
                song.track,
                song.year,
                song.genre,
                song_id
            ],
        )?;
    } else {
        conn.execute(
            "INSERT INTO rolas (id_performer, id_album, path, title, track, year, genre)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                performer_id,
                album_id,
                song.file_path,
                song.title,
                song.track,
                song.year,
                song.genre
            ],
        )?;
    }

    Ok(())
}

/// Obtiene un listado resumido de albumes.
///
/// Devuelve una colección de AlbumSummary con el id del album, nombre,
/// ruta, numero de canciones y nombre del artista principal o concatenado.
pub fn get_albums(conn: &Connection) -> Result<Vec<AlbumSummary>> {
    let mut stmt = conn.prepare(
    "SELECT a.id_album,
                a.name,
                a.path,
                a.year,
                COUNT(r.id_rola) AS songs,
                COALESCE((SELECT GROUP_CONCAT(DISTINCT p.name)
                          FROM rolas r2
                          LEFT JOIN performers p ON r2.id_performer = p.id_performer
                          WHERE r2.id_album = a.id_album), 'Unknown') AS artist
         FROM albums a
         LEFT JOIN rolas r ON r.id_album = a.id_album
         GROUP BY a.id_album, a.name, a.path, a.year
         ORDER BY songs DESC, a.name COLLATE NOCASE ASC",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(AlbumSummary {
            id_album: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            songs: row.get(4)?,
            artist: row.get(5)?,
        })
    })?;

    rows.collect()
}

/// Obtiene todas las canciones de un álbum específico.
///
/// Recupera las canciones asociadas al album_id recibido y las ordena por pista
/// y luego por título para mostrarlas de forma consistente en la UI.
pub fn get_songs_for_album(conn: &Connection, album_id: i64) -> Result<Vec<SongSummary>> {
    let mut stmt = conn.prepare(
        "SELECT title, path, track, year, genre
         FROM rolas
         WHERE id_album = ?1
         ORDER BY track IS NULL, track ASC, title COLLATE NOCASE ASC",
    )?;

    let rows = stmt.query_map([album_id], |row| {
        Ok(SongSummary {
            title: row.get(0)?,
            track: row.get(2)?,
            genre: row.get(4)?,
        })
    })?;

    rows.collect()
}

/// Obtiene albumes completos con su lista de canciones.
///
/// Usa get_albums para obtener el resumen de albumes y luego completa cada
/// uno con su lista de canciones mediante get_songs_for_album.
pub fn get_albums_with_songs(conn: &Connection) -> Result<Vec<AlbumWithSongs>> {
    let albums = get_albums(conn)?;
    let mut result = Vec::with_capacity(albums.len());

    for album in albums {
        let song_list = get_songs_for_album(conn, album.id_album)?;
        result.push(AlbumWithSongs {
            id_album: album.id_album,
            name: album.name,
            path: album.path,
            songs: album.songs,
            artist: album.artist,
            song_list,
        });
    }

    Ok(result)
}

/// Busca un artista por nombre y, si no existe, lo crea.
///
/// Devuelve el id_performer correspondiente al nombre recibido. Se usa para
/// asegurar que cada cancion quede vinculada a un artista valido.
fn get_or_create_performer(conn: &Connection, artist: &str) -> Result<i64> {
    if let Some(id) = conn
        .query_row(
            "SELECT id_performer FROM performers WHERE name = ?1 LIMIT 1",
            [artist],
            |row| row.get(0),
        )
        .optional()?
    {
        return Ok(id);
    }

    let id_type = if artist == "Unknown" { 2 } else { 0 };
    conn.execute(
        "INSERT INTO performers (id_type, name) VALUES (?1, ?2)",
        params![id_type, artist],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Busca un album por ruta, nombre y año, y si no existe lo crea.
///
/// La ruta se deriva del archivo de la cancion. Si el album ya existe, devuelve
/// su id; si no, inserta uno nuevo y retorna el identificador generado.
fn get_or_create_album(conn: &Connection, song: &SongTag) -> Result<i64> {
    let album_path = Path::new(&song.file_path)
        .parent()
        .map(|value| value.display().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    if let Some(id) = conn
        .query_row(
            "SELECT id_album
             FROM albums
             WHERE path = ?1
               AND name = ?2
               AND (year = ?3 OR (year IS NULL AND ?3 IS NULL))
             LIMIT 1",
            params![album_path, song.album, song.year],
            |row| row.get(0),
        )
        .optional()?
    {
        return Ok(id);
    }

    conn.execute(
        "INSERT INTO albums (path, name, year) VALUES (?1, ?2, ?3)",
        params![album_path, song.album, song.year],
    )?;
    Ok(conn.last_insert_rowid())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_song(
        artist: &str,
        album: &str,
        title: &str,
        path: &str,
        track: Option<i32>,
    ) -> SongTag {
        SongTag {
            file_path: path.to_string(),
            title: title.to_string(),
            artist: artist.to_string(),
            album: album.to_string(),
            genre: "Rock".to_string(),
            track,
            year: Some(1991),
        }
    }

    #[test]
    fn create_db_creates_schema_tables() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_db(&conn)?;

        let table_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('albums', 'performers', 'rolas')",
            [],
            |row| row.get(0),
        )?;

        assert_eq!(table_count, 3);
        Ok(())
    }

    #[test]
    fn upsert_song_inserts_and_updates_same_path() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_db(&conn)?;

        let first = sample_song(
            "Nirvana",
            "Nevermind",
            "Smells Like Teen Spirit",
            "/tmp/music/nirvana/song.mp3",
            Some(1),
        );
        upsert_song(&conn, &first)?;

        let updated = sample_song(
            "Nirvana",
            "Nevermind",
            "Come As You Are",
            "/tmp/music/nirvana/song.mp3",
            Some(2),
        );
        upsert_song(&conn, &updated)?;

        let rows: i64 = conn.query_row("SELECT COUNT(*) FROM rolas", [], |row| row.get(0))?;
        assert_eq!(rows, 1);

        let title: String = conn.query_row(
            "SELECT title FROM rolas WHERE path = ?1",
            ["/tmp/music/nirvana/song.mp3"],
            |row| row.get(0),
        )?;
        assert_eq!(title, "Come As You Are");

        Ok(())
    }

    #[test]
    fn get_albums_and_songs_return_expected_data() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        create_db(&conn)?;

        upsert_song(
            &conn,
            &sample_song(
                "Red Hot Chili Peppers",
                "Californication",
                "Around the World",
                "/tmp/rhcp/californication/01.mp3",
                Some(1),
            ),
        )?;
        upsert_song(
            &conn,
            &sample_song(
                "Red Hot Chili Peppers",
                "Californication",
                "Scar Tissue",
                "/tmp/rhcp/californication/02.mp3",
                Some(2),
            ),
        )?;

        let albums = get_albums(&conn)?;
        assert_eq!(albums.len(), 1);
        assert_eq!(albums[0].name, "Californication");
        assert_eq!(albums[0].songs, 2);
        assert_eq!(albums[0].artist, "Red Hot Chili Peppers");

        let songs = get_songs_for_album(&conn, albums[0].id_album)?;
        assert_eq!(songs.len(), 2);
        assert_eq!(songs[0].title, "Around the World");
        assert_eq!(songs[0].track, Some(1));
        assert_eq!(songs[1].title, "Scar Tissue");
        assert_eq!(songs[1].track, Some(2));

        let albums_with_songs = get_albums_with_songs(&conn)?;
        assert_eq!(albums_with_songs.len(), 1);
        assert_eq!(albums_with_songs[0].song_list.len(), 2);

        Ok(())
    }
}
