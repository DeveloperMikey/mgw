use gio::glib::MainContext;
use gtk::{prelude::*, Application};
use hyprland::{async_closure, event_listener::AsyncEventListener};

mod bar;
mod css;
mod utils;

const APP_ID: &str = "mgw.developermikey.github";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        bar::bar(app.clone()).present();

        css::load_css();
    });

    app.run();
}

