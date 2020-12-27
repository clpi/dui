use gtk::prelude::*;

pub struct Preferences {
    root: gtk::Window,
}

impl From<gtk::Builder> for About {
    fn from(builder: gtk::Builder) -> About {
        builder.add_from_file("../res/ui/about.ui")
            .expect("Could not load preferences UI file");
       let root = builder.get_object::<gtk::Window>("preferences")
            .expect("Did not find preferences in UI schema");
        Self { root }
    }
}
