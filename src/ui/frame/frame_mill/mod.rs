mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct FrameMill(ObjectSubclass<imp::FrameMill>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameMill {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
