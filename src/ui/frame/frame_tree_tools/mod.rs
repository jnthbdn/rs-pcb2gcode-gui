mod imp;

use gtk::{glib, prelude::CastNone, subclass::prelude::ObjectSubclassIsExt};

use crate::{tools::ToolType, ui::object::tree_tool_row::TreeToolRow};

use crate::ui::object::db_label_object::DBLabelObject;

glib::wrapper! {
    pub struct FrameTreeTools(ObjectSubclass<imp::FrameTreeTools>)
        @extends gtk::Widget, gtk::Box;
}

impl FrameTreeTools {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn update_name_model(&self, _tool_type: ToolType, _id: u32, new_name: &str) {
        let tree_expander = self.imp().selected_tree.borrow();

        if tree_expander.is_none() {
            return;
        }

        let tree_expander = tree_expander.as_ref().unwrap();
        tree_expander
            .child()
            .and_downcast::<DBLabelObject>()
            .unwrap()
            .set_label(new_name);
        tree_expander
            .item()
            .and_downcast::<TreeToolRow>()
            .unwrap()
            .set_name(new_name);
    }
}
