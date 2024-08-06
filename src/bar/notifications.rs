use gtk::prelude::*;
use gtk::Orientation;

pub fn bar_notifications() -> gtk::Box {
    let hbox = gtk::Box::new(Orientation::Horizontal, 0);
    hbox.add_css_class("bar-clock");
    hbox.add_css_class("bar-box");

    let label = gtk::Label::new(Some("󰂚"));

    label.add_css_class("bar-clock-label");
    label.add_css_class("bar-text");
    hbox.append(&label);

    hbox
}
