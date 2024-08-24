mod imp;

use gtk::{glib, prelude::ObjectExt, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::units::UnitString;

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

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().safe_z.set_postfix(unit.measure());
        self.imp().tool_change.set_postfix(unit.measure());
        self.imp().tolerance.set_postfix(unit.measure());
        self.imp().optimization.set_postfix(unit.optimization());
        self.imp().offset_x.set_postfix(unit.measure());
        self.imp().offset_y.set_postfix(unit.measure());
        self.imp().mirror_x.set_postfix(unit.measure());
    }

    #[template_callback]
    fn ouput_unit_metric_toogle(&self, check: gtk::CheckButton) {
        self.emit_by_name::<()>("output-unit-change", &[&check.is_active()]);
    }
}
