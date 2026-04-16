use std::path::Path;
use rusqlite::{params, Connection, OptionalExtension, Result};
use super::songtag::SongTag;

pub fn create_db(conn: &Connection) -> Result<()> {
    conn.execute("PRAGMA foreign_keys = ON;", [])?;

    let schema = include_str!("schema.sql");
    conn.execute_batch(schema)?;
    Ok(())
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
