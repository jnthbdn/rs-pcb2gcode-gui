mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::units::UnitString;

glib::wrapper! {
    pub struct FrameDrill(ObjectSubclass<imp::FrameDrill>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameDrill {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().milldrilling_depth.set_postfix(unit.measure());
        self.imp()
            .milldrilling_min_diameter
            .set_postfix(unit.measure());
    }

    #[template_callback]
    fn enable_endmill_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_enable_milldrilling(check.is_active());
    }
}
