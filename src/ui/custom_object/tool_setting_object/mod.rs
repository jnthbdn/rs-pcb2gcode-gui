mod imp;
use gtk::{self, glib, prelude::IsA};

glib::wrapper! {
    pub struct ToolSettingObject(ObjectSubclass<imp::ToolSettingObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl ToolSettingObject {
    pub fn new<P: IsA<gtk::Widget>>() -> Self {
        glib::Object::builder().build()
    }
}
