mod imp;

use gtk::{glib, prelude::CheckButtonExt, subclass::prelude::ObjectSubclassIsExt};

use crate::units::UnitString;

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

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().isolation.set_postfix(unit.measure());
    }

    #[template_callback]
    pub fn voronoi_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_enable_voronoi(check.is_active());
    }
}
