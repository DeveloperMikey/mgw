use active_window::active_client_box;
use clock::bar_clock;
use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell};
use notifications::bar_notifications;

mod active_window;
mod clock;
mod notifications;

fn start() -> gtk::Box {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox
}

fn center() -> gtk::Box {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.append(&active_client_box());
    hbox
}

fn end() -> gtk::Box {
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.append(&bar_clock());
    hbox.append(&bar_notifications());
    hbox
}

pub fn bar(app: gtk::Application) -> gtk::ApplicationWindow {
    let window = gtk::ApplicationWindow::builder().application(&app).build();
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_namespace("bar");

    let widgets = gtk::CenterBox::new();
    widgets.set_start_widget(Some(&start()));
    widgets.set_center_widget(Some(&center()));
    widgets.set_end_widget(Some(&end()));

    window.set_child(Some(&widgets));

    window.set_css_classes(&["bar"]);

    window.auto_exclusive_zone_enable();

    window.set_anchor(Edge::Bottom, true);
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Right, true);

    window
}

