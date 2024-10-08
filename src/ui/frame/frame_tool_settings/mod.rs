mod imp;
use gtk::{
    self, glib,
    prelude::EditableExt,
    prelude::{BuildableExt, ObjectExt, ToValue},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{
    database::database::DatabaseColumn,
    tools::{drill::Drill, endmill::Endmill, vbit::VBit},
    units::UnitString,
};

use crate::ui::object::{
    entry_object::EntryObject, spin_button_object::SpinButtonObject,
    textview_object::TextViewObject,
};

static MAP_NAME_COLUMN: [(&str, DatabaseColumn); 11] = [
    ("general_id", DatabaseColumn::ID),
    ("general_name", DatabaseColumn::Name),
    ("general_note", DatabaseColumn::Note),
    ("diameter_shaft", DatabaseColumn::ShaftDiameter),
    ("diameter_tool", DatabaseColumn::ToolDiameter),
    ("diameter_tip", DatabaseColumn::TipDiameter),
    ("tool_angle", DatabaseColumn::ToolAngle),
    ("pass_depth", DatabaseColumn::PassDepth),
    ("speed_spindle", DatabaseColumn::SpindleSpeed),
    ("speed_vertical", DatabaseColumn::PlungeRate),
    ("speed_horizontal", DatabaseColumn::FeedRate),
];

glib::wrapper! {
    pub struct FrameToolSettings(ObjectSubclass<imp::FrameToolSettings>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameToolSettings {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn update_units(&self, unit: &UnitString) {
        self.imp().diameter_shaft.set_postfix(unit.measure());
        self.imp().diameter_tool.set_postfix(unit.measure());
        self.imp().diameter_tip.set_postfix(unit.measure());
        self.imp().pass_depth.set_postfix(unit.measure());
        self.imp().speed_vertical.set_postfix(unit.feedrate());
        self.imp().speed_horizontal.set_postfix(unit.feedrate());
    }

    pub fn show_endmill(&self, endmill: &Endmill) {
        self.imp().show_endmill(endmill);
        self.update_units(&endmill.base_tool.unit);
    }

    pub fn show_drill(&self, drill: &Drill) {
        self.imp().show_drill(drill);
        self.update_units(&drill.base_tool.unit);
    }

    pub fn show_vbit(&self, vbit: &VBit) {
        self.imp().show_vbit(vbit);
        self.update_units(&vbit.base_tool.unit);
    }

    #[template_callback]
    fn entry_changed(&self, obj: EntryObject) {
        if obj.buildable_id().is_none() {
            return;
        }

        let id = obj.buildable_id().unwrap();
        let col = MAP_NAME_COLUMN.iter().find(|x| x.0 == id).take();

        if col.is_some() {
            let col = col.unwrap();

            self.emit_by_name(
                "setting-changed",
                &[
                    &self.imp().current_tooltype().unwrap(),
                    &col.1,
                    &obj.text().to_value(),
                    &self.imp().current_id(),
                ],
            )
        }
    }

    #[template_callback]
    fn textview_changed(&self, obj: TextViewObject) {
        if obj.buildable_id().is_none() {
            return;
        }

        let id = obj.buildable_id().unwrap();
        let col = MAP_NAME_COLUMN.iter().find(|x| x.0 == id).take();

        if col.is_some() {
            let col = col.unwrap();

            self.emit_by_name(
                "setting-changed",
                &[
                    &self.imp().current_tooltype().unwrap(),
                    &col.1,
                    &obj.all_text().to_value(),
                    &self.imp().current_id(),
                ],
            )
        }
    }

    #[template_callback]
    fn spin_changed(&self, obj: SpinButtonObject) {
        if obj.buildable_id().is_none() {
            return;
        }

        let id = obj.buildable_id().unwrap();
        let col = MAP_NAME_COLUMN.iter().find(|x| x.0 == id).take();

        if col.is_some() {
            let col = col.unwrap();

            self.emit_by_name(
                "setting-changed",
                &[
                    &self.imp().current_tooltype().unwrap(),
                    &col.1,
                    &obj.value_str(false).to_value(),
                    &self.imp().current_id(),
                ],
            )
        }
    }
}
