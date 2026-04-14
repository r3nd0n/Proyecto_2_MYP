use id3::{Tag, TagLike};
use std::path::Path;

fn normalize_text(value: Option<&str>) -> String {
    value
        .filter(|text| text.chars().any(|ch| !ch.is_whitespace() && !ch.is_control()))
        .map(|text| text.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn id3_reader(path: &Path) {
    match Tag::read_from_path(path) {
        Ok(tag) => {
            let title = normalize_text(tag.title());
            let artist = normalize_text(tag.artist());
            let album = normalize_text(tag.album());
            let year = tag
                .year()
                .map(|value| value.to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            println!("Titulo: {}", title);
            println!("Artista: {}", artist);
            println!("Album: {}", album);
            println!("Año: {}\n", year);
        }
        Err(err) => {
            println!("No se pudó leer de la etiqueta ID3: {err}");
        }
    }
}