#[derive(Clone, Debug)]
pub struct usr_query {
    pub artists: Vec<String>,
    pub albums: Vec<String>,
    pub songs: Vec<String>,
}

/// Genera una estructura usr_query a partir de un string.
/// Busca prefijos como "artista:", "album:" y "cancion:".
/// Los bloques se separan con "&" o ",".
///
/// #Arguments
/// search: string que se usara para crear la estructura
/// usr_query.
pub fn query_generator(search: &str) -> usr_query {
    let mut artists = Vec::new();
    let mut albums = Vec::new();
    let mut songs = Vec::new();

    for chunk in search.split(['&', ',']) {
        let value = chunk.trim();
        if value.is_empty() {
            continue;
        }

        let lower = value.to_lowercase();
        if lower.starts_with("artista:") {
            let original = value["artista:".len()..].trim();
            if !original.is_empty() {
                artists.push(original.to_string());
            }
        } else if lower.starts_with("album:") {
            let original = value["album:".len()..].trim();
            if !original.is_empty() {
                albums.push(original.to_string());
            }
        } else if lower.starts_with("cancion:") {
            let original = value["cancion:".len()..].trim();
            if !original.is_empty() {
                songs.push(original.to_string());
            }
        } else if lower.starts_with("canción:") {
            let original = value["canción:".len()..].trim();
            if !original.is_empty() {
                songs.push(original.to_string());
            }
        }
    }

    usr_query {
        artists,
        albums,
        songs,
    }
}


//SELECT r.id_rola, r.title, r.path, r.track, r.year, r.genre
//FROM rolas r
//JOIN performers p ON p.id_performer = r.id_performer
//WHERE p.name = ?1 COLLATE NOCASE
//ORDER BY r.track IS NULL, r.track ASC, r.title COLLATE NOCASE ASC;
//
//SELECT DISTINCT a.id_album, a.name, a.path, a.year
//FROM albums a
//JOIN rolas r ON r.id_album = a.id_album
//JOIN performers p ON p.id_performer = r.id_performer
//WHERE p.name = ?1 COLLATE NOCASE
//ORDER BY a.name COLLATE NOCASE ASC;