use std::cell::Cell;
use std::cell::OnceCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::TreeObjectType;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::TreeToolObject)]
pub struct TreeToolObject {
    #[property(get = Self::get_name, set)]
    name: Cell<String>,

    pub tool_type: OnceCell<TreeObjectType>,
}

impl TreeToolObject {
    pub fn get_name(&self) -> String {
        let s = self.name.take();
        let clone = s.clone();
        self.name.set(s);

        clone
    }
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for TreeToolObject {
    const NAME: &'static str = "TreeToolObject";
    type Type = super::TreeToolObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for TreeToolObject {}
