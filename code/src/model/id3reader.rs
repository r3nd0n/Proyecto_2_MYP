use id3::{Tag, TagLike};
use std::path::Path;
use super::songtag::SongTag;

/// Normaliza un valor de texto leido desde una etiqueta ID3.
///
/// Si el valor es None, devuelve Unknown.
/// Si el texto solo contiene espacios o caracteres de control, tambien devuelve Unknown.
fn normalize_text(value: Option<&str>) -> String {
    value
        .filter(|text| text.chars().any(|ch| !ch.is_whitespace() && !ch.is_control()))
        .map(|text| text.trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

/// Lee la informacion ID3 de un archivo y la convierte en SongTag.
///
/// Extrae titulo, artista, album, genero, pista y año desde la etiqueta del archivo.
/// Si la lectura falla, devuelve None y escribe un mensaje en consola.
pub fn id3_reader(path: &Path) -> Option<SongTag> {
    match Tag::read_from_path(path) {
        Ok(tag) => {
            let title = normalize_text(tag.title());
            let artist = normalize_text(tag.artist());
            let album = normalize_text(tag.album());
            let genre = normalize_text(tag.genre());
            let track = tag.track().map(|value| value as i32);
            let year = tag.year();

            Some(SongTag {
                file_path: path.display().to_string(),
                title,
                artist,
                album,
                genre,
                track,
                year,
            }) 
        }
        Err(err) => {
            println!("No se pudo leer la etiqueta ID3: {err}");
            None
        }
    }
}