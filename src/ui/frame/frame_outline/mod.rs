mod imp;

use gtk::{glib, prelude::CheckButtonExt, subclass::prelude::ObjectSubclassIsExt};

use crate::units::UnitString;

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

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().bridge_width.set_postfix(unit.measure());
        self.imp().bridge_depth.set_postfix(unit.measure());
    }

    #[template_callback]
    pub fn enable_bridge_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_bridge_enable(check.is_active());
    }
}
