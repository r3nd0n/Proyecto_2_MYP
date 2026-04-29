//use gtk::prelude::*;
//use gtk::Application;

use std::env;
use std::fs;
use std::path::Path;

use rusqlite::Connection;

mod view;
mod model;

fn map_albums_to_view(albums: Vec<model::db::AlbumWithSongs>) -> Vec<view::view::AlbumViewData> {
    albums
        .into_iter()
        .map(|album| view::view::AlbumViewData {
            id_album: album.id_album,
            artist: album.artist,
            name: album.name,
            path: album.path,
            //year: album.year,
            songs: album.songs,
            song_list: album
                .song_list
                .into_iter()
                .map(|song| view::view::SongViewData {
                    title: song.title,
                    //path: song.path,
                    track: song.track,
                    //year: song.year,
                    genre: song.genre,
                })
                .collect(),
        })
        .collect()
}

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
    });
}