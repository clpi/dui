use std::convert::TryFrom;
use gtk::prelude::*;

#[derive(Clone, Debug)]
pub struct About {
    pub root: gtk::Popover,
    search: gtk::SearchBar,
}

impl TryFrom<gtk::Builder> for About {

    type Error = Box<dyn std::error::Error>;

    fn try_from(builder: gtk::Builder) -> About {
        builder.add_from_file("../res/ui/about.ui")
            .expect("Could not load about dialog UI file");
        let root = builder.get_object::<gtk::Popover>("search_popover")?;
        let search = builder.get_objeect::<gtk::SearchBar>("search_bar")?;
        Ok(Self { root, search })
    }
}
