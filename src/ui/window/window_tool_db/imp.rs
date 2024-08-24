#![allow(unreachable_code)]

use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::*,
};

use crate::{
    database::database::Database,
    ui::frame::{frame_tool_settings::FrameToolSettings, frame_tree_tools::FrameTreeTools},
};

#[derive(gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window/window_tool_db.ui")]
#[properties(wrapper_type=super::WindowToolDB)]
pub struct WindowToolDB {
    #[template_child]
    pub tree_tool: TemplateChild<FrameTreeTools>,
    #[template_child]
    pub tool_settings: TemplateChild<FrameToolSettings>,

    pub database: RefCell<Option<Arc<Mutex<Database>>>>,
}

impl WindowToolDB {
    pub fn refresh_model(&self) {
        let db = self.database.borrow().as_ref().unwrap().clone();
        self.tree_tool.imp().refresh_tree(db);
    }
}

impl Default for WindowToolDB {
    fn default() -> Self {
        Self {
            tree_tool: Default::default(),
            tool_settings: Default::default(),
            database: RefCell::new(None),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for WindowToolDB {
    const NAME: &'static str = "WindowToolDB";
    type Type = super::WindowToolDB;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        FrameToolSettings::ensure_type();
        FrameTreeTools::ensure_type();

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

        self.tool_settings.set_visible(false);
    }
}
impl WidgetImpl for WindowToolDB {}
impl WindowImpl for WindowToolDB {}
impl ApplicationWindowImpl for WindowToolDB {}
