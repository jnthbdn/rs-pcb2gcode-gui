mod imp;

use glib::Object;
use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};

use crate::tools::ToolType;

#[derive(Debug)]
pub enum TreeObjectType {
    Category(ToolType),
    Tool(ToolType, u32),
}

glib::wrapper! {
    pub struct TreeToolRow(ObjectSubclass<imp::TreeToolRow>);
}

impl TreeToolRow {
    pub fn new_category(name: String, tool_type: ToolType) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s.imp().tool_type.set(TreeObjectType::Category(tool_type));

        s
    }

    pub fn new_drill_tool(name: String, db_id: u32) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s
            .imp()
            .tool_type
            .set(TreeObjectType::Tool(ToolType::Drill, db_id));

        s
    }

    pub fn new_endmill_tool(name: String, db_id: u32) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s
            .imp()
            .tool_type
            .set(TreeObjectType::Tool(ToolType::Endmill, db_id));

        s
    }

    pub fn new_vbit_tool(name: String, db_id: u32) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s
            .imp()
            .tool_type
            .set(TreeObjectType::Tool(ToolType::VBit, db_id));

        s
    }

    pub fn is_category(&self) -> bool {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Category(_) => true,
            TreeObjectType::Tool(_, _) => false,
        }
    }

    pub fn is_tool(&self) -> bool {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Category(_) => false,
            TreeObjectType::Tool(_, _) => true,
        }
    }

    pub fn get_name(&self) -> String {
        self.imp().get_name()
    }

    pub fn get_tool_type(&self) -> ToolType {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Category(tool_type) | TreeObjectType::Tool(tool_type, _) => {
                tool_type.clone()
            }
        }
    }

    pub fn get_tool_id(&self) -> Result<u32, ()> {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Tool(_, id) => Ok(*id),
            _ => Err(()),
        }
    }
}
