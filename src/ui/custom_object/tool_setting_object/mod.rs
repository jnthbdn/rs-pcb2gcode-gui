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
};

use super::entry_object::EntryObject;

static LOOKUPO_NAME_COLUMN: [(&str, DatabaseColumn); 11] = [
    ("general_id", DatabaseColumn::ID),
    ("general_name", DatabaseColumn::NAME),
    ("general_note", DatabaseColumn::NOTE),
    ("diameter_shaft", DatabaseColumn::SHAFT_DIAMETER),
    ("diameter_tool", DatabaseColumn::TOOL_DIAMETER),
    ("diameter_tip", DatabaseColumn::TIP_DIAMETER),
    ("diameter_angle", DatabaseColumn::TOOL_ANGLE),
    ("pass_depth", DatabaseColumn::PASS_DEPTH),
    ("speed_spindle", DatabaseColumn::SPINDLE_SPEED),
    ("speed_vertical", DatabaseColumn::PLUGNE_RATE),
    ("speed_horizontal", DatabaseColumn::FEED_RATE),
];

glib::wrapper! {
    pub struct ToolSettingObject(ObjectSubclass<imp::ToolSettingObject>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl ToolSettingObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn show_endmill(&self, endmill: &Endmill) {
        self.imp().show_endmill(endmill);
    }

    pub fn show_drill(&self, drill: &Drill) {
        self.imp().show_drill(drill);
    }

    pub fn show_vbit(&self, vbit: &VBit) {
        self.imp().show_vbit(vbit);
    }

    #[template_callback]
    fn entry_changed(&self, obj: EntryObject) {
        println!("Entry changed ({:?})", obj.buildable_id());

        if obj.buildable_id().is_none() {
            return;
        }

        let id = obj.buildable_id().unwrap();
        let col = LOOKUPO_NAME_COLUMN.iter().find(|x| x.0 == id).take();

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
}
