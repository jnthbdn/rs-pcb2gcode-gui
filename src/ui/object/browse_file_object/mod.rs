mod imp;

use gtk::{glib, prelude::EditableExt, subclass::prelude::ObjectSubclassIsExt, template_callbacks};

glib::wrapper! {
    pub struct BrowseFileObject(ObjectSubclass<imp::BrowseFileObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Buildable;
}

#[template_callbacks]
impl BrowseFileObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_default_folder(&self, path: &String) {
        self.imp().set_default_folder(path);
    }

    #[template_callback]
    pub fn browse_clicked(&self, _btn: gtk::Button) {
        self.imp().open_dialog();
    }

    pub fn get_path(&self) -> String {
        self.imp().entry.text().into()
    }
}
