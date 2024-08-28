#![allow(unreachable_code)]
use std::sync::OnceLock;

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::object::{
    browse_file_object::BrowseFileObject, info_tooltip_object::InfoToolTipObject,
    spin_button_object::SpinButtonObject,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_common.ui")]
#[properties(wrapper_type=super::FrameCommon)]
pub struct FrameCommon {
    #[template_child]
    pub input_unit_metric: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub output_unit_metric: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub input_unit_imperial: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub output_unit_imperial: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub safe_z: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub tool_change: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub tool_change_as_machine_coord: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub tolerance: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub optimization: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub tiles_x: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub tiles_y: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub offset_x: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub offset_y: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub mirror_x: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub mirror_y_instead_x: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub zero_start: TemplateChild<gtk::CheckButton>,
}

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

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("output-unit-change")
                .param_types([bool::static_type()])
                .build()]
        })
    }
}
impl WidgetImpl for FrameCommon {}
impl BoxImpl for FrameCommon {}
