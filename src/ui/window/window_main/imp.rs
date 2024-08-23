#![allow(unreachable_code)]

use std::cell::RefCell;

use gtk::{glib, prelude::StaticTypeExt, subclass::prelude::*};

use crate::ui::{
    frame::{
        frame_common::FrameCommon, frame_drill::FrameDrill, frame_input_output::FrameInputOutput,
        frame_mill::FrameMill, frame_outline::FrameOutline,
    },
    window::window_tool_db::WindowToolDB,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window/window_main.ui")]
#[properties(wrapper_type=super::WindowMain)]
pub struct WindowMain {
    pub win_tool_db: RefCell<Option<WindowToolDB>>,
    // #[template_child]
    // pub button: TemplateChild<gtk::Button>,
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

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for WindowMain {}
impl WidgetImpl for WindowMain {}
impl WindowImpl for WindowMain {}
impl ApplicationWindowImpl for WindowMain {}
