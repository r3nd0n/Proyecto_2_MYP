use gtk::prelude::*;
use gtk::Application;

use super::design;

pub fn show_view() {
    let app = Application::builder()
        .application_id("com.modelado.proyecto2")
        .build();

    app.connect_activate(design::design_app);

    app.run();
}

