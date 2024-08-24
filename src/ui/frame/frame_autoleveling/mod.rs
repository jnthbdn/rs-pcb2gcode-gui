mod imp;

use gtk::{glib, prelude::CheckButtonExt, subclass::prelude::ObjectSubclassIsExt};

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

    #[template_callback]
    pub fn enable_autolevel_toggled(&self, _: gtk::CheckButton) {
        self.imp().set_autolevel_enable(
            self.imp().enable_front.is_active() || self.imp().enable_back.is_active(),
        )
    }
}
