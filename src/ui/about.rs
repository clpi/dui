use std::convert::TryFrom;
use gtk::prelude::*;

#[derive(Clone, Debug)]
pub struct About {
    pub root: gtk::AboutDialog,
    close: gtk::Button,
}

impl TryFrom<gtk::Builder> for About {

    type Error = Box<dyn std::error::Error>;

    fn try_from(builder: gtk::Builder) -> Result<Self, Self::Error> {
        builder.add_from_file("../res/ui/about.ui")?;
        let root: gtk::AboutDialog = builder.get_object("about_dialog").expect("Could not get about dialog");
        let close: gtk::Button = builder.get_object("close_btn").expect("Could not get close button");
        Ok(Self { root, close })
    }
}
