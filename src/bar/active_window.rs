use gio::glib::MainContext;
use gtk::prelude::*;
use gtk::Box;
use gtk::Label;
use gtk::Orientation;
use hyprland::data::Client;
use hyprland::event_listener::EventListener;
use hyprland::shared::HyprDataActiveOptional;

fn hook_client_name(name: String) -> String {
    match name.to_lowercase().as_str() {
        "kitty" => "󰄛 Terminal",
        "firefox" => "󰈹 Firefox",
        "webcord" => "  Discord",
        "discord" => "  Discord",
        "vesktop" => "  Discord",
        "steam" => " Steam",
        _ => name.as_str(),
    }
    .to_string()
}

fn get_active_client() -> String {
    let name = match Client::get_active().ok().flatten() {
        Some(client) => client
            .class
            .split('\0')
            .next()
            .unwrap_or(&client.class)
            .to_string(),
        None => "󰇄 Desktop".to_string(),
    };

    hook_client_name(name)
}

pub fn active_client_box() -> Box {
    let hbox = Box::new(Orientation::Horizontal, 0);
    hbox.add_css_class("active-client");
    hbox.add_css_class("bar-box");

    let active_app_label = Label::new(Some(&get_active_client()));
    active_app_label.add_css_class("active-client-label");
    active_app_label.add_css_class("bar-text");

    hbox.append(&active_app_label.clone());

    let active_app_label1 = active_app_label.clone();
    let active_app_label2 = active_app_label.clone();
    let active_app_label3 = active_app_label.clone();
    MainContext::default().spawn_local(async move {
        let mut event_listener = EventListener::new();

        event_listener.add_active_window_change_handler(move |data| {
            let app = data.unwrap().window_class;
            active_app_label1.set_label(&hook_client_name(app));
        });

        event_listener.add_workspace_change_handler(move |_| {
            active_app_label2.set_label(&get_active_client());
        });

        event_listener.add_window_close_handler(move |_| {
            active_app_label3.set_label(&get_active_client());
        });

        event_listener.start_listener_async().await.unwrap();
    });

    hbox
}
