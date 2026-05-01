use rusqlite::{params_from_iter, Connection, Result};
use crate::model::db::{self, AlbumSummary, AlbumWithSongs};

#[derive(Debug, Clone)]
pub struct SearchQuery {
	pub artists: Vec<String>,
	pub albums: Vec<String>,
	pub songs: Vec<String>,
}

fn album_query_sql(q: &SearchQuery) -> (String, Vec<String>) {
	let mut sql = String::from(
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
		 LEFT JOIN rolas r ON r.id_album = a.id_album"
	);
	let mut params = Vec::new();
	let mut clauses = Vec::new();

	if !q.artists.is_empty() {
		let placeholders = vec!["?"; q.artists.len()].join(", ");
		clauses.push(format!(
			"EXISTS (SELECT 1
			          FROM rolas r_artist
			          JOIN performers p_artist ON p_artist.id_performer = r_artist.id_performer
			          WHERE r_artist.id_album = a.id_album
			            AND p_artist.name COLLATE NOCASE IN ({}))",
			placeholders
		));
		params.extend(q.artists.iter().cloned());
	}

	if !q.albums.is_empty() {
		let placeholders = vec!["?"; q.albums.len()].join(", ");
		clauses.push(format!("a.name COLLATE NOCASE IN ({})", placeholders));
		params.extend(q.albums.iter().cloned());
	}

	if !q.songs.is_empty() {
		let placeholders = vec!["?"; q.songs.len()].join(", ");
		clauses.push(format!(
			"EXISTS (SELECT 1
			          FROM rolas r_song
			          WHERE r_song.id_album = a.id_album
			            AND r_song.title COLLATE NOCASE IN ({}))",
			placeholders
		));
		params.extend(q.songs.iter().cloned());
	}

	if !clauses.is_empty() {
		sql.push_str(" WHERE ");
		sql.push_str(&clauses.join(" OR "));
	}

	sql.push_str(
		" GROUP BY a.id_album, a.name, a.path, a.year
		  ORDER BY songs DESC, a.name COLLATE NOCASE ASC",
	);

	(sql, params)
}

pub fn search(conn: &Connection, q: &SearchQuery) -> Result<Vec<AlbumWithSongs>> {
	if q.artists.is_empty() && q.albums.is_empty() && q.songs.is_empty() {
		return db::get_albums_with_songs(conn);
	}

	let (sql, params) = album_query_sql(q);
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(params), |row| {
		Ok(AlbumSummary {
			id_album: row.get(0)?,
			name: row.get(1)?,
			path: row.get(2)?,
			year: row.get(3)?,
			songs: row.get(4)?,
			artist: row.get(5)?,
		})
	})?;

	let mut result = Vec::new();
	for album in rows {
		let album = album?;
		let song_list = db::get_songs_for_album(conn, album.id_album)?;
		result.push(AlbumWithSongs {
			id_album: album.id_album,
			name: album.name,
			path: album.path,
			year: album.year,
			songs: album.songs,
			artist: album.artist,
			song_list,
		});
	}

	Ok(result)
}

#[cfg(test)]
mod tests {
	use super::*;
	use rusqlite::Connection;
	use crate::model::{db, songtag::SongTag};

	fn sample_song(artist: &str, album: &str, title: &str, path: &str) -> SongTag {
		SongTag {
			file_path: path.to_string(),
			title: title.to_string(),
			artist: artist.to_string(),
			album: album.to_string(),
			genre: "Rock".to_string(),
			track: Some(1),
			year: Some(1991),
		}
	}

	#[test]
	fn search_matches_artist_and_album_filters() -> rusqlite::Result<()> {
		let conn = Connection::open_in_memory()?;
		db::create_db(&conn)?;
		db::upsert_song(&conn, &sample_song("Nirvana", "Nevermind", "Smells Like Teen Spirit", "/tmp/n1.mp3"))?;
		db::upsert_song(&conn, &sample_song("Radiohead", "OK Computer", "Paranoid Android", "/tmp/r1.mp3"))?;

		let q = SearchQuery {
			artists: vec!["Nirvana".to_string()],
			albums: vec![],
			songs: vec![],
		};
		let res = search(&conn, &q)?;
		assert_eq!(res.len(), 1);
		assert_eq!(res[0].artist, "Nirvana");
		Ok(())
	}

	#[test]
	fn search_can_mix_artists_and_albums() -> rusqlite::Result<()> {
		let conn = Connection::open_in_memory()?;
		db::create_db(&conn)?;
		db::upsert_song(&conn, &sample_song("Nirvana", "Nevermind", "In Bloom", "/tmp/n2.mp3"))?;
		db::upsert_song(&conn, &sample_song("Nirvana", "Bleach", "About a Girl", "/tmp/n3.mp3"))?;
		db::upsert_song(&conn, &sample_song("Pearl Jam", "Ten", "Alive", "/tmp/p1.mp3"))?;

		let q = SearchQuery {
			artists: vec!["Nirvana".to_string()],
			albums: vec!["Ten".to_string()],
			songs: vec![],
		};
		let res = search(&conn, &q)?;
		assert_eq!(res.len(), 3);

		Ok(())
	}
}
