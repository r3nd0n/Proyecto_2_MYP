use gtk::prelude::*;
use gtk::{
    Align,
    Application, 
    ApplicationWindow,
    Button,
    GestureClick,
    Box as GtkBox,
    Label,
    Orientation,
    PolicyType,
    ScrolledWindow,
    SearchEntry,
};
use std::rc::Rc;

use crate::view::view::AlbumViewData;
use crate::view::query::query_generator;
use super::mine_route;
use crate::view::styles;

/// Delete all the child widgets from the container.
/// 
/// #Arguments
/// container: the GTK container who all his child widgets
/// will be eliminated at the end of the function.
fn clear_children(container: &GtkBox) {
    while let Some(child) = container.first_child() {
        container.remove(&child);
    }
}

/// Actualiza el album de manera detallada en la interfaz
/// No se crean ventanas nuevas, se reusa target y se relena
/// con la información.
/// #Arguments
/// target: Container GTK donde se van a dibujar los albumes.
/// album: el album que se va a mostrar.
fn render_album_detail(target: &GtkBox, album: &AlbumViewData) {
    clear_children(target);

    let artist = Label::new(Some(&album.artist));
    artist.set_halign(Align::Start);
    artist.add_css_class("album-artist");

    let header = Label::new(Some(&album.name));
    header.set_halign(Align::Start);
    header.add_css_class("album-detail-title");

    let album_meta = format!(
        "Id: {} \n{} canciones",
        album.id_album,
        album.songs
    );

    let meta = Label::new(Some(&album_meta));
    meta.set_halign(Align::Start);

    let path = Label::new(Some(&album.path));
    path.set_halign(Align::Start);
    path.set_wrap(true);

    target.append(&header);
    target.append(&artist);
    target.append(&meta);
    //target.append(&path);

    let songs_title = Label::new(Some("Canciones"));
    songs_title.set_halign(Align::Start);
    songs_title.add_css_class("album-songs-title");
    target.append(&songs_title);

    if album.song_list.is_empty() {
        let empty = Label::new(Some("Este album no tiene canciones registradas."));
        empty.set_halign(Align::Start);
        target.append(&empty);
        return;
    }

    for song in &album.song_list {
        let song_row = GtkBox::new(Orientation::Vertical, 2);
        song_row.add_css_class("song-row");

        let title_text = match song.track {
            Some(track) => format!("{:02}. {}", track, song.title),
            None => song.title.clone(),
        };
        let title = Label::new(Some(&title_text));
        title.set_halign(Align::Start);

        let detail_text = format!(
            "{} \n{}",
            //song.year
            //    .map(|year| year.to_string())
            //    .unwrap_or_else(|| "Sin anio".to_string()),
            song.genre.clone().unwrap_or_else(|| "Sin genero".to_string()),
            album.artist,
            //song.path
        );
        let detail = Label::new(Some(&detail_text));
        detail.set_halign(Align::Start);
        detail.set_wrap(true);

        song_row.append(&title);
        song_row.append(&detail);
        target.append(&song_row);
    }
}

fn render_albums_list(albums_list: &GtkBox, detail_content: &GtkBox, albums: &[AlbumViewData]) {
    clear_children(albums_list);
    clear_children(detail_content);

    if albums.is_empty() {
        let empty = Label::new(Some("No hay albumes minados todavia"));
        empty.set_halign(Align::Start);
        albums_list.append(&empty);

        let empty_detail = Label::new(Some("Selecciona un album para ver su detalle."));
        empty_detail.set_halign(Align::Start);
        detail_content.append(&empty_detail);
        return;
    }

    render_album_detail(detail_content, &albums[0]);

    for album in albums {
        let album_row = GtkBox::new(Orientation::Vertical, 2);
        album_row.add_css_class("album-row");

        let title = Label::new(Some(&album.name));
        title.set_halign(Align::Start);

        let detail_text = format!(
            "{} \n{} canciones",
            album.artist,
            album.songs,
            //album.path
        );
        let detail = Label::new(Some(&detail_text));
        detail.set_halign(Align::Start);
        detail.set_wrap(true);

        album_row.append(&title);
        album_row.append(&detail);

        let detail_panel = detail_content.clone();
        let album_data = album.clone();
        let click = GestureClick::new();
        click.connect_pressed(move |_, _, _, _| {
            render_album_detail(&detail_panel, &album_data);
        });
        album_row.add_controller(click);

        albums_list.append(&album_row);
    }
}


pub fn design_app(
    app: &Application,
    albums: Vec<AlbumViewData>,
    on_mine: Rc<dyn Fn(String) -> Vec<AlbumViewData>>,
    on_search: Rc<dyn Fn(crate::view::query::UsrQuery) -> Vec<AlbumViewData>>,
) {

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Minero")
        .default_width(900)
        .default_height(1200)
        .build();

    // Box principal
    let container = GtkBox::new(Orientation::Vertical, 12);
    container.add_css_class("main-box");

    // Boxes contenedoras de botones

    //
    // TOP
    //
    let container_top = GtkBox::new(Orientation::Horizontal, 12);
    container_top.add_css_class("top-box");
    container_top.set_hexpand(true);

    let container_inner_top = GtkBox::new(Orientation::Horizontal,20);
    container_inner_top.set_hexpand(true);

    //
    // MID
    //
    let container_mid = GtkBox::new(Orientation::Horizontal, 12);
    //container_mid.add_css_class("mid-box");
    
    let container_inner_mid_left= GtkBox::new(Orientation::Vertical,12);
    container_inner_mid_left.set_size_request(350, 80);
    container_inner_mid_left.set_hexpand(false);
    container_inner_mid_left.set_vexpand(true);
    container_inner_mid_left.add_css_class("mid-box-left");

    let spacer_inner_mid_left= GtkBox::new(Orientation::Vertical,12);
    spacer_inner_mid_left.set_size_request(350, 30);

    let albums_list = GtkBox::new(Orientation::Vertical, 8);
    albums_list.set_hexpand(true);
    albums_list.set_vexpand(true);

    let detail_content = GtkBox::new(Orientation::Vertical, 8);
    detail_content.set_hexpand(true);
    detail_content.set_vexpand(true);

    let detail_scroll = ScrolledWindow::new();
    detail_scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
    detail_scroll.set_hexpand(true);
    detail_scroll.set_vexpand(true);
    detail_scroll.set_child(Some(&detail_content));

    render_albums_list(&albums_list, &detail_content, &albums);

    let album_scroll = ScrolledWindow::new();
    album_scroll.set_policy(PolicyType::Never, PolicyType::Automatic);
    album_scroll.set_hexpand(true);
    album_scroll.set_vexpand(true);
    album_scroll.set_child(Some(&albums_list));
    
    let container_inner_mid_right= GtkBox::new(Orientation::Horizontal,12);
    container_inner_mid_right.set_hexpand(true);
    container_inner_mid_right.set_vexpand(true);
    container_inner_mid_right.add_css_class("mid-box-right");
    container_inner_mid_right.append(&detail_scroll);

    //
    // BOTTOM
    //
    let container_bottom = GtkBox::new(Orientation::Horizontal, 12);
    container_bottom.set_size_request(350, 120);
    container_bottom.set_vexpand(false);
    container_bottom.add_css_class("bottom-box");

    // BOTON DE MINADO
    let miner_btn = Button::with_label("Minar");
    let app_for_mine = app.clone();
    let albums_list_for_refresh = albums_list.clone();
    let detail_content_for_refresh = detail_content.clone();
    let on_mine_for_click = on_mine.clone();

    miner_btn.connect_clicked(move |_| {
        let on_mine_for_window = on_mine_for_click.clone();
        let albums_list_for_window = albums_list_for_refresh.clone();
        let detail_content_for_window = detail_content_for_refresh.clone();

        mine_route::open_mine_window(
            &app_for_mine,
            Rc::new(move |route| {
                let refreshed_albums = on_mine_for_window(route);
                render_albums_list(
                    &albums_list_for_window,
                    &detail_content_for_window,
                    &refreshed_albums,
                );
            }),
        );
    });
    miner_btn.add_css_class("button-mine");
    miner_btn.set_halign(Align::End);

    // BARRA DE BUSQUEDA
    let search_box = SearchEntry::new();
    search_box.set_size_request(350,30);
    search_box.set_hexpand(false);
    search_box.set_halign(Align::Fill);
    search_box.set_placeholder_text(Some("Qué quieres escuchar?"));
    search_box.add_css_class("search-box");
    
    let album_list_for_search = albums_list.clone();
    let detail_content_for_search = detail_content.clone();
    let on_search_for_click = on_search.clone();

    search_box.connect_search_changed(move |entry|{
        let search_txt = entry.text().to_string();
        let query = query_generator(&search_txt);
        //TODO 
        //conectar la busqueda que entra el usuario con /model 
        //mediante main.rs.
        let refreshed = on_search_for_click(query);

        render_albums_list(&album_list_for_search, &detail_content_for_search, &refreshed);
    });

    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    // ETIQUETAS
    let label_lib = Label::new(Some("Tus Albums"));
    label_lib.set_halign(Align::Center);

    // CONTAINERS
    container_inner_top.append(&search_box);
    container_inner_top.append(&spacer);
    container_inner_top.append(&miner_btn);
    container_top.append(&container_inner_top);

    spacer_inner_mid_left.append(&label_lib);
    container_inner_mid_left.append(&spacer_inner_mid_left);
    container_inner_mid_left.append(&album_scroll);
    container_mid.append(&container_inner_mid_left);
    container_mid.append(&container_inner_mid_right);
    
    container.append(&container_top);
    container.append(&container_mid);
    container.append(&container_bottom);

    styles::apply_styles();

    window.set_child(Some(&container));
    window.present();
}