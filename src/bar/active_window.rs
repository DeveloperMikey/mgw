use async_channel::bounded;
use gio::glib;
use gio::glib::clone;
use gtk::prelude::*;
use gtk::Orientation;
use hyprland::data::Client;
use hyprland::event_listener::AsyncEventListener;
use hyprland::shared::HyprDataActiveOptional;
use hyprland::Result;
use std::sync::{Arc, Mutex};

use crate::utils::runtime;

fn hook_name(name: String) -> String {
    match name.to_lowercase().as_str() {
        "kitty" => "󰄛 Terminal",
        "firefox" => "󰈹 Firefox",
        "webcord" => " Discord",
        "discord" => " Discord",
        "vesktop" => " Discord",
        "steam" => " Steam",
        _ => name.as_str()
    }.to_string()
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
    hook_name(name)
}

pub fn active_client_box() -> gtk::Box {
    let hbox = gtk::Box::new(Orientation::Horizontal, 0);
    hbox.add_css_class("active-client");
    hbox.add_css_class("bar-box");

    let active_app_label = gtk::Label::new(None);

    active_app_label.set_label(&get_active_client());

    active_app_label.add_css_class("active-client-label");
    active_app_label.add_css_class("bar-text");
    hbox.append(&active_app_label);
    let (sender, receiver) = bounded::<String>(1);
    let runtime = runtime();

    let sender = Arc::new(Mutex::new(sender));
    let sender_clone = Arc::clone(&sender);

    runtime.spawn(async move {
        let mut event_listener = AsyncEventListener::new();

        event_listener.add_workspace_change_handler({
            let sender_clone = Arc::clone(&sender_clone);
            move |data| {
                let sender_clone = Arc::clone(&sender_clone);
                Box::pin(async move {
                    if let Ok(sender) = sender_clone.lock() {
                        sender
                            .send_blocking(get_active_client())
                            .expect("The channel needs to be open.");
                    }
                })
            }
        });

        event_listener.add_window_close_handler({
            let sender_clone = Arc::clone(&sender_clone);
            move |data| {
                let sender_clone = Arc::clone(&sender_clone);
                Box::pin(async move {
                    if let Ok(sender) = sender_clone.lock() {
                        sender
                            .send_blocking(get_active_client())
                            .expect("The channel needs to be open.");
                    }
                })
            }
        });

        event_listener.add_active_window_change_handler({
            let sender_clone = Arc::clone(&sender_clone);
            move |data| {
                let sender_clone = Arc::clone(&sender_clone);
                Box::pin(async move {
                    let title = if let Some(data) = data {
                        data.window_class
                    } else {
                        String::from("How?")
                    };

                    if let Ok(sender) = sender_clone.lock() {
                        sender
                            .send_blocking(hook_name(title))
                            .expect("The channel needs to be open.");
                    }
                })
            }
        });

        let _ = event_listener.start_listener_async().await;
    });

    glib::spawn_future_local(clone!(
        #[weak]
        active_app_label,
        async move {
            while let Ok(title) = receiver.recv().await {
                active_app_label.set_label(&title);
            }
        }
    ));

    hbox
}
