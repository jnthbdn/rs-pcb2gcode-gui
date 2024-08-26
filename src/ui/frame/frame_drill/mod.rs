mod imp;

use std::sync::{Arc, Mutex};

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{database::database::Database, ui::bool_to_str, units::UnitString};

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

    pub fn refresh_tools(&self) {
        self.imp().drill_tool.refresh_tools();
        self.imp().milldrilling_tool.refresh_tools();
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.imp().drill_tool.set_database(db.clone());
        self.imp().milldrilling_tool.set_database(db);
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

    pub fn get_string_param(&self, db: Arc<Mutex<Database>>) -> Result<String, String> {
        let mut result = String::new();

        let drill = self.imp().drill_tool.get_selected();

        if drill.is_none() || drill.as_ref().unwrap().is_category() {
            return Err("No drill selected".to_string());
        }

        let db = db.lock().unwrap();
        let drill = match db.get_drill(drill.unwrap().get_tool_id().unwrap()) {
            Ok(drill) => drill.unwrap(),
            Err(e) => return Err(format!("{e}")),
        };

        result += &format!("--zdrill={} ", self.imp().depth.value_str(true));
        result += &format!("--drill-feed={} ", drill.base_tool.feed_rate);
        result += &format!("--drill-speed={} ", drill.base_tool.spindle_speed);
        result += &format!(
            "--drill-side={} ",
            self.imp()
                .side
                .selected_item()
                .and_downcast::<gtk::StringObject>()
                .unwrap()
                .string()
        );

        if self.imp().enable_milldrilling.is_active() {
            let mill_tool = self.imp().milldrilling_tool.get_selected();

            if mill_tool.is_none() || mill_tool.as_ref().unwrap().is_category() {
                return Err("No tool selected for milldrilling".to_string());
            }

            let mill_tool = match db.get_endmill(mill_tool.unwrap().get_tool_id().unwrap()) {
                Ok(t) => t.unwrap(),
                Err(e) => return Err(format!("{e}")),
            };

            result += &format!(
                "--min-milldrill-hole-diameter={} ",
                self.imp().milldrilling_min_diameter.value_str(true)
            );
            result += &format!(
                "--milldrill-diameter={} ",
                mill_tool.base_tool.tool_diameter
            );
            result += &format!(
                "--zmilldrill={} ",
                self.imp().milldrilling_depth.value_str(true)
            );
            result += &format!("--drill-feed={} ", drill.base_tool.feed_rate);
        } else {
            result += "--min-milldrill-hole-diameter=inf ";
        }

        result += &format!(
            "--nog91-1={} ",
            bool_to_str(self.imp().no_g91_1.is_active())
        );
        result += &format!("--nog81={} ", bool_to_str(self.imp().no_g81.is_active()));
        result += "--onedrill=true ";

        Ok(result)
    }
}
