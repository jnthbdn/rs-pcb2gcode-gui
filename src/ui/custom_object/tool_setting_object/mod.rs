mod imp;
use gtk::{self, glib, subclass::prelude::ObjectSubclassIsExt};

use crate::tools::{drill::Drill, endmill::Endmill, vbit::VBit};

glib::wrapper! {
    pub struct ToolSettingObject(ObjectSubclass<imp::ToolSettingObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl ToolSettingObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn show_endmill(&self, endmill: &Endmill) {
        self.imp().show_endmill(endmill);
    }

    pub fn show_drill(&self, drill: &Drill) {
        self.imp().show_drill(drill);
    }

    pub fn show_vbit(&self, vbit: &VBit) {
        self.imp().show_vbit(vbit);
    }
}
