use walkdir::WalkDir;
use rusqlite::Connection;
use super::db::upsert_song;
use super::id3reader::id3_reader;

/// Recorre recursivamente una ruta en disco, lee etiquetas ID3 de archivos MP3
/// e inserta/actualiza su informacion en la base de datos.
///
/// Devuelve la cantidad de canciones que se pudieron guardar correctamente.
pub fn miner(conn: &Connection, route: &str) -> usize {
    let mut inserted_or_updated = 0;

    for entry in WalkDir::new(route).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file()
            && path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("mp3"))
        {
            println!("{}", path.display());
            if let Some(song) = id3_reader(path) {
                match upsert_song(conn, &song) {
                    Ok(()) => {
                        inserted_or_updated += 1;
                    }
                    Err(err) => {
                        eprintln!("No se pudo guardar {}: {err}", song.file_path);
                    }
                }
            }
        }
    }

    inserted_or_updated
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::db;
    use id3::{Tag, TagLike, Version};
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("El reloj del sistema retrocedio")
            .as_nanos();
        std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), unique))
    }

    #[test]
    fn miner_imports_mp3_with_valid_id3_tag() -> rusqlite::Result<()> {
        let conn = Connection::open_in_memory()?;
        db::create_db(&conn)?;

        let base_dir = temp_dir("miner_test");
        fs::create_dir_all(&base_dir).expect("No se pudo crear directorio temporal");
        let file_path = base_dir.join("track01.mp3");
        fs::write(&file_path, []).expect("No se pudo crear archivo mp3 temporal");

        let mut tag = Tag::new();
        tag.set_title("Around the World");
        tag.set_artist("Red Hot Chili Peppers");
        tag.set_album("Californication");
        tag.set_genre("Rock");
        tag.set_track(1);
        tag.set_year(1999);
        tag.write_to_path(&file_path, Version::Id3v24)
            .expect("No se pudo escribir el tag ID3 de prueba");

        let imported = miner(&conn, base_dir.to_str().expect("Ruta UTF-8 valida"));
        assert_eq!(imported, 1);

        let rows: i64 = conn.query_row("SELECT COUNT(*) FROM rolas", [], |row| row.get(0))?;
        assert_eq!(rows, 1);

        let _ = fs::remove_file(&file_path);
        let _ = fs::remove_dir_all(&base_dir);

        Ok(())
    }
}
