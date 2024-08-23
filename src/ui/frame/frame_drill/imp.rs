#![allow(unreachable_code)]
use std::sync::Mutex;

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::{
    database::database::Database,
    ui::object::{
        browse_file_object::BrowseFileObject, select_tool_object::SelectToolObject,
        spin_button_object::SpinButtonObject, textview_object::TextViewObject,
    },
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_drill.ui")]
#[properties(wrapper_type=super::FrameDrill)]
pub struct FrameDrill {}

impl FrameDrill {}

#[glib::object_subclass]
impl ObjectSubclass for FrameDrill {
    const NAME: &'static str = "FrameDrill";
    type Type = super::FrameDrill;
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
impl ObjectImpl for FrameDrill {
    // fn constructed(&self) {
    //     self.parent_constructed();

    // }

    // fn signals() -> &'static [glib::subclass::Signal] {
    //     static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
    //     SIGNALS.get_or_init(|| {
    //         vec![glib::subclass::Signal::builder("setting-changed")
    //             .param_types([
    //                 ToolType::static_type(),
    //                 DatabaseColumn::static_type(),
    //                 glib::GString::static_type(),
    //                 u32::static_type(),
    //             ])
    //             .build()]
    //     })
    // }
}
impl WidgetImpl for FrameDrill {}
impl BoxImpl for FrameDrill {}
