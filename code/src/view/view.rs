use gtk::prelude::*;
use gtk::Application;
use std::rc::Rc;

use super::design;

#[derive(Clone, Debug)]
pub struct SongViewData {
    pub title: String,
    //pub path: String,
    pub track: Option<i64>,
    //pub year: Option<i64>,
    pub genre: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AlbumViewData {
    pub id_album: i64,
    pub artist: String,
    pub name: String,
    pub path: String,
    //pub year: Option<i64>,
    pub songs: i64,
    pub song_list: Vec<SongViewData>,
}

pub fn show_view<F>(albums: Vec<AlbumViewData>, on_mine: F)
where
    F: Fn(String) -> Vec<AlbumViewData> + 'static,
{
    let on_mine = Rc::new(on_mine) as Rc<dyn Fn(String) -> Vec<AlbumViewData>>;

    let app = Application::builder()
        .application_id("com.modelado.proyecto2")
        .build();

    let on_mine_for_activate = on_mine.clone();
    app.connect_activate(move |app| {
        design::design_app(app, albums.clone(), on_mine_for_activate.clone());
    });

    app.run();
}

