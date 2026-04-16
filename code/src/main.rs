//use gtk::prelude::*;
//use gtk::Application;

use rusqlite::Connection;

mod view;
mod model;

fn main() {
    let read_from = "/home/ahalgana".to_string();

    let conn = Connection::open("music.db").expect("No se pudo abrir music.db");
    model::db::create_db(&conn).expect("No se pudo inicializar el esquema");

    let total = model::minero::miner(&conn, &read_from);
    println!("Canciones insertadas/actualizadas: {total}");

    view::view::show_view();
}