use gtk::{prelude::*, Application};

mod bar;
mod css;
mod utils;

const APP_ID: &str = "mgw.developermikey.github";

#[tokio::main]
async fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        bar::bar(app.clone()).present();

        css::load_css();
    });

    app.run();
}

