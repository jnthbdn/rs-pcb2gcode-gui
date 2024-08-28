mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    ffi::GTK_INVALID_LIST_POSITION,
    gio,
    glib::{self, property::PropertySet},
    prelude::{CastNone, ListModelExt},
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::ui::object::tree_tool_row::TreeToolRow;
use crate::{database::database::Database, tools::ToolType};

glib::wrapper! {
    pub struct SelectToolObject(ObjectSubclass<imp::SelectToolObject>)
        @extends gtk::Widget, gtk::Box;
}

#[gtk::template_callbacks]
impl SelectToolObject {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>, is_metric: bool) {
        self.imp().database.set(Some(db));
        self.refresh_tools(is_metric);
    }

    pub fn refresh_tools(&self, is_metric: bool) {
        self.imp().generate_list(is_metric);
    }

    pub fn get_selected(&self) -> Option<TreeToolRow> {
        if self.imp().dropdown.selected() == GTK_INVALID_LIST_POSITION {
            None
        } else {
            self.imp().dropdown.selected_item().and_downcast()
        }
    }

    pub fn select_item(&self, tool_type: ToolType, tool_id: u32) {
        let list = self
            .imp()
            .dropdown
            .model()
            .and_downcast::<gio::ListStore>()
            .expect("list need to be ListStore");

        for i in 0..list.n_items() {
            let item: TreeToolRow = list
                .item(i)
                .and_downcast()
                .expect("item need to be TreeToolRow");

            if item.is_tool()
                && item.get_tool_type().as_ref().unwrap() == &tool_type
                && item.get_tool_id().as_ref().unwrap() == &tool_id
            {
                self.imp().dropdown.set_selected(i);
                return;
            }
        }
    }
}
