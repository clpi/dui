use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;
use crossbeam_channel::unbounded;

pub mod config;
pub mod window;
pub mod models;
pub mod app;
pub mod ui;
pub mod state;
pub mod util;

use crate::{
    window::Win,
    state::State,
};

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    glib::set_program_name(Some("per.li"));
    glib::set_application_name("per.li".into());

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("pl", config::LOCALEDIR);
    textdomain("pl");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/pl.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("li.per.pl"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = app.run(&["per".into()]);
        let (mut req_recv, req_updt) = unbounded::<State>();
        let (res_send, res_recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        std::thread::spawn(move || {
            loop {
                let req = req_recv.latest_mut();
                if let Some(state) = req.take() {
                    res_send.send("Hello!".into()).expect("No open GLIB channel");
                }
            }
            std::thread::sleep_ms(100);
        });
        let icon_theme = gtk::IconTheme::get_default().expect("Could not load icon theme");

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
