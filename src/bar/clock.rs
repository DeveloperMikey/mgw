use gio::glib;
use gtk::prelude::*;
use gtk::Orientation;

pub fn bar_clock() -> gtk::Box {
    let hbox = gtk::Box::new(Orientation::Horizontal, 0);
    hbox.add_css_class("bar-clock");
    hbox.add_css_class("bar-box");

    let label = gtk::Label::new(None);

    label.add_css_class("bar-clock-label");
    label.add_css_class("bar-text");
    hbox.append(&label);

    let update_time_label = move || {
        let format = "%H:%M";
        let now = glib::DateTime::now_local();
        let time = now.unwrap().format(format).unwrap();

        label.set_label(time.as_ref());

        glib::ControlFlow::Continue
    };

    update_time_label();
    glib::timeout_add_seconds_local(1, update_time_label);

    hbox
}
