use rusqlite::{params_from_iter, Connection, Result};
use crate::model::db::{self, AlbumSummary, AlbumWithSongs};

fn album_query_sql_from_input(input: &str) -> (String, Vec<String>) {
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

	let mut params: Vec<String> = Vec::new();
	let mut group_clauses: Vec<String> = Vec::new();

	for raw_group in input.split('&') {
		let mut token_clauses: Vec<String> = Vec::new();

		for token in raw_group.split(',') {
			let value = token.trim();
			if value.is_empty() {
				continue;
			}

			let lower = value.to_lowercase();

			if lower.starts_with("ar:") {
				let v = value["ar:".len()..].trim().to_string();
				if !v.is_empty() {
					token_clauses.push("EXISTS (SELECT 1
						  FROM rolas r_artist
						  JOIN performers p_artist ON p_artist.id_performer = r_artist.id_performer
						  WHERE r_artist.id_album = a.id_album
							AND p_artist.name COLLATE NOCASE = ?)
						".to_string());
					params.push(v);
				}
			} else if lower.starts_with("al:") {
				let v = value["al:".len()..].trim().to_string();
				if !v.is_empty() {
					token_clauses.push("a.name COLLATE NOCASE = ?".to_string());
					params.push(v);
				}
			} else if lower.starts_with("ca:") {
				let v = value["ca:".len()..].trim().to_string();
				if !v.is_empty() {
					token_clauses.push("EXISTS (SELECT 1
						  FROM rolas r_song
						  WHERE r_song.id_album = a.id_album
							AND r_song.title COLLATE NOCASE = ?)
						".to_string());
					params.push(v);
				}
			}
		}

		if !token_clauses.is_empty() {
			group_clauses.push(format!("({})", token_clauses.join(" OR ")));
		}
	}

	if !group_clauses.is_empty() {
		sql.push_str(" WHERE ");
		sql.push_str(&group_clauses.join(" AND "));
	}

	sql.push_str(
		" GROUP BY a.id_album, a.name, a.path, a.year
		  ORDER BY songs DESC, a.name COLLATE NOCASE ASC",
	);

	(sql, params)
}

pub fn search_from_raw(conn: &Connection, raw: &str) -> Result<Vec<AlbumWithSongs>> {
	let (sql, params) = album_query_sql_from_input(raw);
	let mut stmt = conn.prepare(&sql)?;
	let rows = stmt.query_map(params_from_iter(params.clone()), |row| {
		let _ignored_year: Option<i64> = row.get(3)?;
		Ok(AlbumSummary {
			id_album: row.get(0)?,
			name: row.get(1)?,
			path: row.get(2)?,
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
	fn search_from_raw_matches_artist_and_album_filters() -> rusqlite::Result<()> {
		let conn = Connection::open_in_memory()?;
		db::create_db(&conn)?;
		db::upsert_song(&conn, &sample_song("Nirvana", "Nevermind", "Smells Like Teen Spirit", "/tmp/n1.mp3"))?;
		db::upsert_song(&conn, &sample_song("Radiohead", "OK Computer", "Paranoid Android", "/tmp/r1.mp3"))?;

		let res = search_from_raw(&conn, "ar: Nirvana")?;
		assert_eq!(res.len(), 1);
		assert_eq!(res[0].artist, "Nirvana");
		Ok(())
	}

	#[test]
	fn search_from_raw_applies_and_between_groups() -> rusqlite::Result<()> {
		let conn = Connection::open_in_memory()?;
		db::create_db(&conn)?;
		db::upsert_song(&conn, &sample_song("Red Hot Chili Peppers", "Californication", "Californication", "/tmp/rhcp.mp3"))?;
		db::upsert_song(&conn, &sample_song("Red Hot Chili Peppers", "Blood Sugar Sex Magik", "Under the Bridge", "/tmp/rhcp2.mp3"))?;

		let res = search_from_raw(&conn, "ar: Red Hot Chili Peppers & al: Californication")?;
		assert_eq!(res.len(), 1);
		assert_eq!(res[0].name, "Californication");

		Ok(())
	}
}
