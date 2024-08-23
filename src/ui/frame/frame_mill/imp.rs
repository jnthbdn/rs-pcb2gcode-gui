#![allow(unreachable_code)]
use std::sync::Mutex;

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::{
    database::database::Database,
    ui::object::{
        browse_file_object::BrowseFileObject, info_tooltip_object::InfoToolTipObject,
        select_tool_object::SelectToolObject, spin_button_object::SpinButtonObject,
        textview_object::TextViewObject,
    },
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_mill.ui")]
#[properties(wrapper_type=super::FrameMill)]
pub struct FrameMill {}

impl FrameMill {
    pub fn set_database(db: Mutex<Database>) {}
}

#[glib::object_subclass]
impl ObjectSubclass for FrameMill {
    const NAME: &'static str = "FrameMill";
    type Type = super::FrameMill;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        BrowseFileObject::ensure_type();
        SpinButtonObject::ensure_type();
        SelectToolObject::ensure_type();
        TextViewObject::ensure_type();
        InfoToolTipObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for FrameMill {
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
impl WidgetImpl for FrameMill {}
impl BoxImpl for FrameMill {}
