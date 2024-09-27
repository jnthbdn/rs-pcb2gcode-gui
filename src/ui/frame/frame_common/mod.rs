mod imp;

use gtk::{glib, prelude::ObjectExt, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{
    settings::settings_frame_common::SettingsFrameCommon, ui::bool_to_str, units::UnitString,
};

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

    pub fn load_frame_settings(&self, settings: &SettingsFrameCommon) {
        let self_imp = self.imp();

        self_imp
            .input_unit_metric
            .set_active(settings.is_input_metric());
        self_imp
            .input_unit_imperial
            .set_active(!settings.is_input_metric());
        self_imp
            .output_unit_metric
            .set_active(settings.is_output_metric());
        self_imp
            .output_unit_imperial
            .set_active(!settings.is_output_metric());
        self_imp.safe_z.init_value(settings.safe_z());
        self_imp.tool_change.init_value(settings.tool_change());
        self_imp
            .tool_change_as_machine_coord
            .set_active(settings.is_tool_change_as_machine_coord());
        self_imp.tolerance.init_value(settings.tolerance());
        self_imp.optimization.init_value(settings.optimization());
        self_imp.tiles_x.init_value(settings.tile_x());
        self_imp.tiles_y.init_value(settings.tile_y());
        self_imp.offset_x.init_value(settings.offset_x());
        self_imp.offset_y.init_value(settings.offset_y());
        self_imp.mirror_x.init_value(settings.mirror_x());
        self_imp
            .mirror_y_instead_x
            .set_active(settings.is_mirror_y());
        self_imp.zero_start.set_active(settings.is_zero_start());
    }

    pub fn save_frame_settings(&self, settings: &mut SettingsFrameCommon) {
        let self_imp = self.imp();

        settings.set_is_input_metric(self_imp.input_unit_metric.is_active());
        settings.set_is_output_metric(self_imp.output_unit_metric.is_active());
        settings.set_safe_z(self_imp.safe_z.value());
        settings.set_tool_change(self_imp.tool_change.value());
        settings
            .set_is_tool_change_as_machine_coord(self_imp.tool_change_as_machine_coord.is_active());
        settings.set_tolerance(self_imp.tolerance.value());
        settings.set_optimization(self_imp.optimization.value());
        settings.set_tile_x(self_imp.tiles_x.value());
        settings.set_tile_y(self_imp.tiles_y.value());
        settings.set_offset_x(self_imp.offset_x.value());
        settings.set_offset_y(self_imp.offset_y.value());
        settings.set_mirror_x(self_imp.mirror_x.value());
        settings.set_is_mirror_y(self_imp.mirror_y_instead_x.is_active());
        settings.set_is_zero_start(self_imp.zero_start.is_active());
    }

    #[template_callback]
    fn ouput_unit_metric_toogle(&self, check: gtk::CheckButton) {
        self.emit_by_name::<()>("output-unit-change", &[&check.is_active()]);
    }

    pub fn get_params(&self) -> Result<Vec<String>, String> {
        let mut result = Vec::new();

        result.push(format!(
            "--metric={}",
            if self.imp().input_unit_metric.is_active() {
                "true"
            } else {
                "false"
            }
        ));

        result.push(format!(
            "--metricoutput={}",
            bool_to_str(self.imp().output_unit_metric.is_active())
        ));

        result.push(format!("--zsafe={}", self.imp().safe_z.value_str(true)));
        result.push(format!(
            "--zchange={}",
            self.imp().tool_change.value_str(true)
        ));
        result.push(format!(
            "--zchange-absolute={}",
            bool_to_str(self.imp().tool_change_as_machine_coord.is_active())
        ));
        result.push(format!(
            "--tolerance={}",
            self.imp().tolerance.value_str(false)
        ));
        result.push(format!(
            "--optimise={}",
            self.imp().optimization.value_str(true)
        ));
        result.push(format!("--tile-x={}", self.imp().tiles_x.value_str(true)));
        result.push(format!("--tile-y={}", self.imp().tiles_y.value_str(true)));
        result.push(format!(
            "--x-offset={}",
            self.imp().offset_x.value_str(true)
        ));
        result.push(format!(
            "--y-offset={}",
            self.imp().offset_y.value_str(true)
        ));
        result.push(format!(
            "--mirror-axis={}",
            self.imp().mirror_x.value_str(true)
        ));
        result.push(format!(
            "--mirror-yaxis={}",
            bool_to_str(self.imp().mirror_y_instead_x.is_active())
        ));
        result.push(format!(
            "--zero-start={}",
            bool_to_str(self.imp().zero_start.is_active())
        ));

        Ok(result)
    }
}
