mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

glib::wrapper! {
    pub struct TextViewObject(ObjectSubclass<imp::TextViewObject>)
        @extends gtk::Widget, gtk::Box, gtk::TextView,
        @implements gtk::Buildable;
}

impl TextViewObject {
    pub fn set_text(&self, text: &str) {
        *self.imp().old_text.borrow_mut() = text.into();
        self.buffer().set_text(text);
    }

    pub fn all_text(&self) -> glib::GString {
        self.buffer()
            .text(&self.buffer().start_iter(), &self.buffer().end_iter(), true)
    }
}
