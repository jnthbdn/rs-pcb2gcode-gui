mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    glib,
    prelude::{CastNone, CheckButtonExt},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{database::database::Database, ui::bool_to_str, units::UnitString};

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
