mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    ffi::GTK_INVALID_LIST_POSITION,
    glib::{self, property::PropertySet},
    prelude::CastNone,
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::database::database::Database;

use super::db_label_object::DBLabelObject;

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

    pub fn get_selected(&self) -> Option<DBLabelObject> {
        if self.imp().dropdown.selected() == GTK_INVALID_LIST_POSITION {
            None
        } else {
            Some(self.imp().dropdown.selected_item().and_downcast().unwrap())
        }
    }
}
