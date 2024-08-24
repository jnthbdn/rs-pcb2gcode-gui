mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    glib::{self, property::PropertySet},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::database::database::Database;

glib::wrapper! {
    pub struct SelectToolObject(ObjectSubclass<imp::SelectToolObject>)
        @extends gtk::Widget, gtk::Box;
}

#[gtk::template_callbacks]
impl SelectToolObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.imp().database.set(Some(db));
        self.refresh_tools();
    }

    pub fn refresh_tools(&self) {
        self.imp().generate_list();
    }
}
