mod imp;

use gtk::{glib, subclass::prelude::ObjectSubclassIsExt, template_callbacks};

glib::wrapper! {
    pub struct SpinButtonObject(ObjectSubclass<imp::SpinButtonObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable;
}

#[template_callbacks]
impl SpinButtonObject {
    pub fn init_value(&self, val: f64) {
        self.imp().old_value.set(val);
        self.set_value(val);
    }

    pub fn value_str(&self, add_postfix: bool) -> String {
        if add_postfix {
            format!("{}{}", self.imp().value_str(), self.imp().postfix.borrow())
        } else {
            self.imp().value_str()
        }
    }

    pub fn has_focus(&self) -> bool {
        self.imp().has_focus()
    }
}
