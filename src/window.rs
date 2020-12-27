use gtk::prelude::*;
use crate::{
     models::user::User, state::AppConfig,
};

pub struct Win {
    config: AppConfig,
    pub widget: gtk::ApplicationWindow,
}

impl Default for Win {
    fn default() -> Self {
        let config = AppConfig::default();
        let builder = gtk::Builder::from_file("/home/chrisp/div/ui/gtk/divis/res/ui/main.ui");
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
