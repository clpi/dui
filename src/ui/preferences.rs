use std::convert::TryFrom;
use gtk::prelude::*;

#[derive(Clone, Debug)]
pub struct Preferences {
    pub root: gtk::Window,
}

impl TryFrom<gtk::Builder> for Preferences {

    type Error = Box<dyn std::error::Error>;

    fn try_from(builder: gtk::Builder) -> Result<Self, Self::Error> {
        builder.add_from_file("../res/ui/about.ui")?;
        let root: gtk::Window = builder.get_object("preferences").expect("Could not get preferences window");
        Ok(Self { root })
    }
}
