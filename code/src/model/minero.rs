use walkdir::WalkDir;
use super::id3reader::id3_reader;

pub fn miner(route: String) {
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
            id3_reader(path);
        }
    }
}
