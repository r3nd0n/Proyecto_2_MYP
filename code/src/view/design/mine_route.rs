use gtk::prelude::*;
use gtk::{
    AlertDialog,
    Align,
    Application, 
    ApplicationWindow,
    Button,
    Box as GtkBox,
    Entry,
    Label,
    Orientation,
};
use std::rc::Rc;

/// Abre una ventana auxiliar para capturar la ruta a minar.
///
/// La ventana solicita un directorio, valida que no este vacio y, al presionar
/// Minar, ejecuta el callback on_start con la ruta capturada.
/// Si la ruta esta vacia, muestra un AlertDialog y no cierra la ventana.
///
/// # Arguments
/// app: Instancia de la aplicacion GTK.
/// on_start: Callback que recibe la ruta ingresada y dispara el proceso de minado.
pub fn open_mine_window(app: &Application, on_start: Rc<dyn Fn(String)>) {

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Ventana de minado")
        .default_width(700)
        .build();

    let container = GtkBox::new(Orientation::Vertical,12);
    container.add_css_class("main-box");

    let container_top = GtkBox::new(Orientation::Horizontal, 12);
    container_top.add_css_class("top-box");
    container_top.set_hexpand(true);

    let description = Label::new(Some(
        "Ingresa la ruta del directorio:"
    ));
    description.set_wrap(true);
    description.set_hexpand(true);
    description.set_halign(Align::Center);
    description.add_css_class("mine-description");

    let route_input = Entry::new();
    route_input.set_size_request(350,30);
    route_input.set_hexpand(true);
    route_input.set_halign(Align::Fill);
    route_input.add_css_class("search-box");
    route_input.set_placeholder_text(Some("/ruta/a/tu/musica"));

    let start_btn = Button::with_label("Minar");
    start_btn.set_halign(Align::End);

    let input_row = GtkBox::new(Orientation::Horizontal, 12);
    input_row.set_hexpand(true);

    let route_input_for_start = route_input.clone();
    let on_start_for_start = on_start.clone();
    let window_for_start = window.clone();
    start_btn.connect_clicked(move |_| {
        let route = route_input_for_start.text().trim().to_string();

        if route.is_empty() {
            let alert = AlertDialog::builder()
                .modal(true)
                .message("Debes especificar la ruta para minar.")
                .build();
            alert.show(Some(&window_for_start));
            return;
        }

        on_start_for_start(route);
        window_for_start.close();
    });

    container_top.append(&description);
    input_row.append(&route_input);
    input_row.append(&start_btn);

    container.append(&container_top);
    container.append(&input_row);

    window.set_child(Some(&container));
    window.present();
}