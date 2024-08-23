#![allow(unreachable_code)]
use std::cell::{Cell, RefCell};
use std::sync::Mutex;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::database::database::Database;

// Object holding the state
#[derive(gtk::CompositeTemplate, glib::Properties)]
#[template(
    resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/object/select_tool_object.ui"
)]
#[properties(wrapper_type = super::SelectToolObject)]
pub struct SelectToolObject {
    #[template_child]
    pub dropdown: TemplateChild<gtk::DropDown>,

    #[template_child]
    pub model: TemplateChild<gtk::StringList>,

    #[property(get, set)]
    show_drill: Cell<bool>,

    #[property(get, set)]
    show_endmill: Cell<bool>,

    #[property(get, set)]
    show_vbit: Cell<bool>,

    database: RefCell<Option<Mutex<Database>>>,
}

impl SelectToolObject {
    fn generate_list(&self) {
        if self.database.borrow().is_none() {
            log::error!("No database, unable to generate list");
            return;
        }

        self.clear_model();

        if self.show_drill.get() {
            for drill in self
                .database
                .borrow()
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .get_all_drills()
                .unwrap()
            {
                self.model
                    .append(&format!("[DRILL] {}", drill.base_tool.name));
            }
        }

        if self.show_endmill.get() {
            for endmill in self
                .database
                .borrow()
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .get_all_endmills()
                .unwrap()
            {
                self.model
                    .append(&format!("[ENDMILL] {}", endmill.base_tool.name));
            }
        }

        if self.show_vbit.get() {
            for vbit in self
                .database
                .borrow()
                .as_ref()
                .unwrap()
                .lock()
                .unwrap()
                .get_all_vbits()
                .unwrap()
            {
                self.model
                    .append(&format!("[V-BIT] {}", vbit.base_tool.name));
            }
        }
    }

    fn clear_model(&self) {
        for i in (0..self.model.n_items()).rev() {
            self.model.remove(i);
        }
    }

    pub fn set_database(&self, db: Mutex<Database>) {
        self.database.set(Some(db));
        self.generate_list();
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
impl ObjectImpl for SelectToolObject {}
impl WidgetImpl for SelectToolObject {}
impl BoxImpl for SelectToolObject {}

impl Default for SelectToolObject {
    fn default() -> Self {
        Self {
            dropdown: Default::default(),
            model: Default::default(),
            show_drill: Cell::new(true),
            show_endmill: Cell::new(true),
            show_vbit: Cell::new(true),
            database: RefCell::new(None),
        }
    }
}
