use gtk::prelude::*;
use crate::{
    app_config::AppConfig, models::user::User,
};

pub struct Win {
    config: AppConfig,
    pub widget: gtk::ApplicationWindow,
}

impl Default for Win {
    fn default() -> Self {
        let config = AppConfig::default();
        let builder = gtk::Builder::from_file("/home/chrisp/per/gtp/pl/src/window.ui");
        let widget: gtk::ApplicationWindow = builder.get_object("window")
            .expect("Failed to find the window object");
        Self { config, widget }
    }
}

impl Win {
    pub fn new() -> Self {
        Self::default()
    }
}
