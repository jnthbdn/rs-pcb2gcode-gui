mod imp;

use gtk::{glib, subclass::prelude::ObjectSubclassIsExt, template_callbacks};

glib::wrapper! {
    pub struct SpinButtonObject(ObjectSubclass<imp::SpinButtonObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable;
}

#[template_callbacks]
impl SpinButtonObject {
    pub fn value_str(&self) -> String {
        self.imp().value_str()
    }

    pub fn has_focus(&self) -> bool {
        self.imp().has_focus()
    }
}
