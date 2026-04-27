use std::path::Path;
use rusqlite::{params, Connection, OptionalExtension, Result};
use super::songtag::SongTag;

#[derive(Clone, Debug)]
pub struct AlbumSummary {
    pub id_album: i64,
    pub name: String,
    pub path: String,
    pub year: Option<i64>,
    pub songs: i64,
}

#[derive(Clone, Debug)]
pub struct SongSummary {
    pub title: String,
    pub path: String,
    pub track: Option<i64>,
    pub year: Option<i64>,
    pub genre: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AlbumWithSongs {
    pub id_album: i64,
    pub name: String,
    pub path: String,
    pub year: Option<i64>,
    pub songs: i64,
    pub song_list: Vec<SongSummary>,
}

pub fn create_db(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    let schema = include_str!("schema.sql");
    conn.execute_batch(schema)?;
    Ok(())
}



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

pub fn get_albums(conn: &Connection) -> Result<Vec<AlbumSummary>> {
    let mut stmt = conn.prepare(
    "SELECT a.id_album,
                a.name,
                a.path,
                a.year,
                COUNT(r.id_rola) AS songs
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
            year: row.get(3)?,
            songs: row.get(4)?,
        })
    })?;

    rows.collect()
}

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
            path: row.get(1)?,
            track: row.get(2)?,
            year: row.get(3)?,
            genre: row.get(4)?,
        })
    })?;

    rows.collect()
}

pub fn get_albums_with_songs(conn: &Connection) -> Result<Vec<AlbumWithSongs>> {
    let albums = get_albums(conn)?;
    let mut result = Vec::with_capacity(albums.len());

    for album in albums {
        let song_list = get_songs_for_album(conn, album.id_album)?;
        result.push(AlbumWithSongs {
            id_album: album.id_album,
            name: album.name,
            path: album.path,
            year: album.year,
            songs: album.songs,
            song_list,
        });
    }

    Ok(result)
}

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
