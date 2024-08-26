#![allow(unreachable_code)]
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::database::database::Database;
use crate::tools::ToolType;
use crate::ui::object::db_label_object::DBLabelObject;
use crate::ui::object::tree_tool_row::TreeToolRow;

// Object holding the state
#[derive(gtk::CompositeTemplate, glib::Properties)]
#[template(
    resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/object/select_tool_object.ui"
)]
#[properties(wrapper_type = super::SelectToolObject)]
pub struct SelectToolObject {
    #[template_child]
    pub dropdown: TemplateChild<gtk::DropDown>,

    // #[template_child]
    // pub model: TemplateChild<gtk::StringList>,
    #[property(get, set)]
    show_drill: Cell<bool>,

    #[property(get, set)]
    show_endmill: Cell<bool>,

    #[property(get, set)]
    show_vbit: Cell<bool>,

    pub database: RefCell<Option<Arc<Mutex<Database>>>>,
}

impl SelectToolObject {
    pub fn factory_setup(&self, list_item: &glib::Object) {
        let list_item: &gtk::ListItem = list_item
            .downcast_ref()
            .expect("list_item need to be ListItem");

        list_item.set_child(Some(&DBLabelObject::new()));
    }

    pub fn factory_bind(&self, list_item: &glib::Object) {
        let list_item: &gtk::ListItem = list_item
            .downcast_ref()
            .expect("list_item need to be ListItem");

        let item: TreeToolRow = list_item
            .item()
            .and_downcast()
            .expect("item need to be TreeToolRow");

        let child: DBLabelObject = list_item
            .child()
            .and_downcast()
            .expect("child need to be DBLabelObject");

        child.set_label(&item.name());

        match item.get_tool_id() {
            Some(id) => child.set_db_id(id),
            None => (),
        };

        if item.is_category() {
            child.add_css_class("dropdown_category");
            list_item.set_selectable(false);
            list_item.set_focusable(false);
            list_item.set_activatable(false);
        } else {
            child.remove_css_class("dropdown_category");
        }
    }

    pub fn generate_list(&self, is_metric: bool) {
        if self.database.borrow().is_none() {
            log::error!("No database, unable to generate list");
            return;
        }

        let mut vec_model: Vec<TreeToolRow> = Vec::new();

        let db = self.database.borrow();
        let db = db.as_ref().unwrap().lock().unwrap();

        if self.show_drill.get() {
            vec_model.push(TreeToolRow::new_category(
                "Drill".to_string(),
                ToolType::Drill,
                is_metric,
            ));

            for drill in db.get_all_drills(is_metric).unwrap() {
                vec_model.push(TreeToolRow::new_drill_tool(
                    drill.base_tool.name,
                    drill.base_tool.id,
                    is_metric,
                ))
            }
        }

        if self.show_endmill.get() {
            vec_model.push(TreeToolRow::new_category(
                "Endmill".to_string(),
                ToolType::Endmill,
                is_metric,
            ));

            for endmill in db.get_all_endmills(is_metric).unwrap() {
                vec_model.push(TreeToolRow::new_endmill_tool(
                    endmill.base_tool.name,
                    endmill.base_tool.id,
                    is_metric,
                ))
            }
        }

        if self.show_vbit.get() {
            vec_model.push(TreeToolRow::new_category(
                "V-Bit".to_string(),
                ToolType::VBit,
                is_metric,
            ));

            for vbit in db.get_all_vbits(is_metric).unwrap() {
                vec_model.push(TreeToolRow::new_vbit_tool(
                    vbit.base_tool.name,
                    vbit.base_tool.id,
                    is_metric,
                ))
            }
        }

        let list = gio::ListStore::new::<TreeToolRow>();
        list.extend_from_slice(&vec_model);

        self.dropdown.set_model(Some(&list));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SelectToolObject {
    const NAME: &'static str = "SelectToolObject";
    type Type = super::SelectToolObject;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for SelectToolObject {
    fn constructed(&self) {
        self.parent_constructed();

        let signal_factory = gtk::SignalListItemFactory::new();

        let clone_obj = self.obj().clone();
        signal_factory.connect_setup(move |_factory, object| clone_obj.imp().factory_setup(object));

        let clone_obj = self.obj().clone();
        signal_factory.connect_bind(move |_factory, object| clone_obj.imp().factory_bind(object));

        self.dropdown.set_factory(Some(&signal_factory));
    }
}

impl WidgetImpl for SelectToolObject {}
impl BoxImpl for SelectToolObject {}

impl Default for SelectToolObject {
    fn default() -> Self {
        Self {
            dropdown: Default::default(),
            // model: Default::default(),
            show_drill: Cell::new(true),
            show_endmill: Cell::new(true),
            show_vbit: Cell::new(true),
            database: RefCell::new(None),
        }
    }
}
