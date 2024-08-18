mod imp;
use gtk::{self, glib};

glib::wrapper! {
    pub struct ToolSettingObject(ObjectSubclass<imp::ToolSettingObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl ToolSettingObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
