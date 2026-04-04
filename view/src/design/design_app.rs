use gtk::prelude::*;
use gtk::{
    Align,
    Application, 
    ApplicationWindow,
    Button,
    Box as GtkBox,
    Label,
    Orientation,
    SearchEntry,
};

use crate::styles;


pub fn design_app(app: &Application) {

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
    
    let container_inner_mid_right= GtkBox::new(Orientation::Horizontal,12);
    container_inner_mid_right.set_hexpand(true);
    container_inner_mid_right.add_css_class("mid-box-left");

    //
    // BOTTOM
    //
    let container_bottom = GtkBox::new(Orientation::Horizontal, 12);
    container_bottom.set_size_request(350, 120);
    container_bottom.set_vexpand(false);
    container_bottom.add_css_class("bottom-box");

    // BUTTONS
    let miner_btn = Button::with_label("Minar");
    miner_btn.connect_clicked(|_| {
        // Aquí se debe abrir la otra ventana de minado.
    });
    miner_btn.add_css_class("button-mine");
    miner_btn.set_halign(Align::End);

    let search_box = SearchEntry::new();
    search_box.set_size_request(350,30);
    search_box.set_hexpand(false);
    search_box.set_halign(Align::Fill);
    search_box.add_css_class("search-box");

    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);

    // ETIQUETAS
    let label_lib = Label::new(Some("Tu Biblioteca"));
    label_lib.set_halign(Align::Center);

    // CONTAINERS
    container_inner_top.append(&search_box);
    container_inner_top.append(&spacer);
    container_inner_top.append(&miner_btn);
    container_top.append(&container_inner_top);

    spacer_inner_mid_left.append(&label_lib);
    container_inner_mid_left.append(&spacer_inner_mid_left);
    container_mid.append(&container_inner_mid_left);
    container_mid.append(&container_inner_mid_right);
    
    container.append(&container_top);
    container.append(&container_mid);
    container.append(&container_bottom);

    styles::apply_styles();

    window.set_child(Some(&container));
    window.present();
}