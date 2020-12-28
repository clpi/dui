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
        let root: gtk::Popover = builder.get_object("search_popover").expect("Could not get search popover");
        let search: gtk::SearchBar = builder.get_object("search_bar").expect("Could not get searchbar");
        Ok(Self { root, search })
    }
}
