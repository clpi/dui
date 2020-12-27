use gettextrs::*;
use gtk::prelude::*;
use crossbeam_channel::unbounded;
use gio::{prelude::*, ApplicationExt};
use crate::{
    models, state,
};

pub struct App {
    pub app: gtk::Application,
    pub builder: gtk::Builder,
    pub main_window: gtk::ApplicationWindow,
    pub state: state::State,
    pub config: state::AppConfig,

}

impl App {
    pub fn new() -> Self {
        let config = state::AppConfig::default();
        let state = state::State::default();
        Self::startup()?;
        let builder = gtk::Builder::from_file("/home/chrisp/div/ui/gtk/divis/res/ui/main.ui");
        let main_window: gtk::ApplicationWindow = builder.get_object("window");
        let app = gtk::Application::new(Some(config::APP_ID), gio::ApplicationFlags::empty())?;

        Self {
            app, config, state, builder, main_window,
        }
    }

    pub fn startup() -> Result<(), Box<dyn std::error::Error>> {
        gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
        glib::set_program_name(Some("div.is"));
        glib::set_application_name("div.is".into());
        gst::init()?;

        setlocale(LocaleCategory::LcAll, "");
        bindtextdomain("pl", config::LOCALEDIR);
        textdomain("pl");

        let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/di.gresource")
            .expect("Could not load resources");
        gio::resources_register(&res);
        Ok(())
    }

    pub fn load_ui(self) -> Result<(), Box<dyn std::error::Error>> {
        self.builder.add_from_file("../res/ui/about.ui")
            .expect("Could not load UI resource from file");
        self.builder.add_from_file("../res/ui/preferences.ui")
            .expect("Could not load UI preferences resource from file");
        Ok(())
    }


    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        self.app.connect_activate(move |app| {
            let window = app.run(&[]);
            let (mut req_recv, req_updt) = unbounded::<State>();
            let (res_send, res_recv) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
            let icon_theme = gtk::IconTheme::get_default().expect("Could not load icon theme");

            window.widget.set_application(Some(app));
            app.add_window(&window.widget);
            window.widget.present();
        });

        let ret = app.run(&std::env::args().collect::<Vec<_>>());
        std::process::exit(ret);
        Ok(())

    }


}

