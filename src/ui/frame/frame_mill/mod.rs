mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    glib,
    prelude::{CastNone, CheckButtonExt},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{database::database::Database, tools::ToolType, ui::bool_to_str, units::UnitString};

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

    pub fn refresh_tools(&self) {
        self.imp().mill_tool.refresh_tools();
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.imp().mill_tool.set_database(db.clone());
    }

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().isolation.set_postfix(unit.measure());
    }

    #[template_callback]
    pub fn voronoi_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_enable_voronoi(check.is_active());
    }

    pub fn get_string_param(&self, db: Arc<Mutex<Database>>) -> Result<String, String> {
        let mut result = String::new();

        let mill = self.imp().mill_tool.get_selected();

        if mill.is_none() || mill.as_ref().unwrap().is_category() {
            return Err("No tool selected".to_string());
        }

        let db = db.lock().unwrap();
        let mill = mill.unwrap();

        let (diameter, base_tool) = match mill.get_tool_type() {
            ToolType::Drill => return Err("Bad tool...".to_string()),
            ToolType::Endmill => {
                let tool = db
                    .get_endmill(mill.get_tool_id().unwrap())
                    .unwrap()
                    .unwrap();
                (tool.base_tool.tool_diameter, tool.base_tool)
            }
            ToolType::VBit => {
                let tool = db.get_vbit(mill.get_tool_id().unwrap()).unwrap().unwrap();
                (tool.diameter(self.imp().depth.value()), tool.base_tool)
            }
        };

        result += &format!("--voronoi={} ", bool_to_str(self.imp().voronoi.is_active()));
        result += &format!("--mill-diameters={} ", diameter);
        result += &format!("--milling-overlap={} ", self.imp().overlap.value_str(true));
        result += &format!(
            "--isolation-width={} ",
            self.imp().isolation.value_str(true)
        );
        result += &format!("--pre-milling-gcode={} ", self.imp().pre_milling.all_text());
        result += &format!(
            "--post-milling-gcode={} ",
            self.imp().post_milling.all_text()
        );
        result += &format!("--zwork={} ", self.imp().depth.value_str(true));
        result += &format!("--mill-feed={} ", base_tool.feed_rate);
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
