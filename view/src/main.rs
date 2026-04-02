use gtk::prelude::*;
use gtk::{
    Application, 
    ApplicationWindow,
};

fn main() {
    let app = Application::builder()
     .application_id("Proyect 2.Modelado.y.Programacion")
     .build();

    app.connect_activate(app_logic);

    app.run;
}

pub fn app_logic() {

}
