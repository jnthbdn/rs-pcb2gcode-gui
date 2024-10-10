mod imp;

use std::sync::{Arc, Mutex};

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{
    database::database::Database, settings::settings_frame_drill::SettingsFrameDrill,
    tools::ToolType, ui::bool_to_str, units::UnitString,
};

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

    pub fn load_frame_settings(&self, settings: &SettingsFrameDrill) {
        let self_imp = self.imp();

        self_imp
            .drill_tool
            .select_item(settings.drill_tool_type(), settings.drill_tool_id());

        self_imp.depth.init_value(settings.depth());
        self_imp.side.set_selected(settings.side());
        self_imp
            .enable_milldrilling
            .set_active(settings.is_enable_milldrilling());
        self_imp.milldrilling_tool.select_item(
            settings.milldrilling_tool_type(),
            settings.milldrilling_tool_id(),
        );
        self_imp
            .milldrilling_min_diameter
            .init_value(settings.milldrilling_min_diameter());
        self_imp
            .milldrilling_depth
            .init_value(settings.milldrilling_depth());
        self_imp.no_g91_1.set_active(settings.is_no_g91_1());
        self_imp.no_g81.set_active(settings.is_no_g81());
    }

    pub fn save_frame_settings(&self, settings: &mut SettingsFrameDrill) {
        let self_imp = self.imp();
        let drill_tool = self_imp.drill_tool.get_selected().unwrap();
        let mill_tool = self_imp.milldrilling_tool.get_selected().unwrap();

        settings.set_drill_tool_type(drill_tool.get_tool_type().unwrap_or(ToolType::Drill));
        settings.set_drill_tool_id(drill_tool.get_tool_id().unwrap_or(u32::MAX));
        settings.set_depth(self_imp.depth.value());
        settings.set_side(self_imp.side.selected());
        settings.set_is_enable_milldrilling(self_imp.enable_milldrilling.is_active());
        settings.set_milldrilling_tool_type(mill_tool.get_tool_type().unwrap_or(ToolType::Endmill));
        settings.set_milldrilling_tool_id(mill_tool.get_tool_id().unwrap_or(u32::MAX));
        settings.set_milldrilling_min_diameter(self_imp.milldrilling_min_diameter.value());
        settings.set_milldrilling_depth(self_imp.depth.value());
        settings.set_is_no_g91_1(self_imp.no_g91_1.is_active());
        settings.set_is_no_g81(self_imp.no_g81.is_active());
    }

    pub fn refresh_tools(&self) {
        self.imp()
            .drill_tool
            .refresh_tools(self.imp().is_unit_metric.get());
        self.imp()
            .milldrilling_tool
            .refresh_tools(self.imp().is_unit_metric.get());
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.imp()
            .drill_tool
            .set_database(db.clone(), self.imp().is_unit_metric.get());
        self.imp()
            .milldrilling_tool
            .set_database(db, self.imp().is_unit_metric.get());
    }

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().milldrilling_depth.set_postfix(unit.measure());
        self.imp()
            .milldrilling_min_diameter
            .set_postfix(unit.measure());
        self.imp().is_unit_metric.set(unit.is_metric());
        self.refresh_tools();
    }

    #[template_callback]
    fn enable_endmill_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_enable_milldrilling(check.is_active());
    }

    pub fn get_params(&self, db: Arc<Mutex<Database>>) -> Result<Vec<String>, String> {
        let mut result: Vec<String> = Vec::new();

        let drill = self.imp().drill_tool.get_selected();

        if drill.is_none() || drill.as_ref().unwrap().is_category() {
            return Err("No drill selected".to_string());
        }

        let db = db.lock().unwrap();
        let drill = match db.get_drill(drill.unwrap().get_tool_id().unwrap()) {
            Ok(drill) => drill.unwrap(),
            Err(e) => return Err(format!("{e}")),
        };

        result.push(format!("--zdrill={}", self.imp().depth.value_str(true)));
        result.push(format!(
            "--drill-feed={}{}",
            drill.base_tool.plunge_rate,
            drill.base_tool.unit.feedrate()
        ));
        result.push(format!("--drill-speed={}", drill.base_tool.spindle_speed));
        result.push(format!(
            "--drill-side={}",
            self.imp()
                .side
                .selected_item()
                .and_downcast::<gtk::StringObject>()
                .unwrap()
                .string()
                .to_ascii_lowercase()
        ));

        if self.imp().enable_milldrilling.is_active() {
            let mill_tool = self.imp().milldrilling_tool.get_selected();

            if mill_tool.is_none() || mill_tool.as_ref().unwrap().is_category() {
                return Err("No tool selected for milldrilling".to_string());
            }

            let mill_tool = match db.get_endmill(mill_tool.unwrap().get_tool_id().unwrap()) {
                Ok(t) => t.unwrap(),
                Err(e) => return Err(format!("{e}")),
            };

            result.push(format!(
                "--min-milldrill-hole-diameter={}",
                self.imp().milldrilling_min_diameter.value_str(true)
            ));
            result.push(format!(
                "--milldrill-diameter={}{}",
                mill_tool.base_tool.tool_diameter,
                mill_tool.base_tool.unit.measure()
            ));
            result.push(format!(
                "--zmilldrill={}",
                self.imp().milldrilling_depth.value_str(true)
            ));
        } else {
            // result.push("--min-milldrill-hole-diameter=inf".to_string());
        }

        result.push(format!(
            "--nog91-1={}",
            bool_to_str(self.imp().no_g91_1.is_active())
        ));
        result.push(format!(
            "--nog81={}",
            bool_to_str(self.imp().no_g81.is_active())
        ));
        result.push("--onedrill=true".to_string());

        Ok(result)
    }
}
