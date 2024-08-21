mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct FrameInputOutput(ObjectSubclass<imp::FrameInputOutput>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameInputOutput {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
