mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    glib,
    prelude::{CastNone, CheckButtonExt},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{
    database::database::Database, settings::settings_frame_outline::SettingsFrameOutline,
    tools::ToolType, ui::bool_to_str, units::UnitString,
};

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

    pub fn refresh_tools(&self) {
        self.imp()
            .tool
            .refresh_tools(self.imp().is_unit_metric.get());
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.imp()
            .tool
            .set_database(db.clone(), self.imp().is_unit_metric.get());
    }

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().bridge_width.set_postfix(unit.measure());
        self.imp().bridge_depth.set_postfix(unit.measure());
        self.imp().is_unit_metric.set(unit.is_metric());
        self.refresh_tools();
    }

    pub fn load_frame_settings(&self, settings: &SettingsFrameOutline) {
        let self_imp = self.imp();

        self_imp
            .tool
            .select_item(settings.tool_type(), settings.tool_id());
        self_imp.side.set_selected(settings.side());
        self_imp.fill_outline.set_active(settings.is_fill_outline());
        self_imp
            .enable_bridge
            .set_active(settings.is_enable_bridge());
        self_imp.bridge_width.init_value(settings.bridge_width());
        self_imp.bridge_number.init_value(settings.bridge_number());
        self_imp.bridge_depth.init_value(settings.bridge_depth());
    }

    pub fn save_frame_settings(&self, settings: &mut SettingsFrameOutline) {
        let self_imp = self.imp();
        let tool = self_imp.tool.get_selected().unwrap();

        settings.set_tool_type(tool.get_tool_type().unwrap_or(ToolType::Endmill));
        settings.set_tool_id(tool.get_tool_id().unwrap_or(u32::MAX));
        settings.set_side(self_imp.side.selected());
        settings.set_is_fill_outline(self_imp.fill_outline.is_active());
        settings.set_is_enable_bridge(self_imp.enable_bridge.is_active());
        settings.set_bridge_width(self_imp.bridge_width.value());
        settings.set_bridge_number(self_imp.bridge_number.value());
        settings.set_bridge_depth(self_imp.bridge_depth.value());
    }

    #[template_callback]
    pub fn enable_bridge_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_bridge_enable(check.is_active());
    }

    pub fn get_string_param(&self, db: Arc<Mutex<Database>>) -> Result<String, String> {
        let mut result = String::new();

        let endmill = self.imp().tool.get_selected();

        if endmill.is_none() || endmill.as_ref().unwrap().is_category() {
            return Err("No tool selected".to_string());
        }

        let db = db.lock().unwrap();
        let endmill = match db.get_endmill(endmill.unwrap().get_tool_id().unwrap()) {
            Ok(tool) => tool.unwrap(),
            Err(e) => return Err(format!("{e}")),
        };

        result += &format!(
            "--fill-outline={} ",
            bool_to_str(self.imp().fill_outline.is_active())
        );
        result += &format!("--cutter-diameter={} ", endmill.base_tool.tool_diameter);
        result += &format!("--zcut={} ", self.imp().depth.value_str(true));
        result += &format!("--cut-feed={} ", endmill.feed_rate);
        result += &format!("--cut-vertfeed={} ", endmill.base_tool.plunge_rate);
        result += &format!("--cut-speed={} ", endmill.base_tool.spindle_speed);
        result += &format!("--cut-infeed={} ", endmill.base_tool.pass_depth);
        result += &format!(
            "--cut-size={} ",
            self.imp()
                .side
                .selected_item()
                .and_downcast::<gtk::StringObject>()
                .unwrap()
                .string()
        );

        if self.imp().enable_bridge.is_active() {
            result += &format!("--bridges={} ", self.imp().bridge_width.value_str(true));
            result += &format!("--bridgesnum={} ", self.imp().bridge_number.value_str(true));
            result += &format!("--zbridges={} ", self.imp().bridge_depth.value_str(true));
        } else {
            result += "--bridges=0 ";
        }

        Ok(result)
    }
}
