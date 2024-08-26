#![allow(unreachable_code)]

use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::{
    database::database::Database,
    settings::Settings,
    ui::{
        frame::{
            frame_autoleveling::FrameAutoleveling, frame_common::FrameCommon,
            frame_drill::FrameDrill, frame_input_output::FrameInputOutput, frame_mill::FrameMill,
            frame_outline::FrameOutline,
        },
        window::window_tool_db::WindowToolDB,
    },
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window/window_main.ui")]
#[properties(wrapper_type=super::WindowMain)]
pub struct WindowMain {
    pub database: Arc<Mutex<Database>>,

    pub win_tool_db: RefCell<Option<WindowToolDB>>,

    pub settings: RefCell<Settings>,

    #[template_child]
    pub frame_input_output: TemplateChild<FrameInputOutput>,

    #[template_child]
    pub frame_common: TemplateChild<FrameCommon>,

    #[template_child]
    pub frame_mill: TemplateChild<FrameMill>,

    #[template_child]
    pub frame_drill: TemplateChild<FrameDrill>,

    #[template_child]
    pub frame_outline: TemplateChild<FrameOutline>,

    #[template_child]
    pub frame_autolevel: TemplateChild<FrameAutoleveling>,
}

#[glib::object_subclass]
impl ObjectSubclass for WindowMain {
    const NAME: &'static str = "WindowMain";
    type Type = super::WindowMain;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        FrameInputOutput::ensure_type();
        FrameCommon::ensure_type();
        FrameMill::ensure_type();
        FrameDrill::ensure_type();
        FrameOutline::ensure_type();
        FrameAutoleveling::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for WindowMain {
    fn constructed(&self) {
        self.parent_constructed();

        self.frame_mill.set_database(self.database.clone());
        self.frame_drill.set_database(self.database.clone());
        self.frame_outline.set_database(self.database.clone());
        self.obj().output_unit_change(true);
    }
}

impl WidgetImpl for WindowMain {}
impl WindowImpl for WindowMain {
    fn close_request(&self) -> glib::Propagation {
        self.obj().save_window_settings();

        glib::Propagation::Proceed
    }
}
impl ApplicationWindowImpl for WindowMain {}
