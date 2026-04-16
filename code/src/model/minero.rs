use walkdir::WalkDir;
use rusqlite::Connection;
use super::db::upsert_song;
use super::id3reader::id3_reader;

pub fn miner(conn: &Connection, route: &str) -> usize {
    let mut inserted_or_updated = 0;

    // La ruta de mi computadora /home/ahalgana
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
