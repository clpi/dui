use gtk::prelude::*;

pub struct About {
    root: gtk::AboutDialog,
}

impl From<gtk::Builder> for About {
    fn from(builder: gtk::Builder) -> About {
        builder.add_from_file("../res/ui/about.ui")
            .expect("Could not load about dialog UI file");
       let root = builder.get_object::<gtk::AboutDialog>("about_dialog")
            .expect("Did not find about dialog in UI schema");
        Self { root }
    }
}
