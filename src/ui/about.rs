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
        let root = builder.get_object::<gtk::AboutDialog>("about_dialog")?;
        let close = builder.get_objeect::<gtk::Button>("close_btn")?;
        Ok(Self { root, close })
    }
}
