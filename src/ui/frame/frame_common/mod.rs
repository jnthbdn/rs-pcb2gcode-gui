mod imp;

use gtk::{glib, prelude::ObjectExt, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{ui::bool_to_str, units::UnitString};

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

    pub fn get_string_param(&self) -> Result<String, String> {
        let mut result = String::new();

        result += &format!(
            "--metric={} ",
            if self.imp().input_unit_metric.is_active() {
                "true"
            } else {
                "false"
            }
        );

        result += &format!(
            "--metricoutput={} ",
            bool_to_str(self.imp().output_unit_metric.is_active())
        );

        result += &format!("--zsafe={} ", self.imp().safe_z.value_str(true));
        result += &format!("--zchange={} ", self.imp().tool_change.value_str(true));
        result += &format!(
            "--zchange-absolute={} ",
            bool_to_str(self.imp().tool_change_as_machine_coord.is_active())
        );
        result += &format!("--tolerance={} ", self.imp().tolerance.value_str(true));
        result += &format!("--optimise={} ", self.imp().optimization.value_str(true));
        result += &format!("--tile-x={} ", self.imp().tiles_x.value_str(true));
        result += &format!("--tile-y={} ", self.imp().tiles_y.value_str(true));
        result += &format!("--x-offset={} ", self.imp().offset_x.value_str(true));
        result += &format!("--y-offset={} ", self.imp().offset_y.value_str(true));
        result += &format!("--mirror-axis={} ", self.imp().mirror_x.value_str(true));
        result += &format!(
            "--mirror-yaxis={} ",
            bool_to_str(self.imp().mirror_y_instead_x.is_active())
        );
        result += &format!(
            "--zero-start={} ",
            bool_to_str(self.imp().zero_start.is_active())
        );

        Ok(result)
    }
}
