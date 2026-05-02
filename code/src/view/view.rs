use gtk::prelude::*;
use gtk::Application;
use std::rc::Rc;

use super::design;

#[derive(Clone, Debug)]
pub struct SongViewData {
    pub title: String,
    pub track: Option<i64>,
    pub genre: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AlbumViewData {
    pub id_album: i64,
    pub artist: String,
    pub name: String,
    pub path: String,
    pub songs: i64,
    pub song_list: Vec<SongViewData>,
}

/// Inicia la aplicacion GTK y muestra la vista principal.
///
/// Recibe los albumes iniciales y dos callbacks:
/// on_mine: ejecuta el proceso de minado y devuelve albumes actualizados.
/// on_search: ejecuta una busqueda y devuelve los resultados para renderizar.
///
/// Los callbacks se envuelven en Rc para poder clonarlos y usarlos dentro del
/// closure de activacion de GTK.
///
/// # Arguments
/// albums: Datos iniciales de albumes a mostrar en la UI.
/// on_mine: Funcion para refrescar datos despues del minado.
/// on_search: Funcion para resolver busquedas desde la UI.
pub fn show_view<F, S>(albums: Vec<AlbumViewData>, on_mine: F, on_search: S)
where
    F: Fn(String) -> Vec<AlbumViewData> + 'static,
    S: Fn(crate::view::query::UsrQuery) -> Vec<AlbumViewData> + 'static,
{
    let on_mine = Rc::new(on_mine) as Rc<dyn Fn(String) -> Vec<AlbumViewData>>;
    let on_search = Rc::new(on_search) as Rc<dyn Fn(crate::view::query::UsrQuery) -> Vec<AlbumViewData>>;

    let app = Application::builder()
        .application_id("com.modelado.proyecto2")
        .build();

    let on_mine_for_activate = on_mine.clone();
    let on_search_for_activate = on_search.clone();

    app.connect_activate(move |app| {
        design::design_app(app, albums.clone(), on_mine_for_activate.clone(), on_search_for_activate.clone());
    });

    app.run();
}

