mod imp;

use gtk::{
    glib,
    prelude::{Cast, EditableExt},
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct EntryObject(ObjectSubclass<imp::EntryObject>)
        @extends gtk::Widget, gtk::Box, gtk::Entry,
        @implements gtk::Editable, gtk::Buildable;
}

impl EntryObject {
    pub fn set_text(&self, text: &str) {
        *self.imp().old_text.borrow_mut() = text.into();
        self.upcast_ref::<gtk::Entry>().set_text(text);
    }
}
