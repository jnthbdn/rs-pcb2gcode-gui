mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct SelectToolObject(ObjectSubclass<imp::SelectToolObject>)
        @extends gtk::Widget, gtk::Box;
}

#[gtk::template_callbacks]
impl SelectToolObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
