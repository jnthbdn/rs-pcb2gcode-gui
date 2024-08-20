mod imp;

use gtk::{glib, subclass::prelude::ObjectSubclassIsExt, template_callbacks};

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

    #[template_callback]
    pub fn browse_clicked(&self, _btn: gtk::Button) {
        self.imp().open_dialog();
    }
}
