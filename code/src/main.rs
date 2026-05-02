use std::env;
use std::fs;
use std::path::Path;

use rusqlite::Connection;

mod view;
mod model;

/// Convierte el modelo de dominio (AlbumWithSongs) al modelo de presentacion
/// usado por la capa de vista.
///
/// Mantiene los datos principales del album y transforma cada cancion al tipo
/// SongViewData para que la UI pueda renderizarla sin depender del modelo DB.
fn map_albums_to_view(albums: Vec<model::db::AlbumWithSongs>) -> Vec<view::view::AlbumViewData> {
    albums
        .into_iter()
        .map(|album| view::view::AlbumViewData {
            id_album: album.id_album,
            artist: album.artist,
            name: album.name,
            path: album.path,
            songs: album.songs,
            song_list: album
                .song_list
                .into_iter()
                .map(|song| view::view::SongViewData {
                    title: song.title,
                    track: song.track,
                    genre: song.genre,
                })
                .collect(),
        })
        .collect()
}

/// Punto de entrada de la aplicacion.
///
/// Flujo principal:
/// - Resuelve la ruta de la base de datos (variable DB_PATH o valor por defecto).
/// - Asegura el directorio y esquema SQLite.
/// - Carga los albumes iniciales para la UI.
/// - Configura callbacks de busqueda y minado.
/// - Lanza la vista GTK con show_view.
fn main() {
    let db_path = env::var("DB_PATH").unwrap_or_else(|_| "data/music.db".to_string());

    if let Some(parent) = Path::new(&db_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).expect("No se pudo crear el directorio de la base de datos");
        }
    }

    let conn = Connection::open(&db_path).expect("No se pudo abrir la base de datos");
    model::db::create_db(&conn).expect("No se pudo inicializar el esquema");

    let albums = model::db::get_albums_with_songs(&conn).expect("No se pudieron cargar los albumes");
    let albums_view = map_albums_to_view(albums);

    let db_path_for_mine = db_path.clone();
    let db_path_for_search = db_path.clone();

    let on_search = move |query: view::query::UsrQuery| {
        let conn = Connection::open(&db_path_for_search)
            .expect("No se pudo abrir la base de datos para buscar");
        model::db::create_db(&conn)
            .expect("No se pudo asegurar el esquema antes de la busqueda");

        let matched_albums = model::interpreter::search_from_raw(&conn, &query.raw)
            .expect("No se pudo ejecutar la busqueda");
        map_albums_to_view(matched_albums)
    };
    
    view::view::show_view(albums_view, move |route| {
        let conn = Connection::open(&db_path_for_mine)
            .expect("No se pudo abrir la base de datos para minar");
        model::db::create_db(&conn)
            .expect("No se pudo asegurar el esquema antes del minado");

        let total = model::minero::miner(&conn, &route);
        println!("Canciones insertadas/actualizadas desde '{route}': {total}");

        let refreshed_albums = model::db::get_albums_with_songs(&conn)
            .expect("No se pudieron recargar los albumes despues del minado");
        map_albums_to_view(refreshed_albums)
    }, on_search);
}