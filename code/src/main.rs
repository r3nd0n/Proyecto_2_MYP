//use gtk::prelude::*;
//use gtk::Application;

mod view;
mod model;

fn main() {
    let read_from = "/home/ahalgana".to_string();

    view::view::show_view();

    model::minero::miner(read_from);
}