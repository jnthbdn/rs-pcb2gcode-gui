mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct TreeToolObject(ObjectSubclass<imp::TreeToolObject>)
        @extends gtk::Widget, gtk::Box;
}

impl TreeToolObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
