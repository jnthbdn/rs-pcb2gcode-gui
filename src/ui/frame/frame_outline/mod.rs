mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct FrameOutline(ObjectSubclass<imp::FrameOutline>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameOutline {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
