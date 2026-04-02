use gtk::prelude::*;
use gtk::{
    Application, 
    ApplicationWindow,
    Button,
    Box as GtkBox,
    Orientation,
    SearchEntry,
};

fn main() {
    let app = Application::builder()
     .application_id("Proyect 2.Modelado.y.Programacion")
     .build();

    app.connect_activate(app_logic);

    app.run();
}

pub fn app_logic(app: &Application) {

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Minero")
        .default_width(900)
        .default_height(1200)
        .build();

    // Box principal
    let container = GtkBox::new(Orientation::Vertical, 12);
    container.set_margin_top(10);
    container.set_margin_bottom(10);
    container.set_margin_start(10);
    container.set_margin_end(10);

    // Boxes contenedoras de botones
    let container_top = GtkBox::new(Orientation::Horizontal, 12);
    container_top.set_margin_top(10);
    container_top.set_margin_bottom(10);
    container_top.set_margin_start(10);
    container_top.set_margin_end(10);

    let container_inner_top = GtkBox::new(Orientation::Horizontal,50);

    let container_mid = GtkBox::new(Orientation::Horizontal, 12);
    container_mid.set_margin_top(10);
    container_mid.set_margin_bottom(10);
    container_mid.set_margin_start(10);
    container_mid.set_margin_end(10);

    let container_bottom = GtkBox::new(Orientation::Horizontal, 12);
    container_bottom.set_margin_top(10);
    container_bottom.set_margin_bottom(10);
    container_bottom.set_margin_start(10);
    container_bottom.set_margin_end(10);

    let miner_btn = Button::with_label("Minar");
    miner_btn.connect_clicked(|_| {
        // Aquí se debe abrir la otra ventana de minado.
    });

    let search_box = SearchEntry::new();

    container_inner_top.append(&search_box);
    container_inner_top.append(&miner_btn);
    container_top.append(&container_inner_top);
    
    container.append(&container_top);
    container.append(&container_mid);
    container.append(&container_bottom);

    window.set_child(Some(&container));
    window.present();
}
