mod imp;

use gtk::{glib, prelude::WidgetExt, subclass::prelude::ObjectSubclassIsExt};

use crate::tools::ToolType;

glib::wrapper! {
    pub struct DBLabelObject(ObjectSubclass<imp::DBLabelObject>)
        @extends gtk::Widget, gtk::Box;
}

impl DBLabelObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_tool_type(&self, tool_type: Option<ToolType>) {
        self.imp().tool_type.set(tool_type);
    }

    pub fn tool_type(&self) -> Option<ToolType> {
        self.imp().tool_type.get()
    }

    pub fn set_db_id(&self, id: u32) {
        self.imp().db_id.set(id);
    }

    pub fn db_id(&self) -> u32 {
        self.imp().db_id.get()
    }

    pub fn set_label(&self, label: &str) {
        self.imp().label.set_label(label);
    }

    pub fn label(&self) -> glib::GString {
        self.imp().label.label()
    }

    pub fn add_css_class(&self, class: &str) {
        self.imp().label.add_css_class(class);
    }
}
