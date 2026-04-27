use gtk::{
    CssProvider,
    gdk,
};

pub fn apply_styles() {
    let provider = CssProvider::new();

    provider.load_from_string(
        "
        .main-box { background: #000000; border-radius: 8px; padding: 10px; }\
        .top-box { background: #77E34F; border-radius: 8px; padding: 10px; }\
        .mid-box-right { background: #141414; border-radius: 8px; padding: 10px; }\
        .mid-box-left { background: #141414; border-radius: 8px; padding: 10px; }\
        .mid-box-left-cubes { background: #292929; border-radius: 8px; padding: 10px; }\
        .search-box { background: #141414; border-radius: 20px; color: #C4C4C4; caret-color: #C4C4C4; -gtk-secondary-caret-color: #C4C4C4; }\
        .search-box image { color: #C4C4C4; }\
        .search-box image.left { color: #C4C4C4; }\
        .button-mine { background: #141414; color: #C4C4C4; }\
        .button-mine:hover { background: #A19F9F; color: #141414; }\
        .button-mine label { color: #C4C4C4; }\
        .button-mine:hover label { color: #141414; }\
        .mine-description { color: #000000; }\
        .bottom-box { background: #141414; border-radius: 8px; padding: 10px; }"
    );

    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("No display available"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}