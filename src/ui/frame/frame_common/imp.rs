#![allow(unreachable_code)]
use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::object::{
    browse_file_object::BrowseFileObject, info_tooltip_object::InfoToolTipObject,
    spin_button_object::SpinButtonObject,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_common.ui")]
#[properties(wrapper_type=super::FrameCommon)]
pub struct FrameCommon {}

impl FrameCommon {}

#[glib::object_subclass]
impl ObjectSubclass for FrameCommon {
    const NAME: &'static str = "FrameCommon";
    type Type = super::FrameCommon;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        BrowseFileObject::ensure_type();
        SpinButtonObject::ensure_type();
        InfoToolTipObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for FrameCommon {
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
impl WidgetImpl for FrameCommon {}
impl BoxImpl for FrameCommon {}
