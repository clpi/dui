use crate::models::User;
use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    pub user: Option<User>,
    pub config: Config,
}

impl Window {
    pub fn new() -> Self {
        let brn = gtk::Button::new_with_label("Click");
        let builder = gtk::Builder::new_from_resource("/li/per/pl/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        Self { widget }
    }
}
