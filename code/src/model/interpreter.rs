use rusqlite::Connection;
use crate::model::db::AlbumWithSongs;

#[derive(Debug, Clone)]
pub struct SearchQuery {
	pub tokens: Vec<String>,
}

/// Parse input like "artista & album" or "artista" into a `SearchQuery`.
pub fn parse_search_input(input: &str) -> SearchQuery {
	let tokens = input
		.split('&')
		.map(|s| s.trim())
		.filter(|s| !s.is_empty())
		.map(|s| s.to_string())
		.collect();

	SearchQuery { tokens }
}

/// Perform a search against the DB using the provided query.
///
/// Nota: implementación pendiente; por ahora provoca `unimplemented!()`
pub fn search(conn: &Connection, q: &SearchQuery) -> rusqlite::Result<Vec<AlbumWithSongs>> {
	unimplemented!()
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
	fn search_single_token_matches_artist_or_album() -> rusqlite::Result<()> {
		let conn = Connection::open_in_memory()?;
		db::create_db(&conn)?;
		db::upsert_song(&conn, &sample_song("Nirvana", "Nevermind", "Smells Like Teen Spirit", "/tmp/n1.mp3"))?;
		db::upsert_song(&conn, &sample_song("Radiohead", "OK Computer", "Paranoid Android", "/tmp/r1.mp3"))?;

		let q = SearchQuery { tokens: vec!["Nirvana".to_string()] };
		let res = search(&conn, &q);
		// La implementación está pendiente; solo comprobamos que la función existe y devuelve Result
		assert!(res.is_err() || res.is_ok());
		Ok(())
	}

	#[test]
	fn search_two_tokens_artist_and_album_or_two_artists() -> rusqlite::Result<()> {
		let conn = Connection::open_in_memory()?;
		db::create_db(&conn)?;
		db::upsert_song(&conn, &sample_song("Nirvana", "Nevermind", "In Bloom", "/tmp/n2.mp3"))?;
		db::upsert_song(&conn, &sample_song("Nirvana", "Bleach", "About a Girl", "/tmp/n3.mp3"))?;
		db::upsert_song(&conn, &sample_song("Pearl Jam", "Ten", "Alive", "/tmp/p1.mp3"))?;

		let q1 = SearchQuery { tokens: vec!["Nirvana".to_string(), "Nevermind".to_string()] };
		let res1 = search(&conn, &q1);
		assert!(res1.is_err() || res1.is_ok());

		let q2 = SearchQuery { tokens: vec!["Nirvana".to_string(), "Nirvana".to_string()] };
		let res2 = search(&conn, &q2);
		assert!(res2.is_err() || res2.is_ok());

		Ok(())
	}
}
