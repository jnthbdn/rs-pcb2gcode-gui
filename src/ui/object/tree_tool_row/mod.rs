mod imp;

use glib::Object;
use gtk::{glib, subclass::prelude::ObjectSubclassIsExt};

use crate::tools::ToolType;

#[derive(Debug)]
pub enum TreeObjectType {
    UnitCategory(),
    Category(ToolType),
    Tool(ToolType, u32),
}

glib::wrapper! {
    pub struct TreeToolRow(ObjectSubclass<imp::TreeToolRow>);
}

impl TreeToolRow {
    pub fn new_unit_category(name: String, is_metric: bool) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s.imp().tool_type.set(TreeObjectType::UnitCategory());
        let _ = s.imp().is_metric.set(is_metric);
        s
    }

    pub fn new_category(name: String, tool_type: ToolType, is_metric: bool) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s.imp().tool_type.set(TreeObjectType::Category(tool_type));
        let _ = s.imp().is_metric.set(is_metric);

        s
    }

    pub fn new_drill_tool(name: String, db_id: u32, is_metric: bool) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s
            .imp()
            .tool_type
            .set(TreeObjectType::Tool(ToolType::Drill, db_id));
        let _ = s.imp().is_metric.set(is_metric);

        s
    }

    pub fn new_endmill_tool(name: String, db_id: u32, is_metric: bool) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s
            .imp()
            .tool_type
            .set(TreeObjectType::Tool(ToolType::Endmill, db_id));
        let _ = s.imp().is_metric.set(is_metric);

        s
    }

    pub fn new_vbit_tool(name: String, db_id: u32, is_metric: bool) -> Self {
        let s: TreeToolRow = Object::builder().property("name", name).build();

        let _ = s
            .imp()
            .tool_type
            .set(TreeObjectType::Tool(ToolType::VBit, db_id));
        let _ = s.imp().is_metric.set(is_metric);

        s
    }

    pub fn is_category(&self) -> bool {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Category(_) => true,
            TreeObjectType::Tool(_, _) => false,
            TreeObjectType::UnitCategory() => false,
        }
    }

    pub fn is_tool(&self) -> bool {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Category(_) => false,
            TreeObjectType::Tool(_, _) => true,
            TreeObjectType::UnitCategory() => false,
        }
    }

    pub fn is_unit_catergory(&self) -> bool {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Category(_) => false,
            TreeObjectType::Tool(_, _) => false,
            TreeObjectType::UnitCategory() => true,
        }
    }

    pub fn get_name(&self) -> String {
        self.imp().get_name()
    }

    pub fn get_tool_type(&self) -> Option<ToolType> {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Category(tool_type) | TreeObjectType::Tool(tool_type, _) => {
                Some(tool_type.clone())
            }
            TreeObjectType::UnitCategory() => None,
        }
    }

    pub fn get_tool_id(&self) -> Option<u32> {
        match self.imp().tool_type.get().unwrap() {
            TreeObjectType::Tool(_, id) => Some(*id),
            _ => None,
        }
    }

    pub fn is_metric(&self) -> bool {
        *self.imp().is_metric.get().unwrap()
    }
}
