use gtk::prelude::*;
use gtk::Application;

mod design;
mod styles;

fn main() {
    let app = Application::builder()
        .application_id("Proyect 2.Modelado.y.Programacion")
        .build();

    app.connect_activate(design::design_app);

    app.run();
}

