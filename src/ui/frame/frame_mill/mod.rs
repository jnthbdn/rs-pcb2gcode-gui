mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    glib,
    prelude::{CastNone, CheckButtonExt},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{
    database::database::Database,
    settings::settings_frame_mill::SettingsFrameMill,
    tools::ToolType,
    ui::{
        bool_to_str,
        object::{spin_button_object::SpinButtonObject, tree_tool_row::TreeToolRow},
    },
    units::UnitString,
};

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

    pub fn load_frame_settings(&self, settings: &SettingsFrameMill) {
        let self_imp = self.imp();

        self_imp
            .isolation_width_tool
            .set_active(settings.is_isolation_width_tool());
        self_imp.isolation.init_value(settings.isolation());

        self_imp
            .mill_tool
            .select_item(settings.tool_type(), settings.tool_id());
        self_imp.overlap.init_value(settings.overlap());
        self_imp.depth.init_value(settings.depth());
        self_imp.direction.set_selected(settings.direction());
        self_imp
            .invert_gerber
            .set_active(settings.is_invert_gerber());
        self_imp.voronoi.set_active(settings.is_voronoi());
        self_imp
            .thermal_region
            .set_active(settings.is_thermal_region());
        self_imp.pre_milling.set_text(settings.pre_milling());
        self_imp.post_milling.set_text(settings.post_milling());
    }

    pub fn save_frame_settings(&self, settings: &mut SettingsFrameMill) {
        let self_imp = self.imp();
        let tool = self_imp.mill_tool.get_selected().unwrap();

        settings.set_tool_type(tool.get_tool_type().unwrap_or(ToolType::VBit));
        settings.set_tool_id(tool.get_tool_id().unwrap_or(u32::MAX));

        settings.set_overlap(self_imp.overlap.value());
        settings.set_depth(self_imp.depth.value());
        settings.set_direction(self_imp.direction.selected());
        settings.set_is_isolation_width_tool(self_imp.isolation_width_tool.is_active());
        settings.set_isolation(self_imp.isolation.value());
        settings.set_is_invert_gerber(self_imp.invert_gerber.is_active());
        settings.set_is_voronoi(self_imp.voronoi.is_active());
        settings.set_is_thermal_region(self_imp.thermal_region.is_active());
        settings.set_pre_milling(self_imp.pre_milling.all_text().to_string());
        settings.set_post_milling(self_imp.post_milling.all_text().to_string());
    }

    pub fn refresh_tools(&self) {
        self.imp()
            .mill_tool
            .refresh_tools(self.imp().is_unit_metric.get());
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.imp().set_database(db);
    }

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().isolation.set_postfix(unit.measure());
        self.imp().is_unit_metric.set(unit.is_metric());
        self.refresh_tools();
    }

    #[template_callback]
    pub fn voronoi_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_enable_voronoi(check.is_active());
    }

    #[template_callback]
    pub fn isolation_witdh_tool_toggled(&self, check: gtk::CheckButton) {
        self.imp()
            .set_enable_isloation_tool_width(check.is_active());
    }

    #[template_callback]
    pub fn depth_changed(&self, _: SpinButtonObject) {
        if self.imp().isolation_width_tool.is_active() {
            self.imp().set_isolation_with_tool_diameter();
        }
    }

    #[template_callback]
    pub fn tool_selected(&self, _tool: TreeToolRow) {
        let imp = self.imp();

        if imp.isolation_width_tool.is_active() {
            imp.set_isolation_with_tool_diameter();
        }
    }

    pub fn get_string_param(&self, db: Arc<Mutex<Database>>) -> Result<String, String> {
        let mut result = String::new();

        let mill = self.imp().mill_tool.get_selected();

        if mill.is_none() || !mill.as_ref().unwrap().is_tool() {
            return Err("No tool selected".to_string());
        }

        let db = db.lock().unwrap();
        let mill = mill.unwrap();

        let (diameter, feed_rate, base_tool) = match mill.get_tool_type().unwrap() {
            ToolType::Drill => return Err("Bad tool...".to_string()),
            ToolType::Endmill => {
                let tool = db
                    .get_endmill(mill.get_tool_id().unwrap())
                    .unwrap()
                    .unwrap();
                (tool.base_tool.tool_diameter, tool.feed_rate, tool.base_tool)
            }
            ToolType::VBit => {
                let tool = db.get_vbit(mill.get_tool_id().unwrap()).unwrap().unwrap();
                (
                    tool.diameter(self.imp().depth.value()),
                    tool.feed_rate,
                    tool.base_tool,
                )
            }
        };

        result += &format!("--voronoi={} ", bool_to_str(self.imp().voronoi.is_active()));
        result += &format!("--mill-diameters={} ", diameter);
        result += &format!("--milling-overlap={} ", self.imp().overlap.value_str(true));
        result += &format!(
            "--isolation-width={} ",
            self.imp().isolation.value_str(true)
        );
        result += &format!(
            "--pre-milling-gcode=\"{}\" ",
            self.imp().pre_milling.all_text()
        );
        result += &format!(
            "--post-milling-gcode=\"{}\" ",
            self.imp().post_milling.all_text()
        );
        result += &format!("--zwork={} ", self.imp().depth.value_str(true));
        result += &format!("--mill-feed={} ", feed_rate);
        result += &format!("--mill-vertfeed={} ", base_tool.plunge_rate);
        result += &format!("--mill-infeed={} ", base_tool.pass_depth);
        result += &format!("--mill-speed={} ", base_tool.spindle_speed);
        result += &format!(
            "--mill-feed-direction={} ",
            self.imp()
                .direction
                .selected_item()
                .and_downcast::<gtk::StringObject>()
                .unwrap()
                .string()
        );
        result += &format!(
            "--invert-gerbers={} ",
            bool_to_str(self.imp().invert_gerber.is_active())
        );
        result += &format!(
            "--preserve-thermal-reliefs={} ",
            bool_to_str(self.imp().thermal_region.is_active())
        );

        Ok(result)
    }
}
