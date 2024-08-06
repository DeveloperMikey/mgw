use notify::{RecursiveMode, Result, Watcher};
use std::path::Path;

use crate::utils::exec;

pub fn watch_css() -> Result<()> {
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => println!("event: {:?}", event),
        Err(e) => println!("watch error: {:?}", e),
    })?;

    watcher.watch(Path::new("/home/mike/Projects/mgw/css"), RecursiveMode::Recursive)?;

    Ok(())
}

pub fn load_css() {
    let provider = gtk::CssProvider::new();
    let output_css = "/tmp/mgw/";
    let output_file_css = "/tmp/mgw/style.css";

    if !std::path::Path::new(output_css).exists() {
        std::fs::create_dir(output_css).unwrap();
    }

    let scss = exec("sassc", vec!["./css/style.scss", output_file_css]);
    if scss.contains("Warning") {
        println!("{}", scss);
    }

    let css = std::fs::read_to_string(output_file_css).expect("Failed to read CSS file");
    provider.load_from_string(&css);

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    _ = watch_css();
}
