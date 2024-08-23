mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct FrameAutoleveling(ObjectSubclass<imp::FrameAutoleveling>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameAutoleveling {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
