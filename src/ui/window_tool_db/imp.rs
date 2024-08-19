#![allow(unreachable_code)]

use std::sync::Arc;

use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    custom_object::tree_tool_row::TreeToolRow,
    database::database::Database,
    tools::ToolType,
    ui::custom_object::{tool_setting_object::ToolSettingObject, tree_tool_object::TreeToolObject},
};

#[derive(gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window_tool_db.ui")]
#[properties(wrapper_type=super::WindowToolDB)]
pub struct WindowToolDB {
    #[template_child]
    pub tree_tool: TemplateChild<TreeToolObject>,
    #[template_child]
    pub tool_settings: TemplateChild<ToolSettingObject>,

    pub database: Arc<Database>,
}

impl WindowToolDB {
    pub fn refresh_model(&self) {
        self.tree_tool.imp().refresh_tree(&self.database);
    }
}

impl Default for WindowToolDB {
    fn default() -> Self {
        Self {
            tree_tool: Default::default(),
            tool_settings: Default::default(),
            database: Arc::new(Database::new().expect("[default] Unable to create database")),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WindowToolDB {
    const NAME: &'static str = "WindowToolDB";
    type Type = super::WindowToolDB;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        ToolSettingObject::ensure_type();
        TreeToolObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for WindowToolDB {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj().setup_actions();

        self.tool_settings.set_visible(false);

        self.tree_tool.imp().set_root_elements(
            vec![
                TreeToolRow::new_category("Drill".to_string(), ToolType::Drill),
                TreeToolRow::new_category("Endmill".to_string(), ToolType::Endmill),
                TreeToolRow::new_category("V bit".to_string(), ToolType::VBit),
            ],
            &self.database,
        );
    }
}
impl WidgetImpl for WindowToolDB {}
impl WindowImpl for WindowToolDB {}
impl ApplicationWindowImpl for WindowToolDB {}
