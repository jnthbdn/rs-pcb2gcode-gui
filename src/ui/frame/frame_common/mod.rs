mod imp;

use gtk::glib;

glib::wrapper! {
    pub struct FrameCommon(ObjectSubclass<imp::FrameCommon>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameCommon {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
