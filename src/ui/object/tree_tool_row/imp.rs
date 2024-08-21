use std::cell::Cell;
use std::cell::OnceCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::TreeObjectType;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::TreeToolRow)]
pub struct TreeToolRow {
    #[property(get = Self::get_name, set)]
    name: Cell<String>,

    pub tool_type: OnceCell<TreeObjectType>,
}

impl TreeToolRow {
    pub fn get_name(&self) -> String {
        let s = self.name.take();
        let clone = s.clone();
        self.name.set(s);

        clone
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TreeToolRow {
    const NAME: &'static str = "TreeToolRow";
    type Type = super::TreeToolRow;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for TreeToolRow {}
