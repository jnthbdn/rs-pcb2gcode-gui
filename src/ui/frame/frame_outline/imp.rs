#![allow(unreachable_code)]
use std::sync::Mutex;

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::{
    database::database::Database,
    ui::{
        object::{
            info_tooltip_object::InfoToolTipObject, select_tool_object::SelectToolObject,
            spin_button_object::SpinButtonObject,
        },
        READONLY_CSS_CLASS,
    },
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_outline.ui")]
#[properties(wrapper_type=super::FrameOutline)]
pub struct FrameOutline {
    #[template_child]
    pub tool: TemplateChild<SelectToolObject>,

    #[template_child]
    pub depth: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub side: TemplateChild<gtk::DropDown>,

    #[template_child]
    pub fill_outline: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub enable_bridge: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub bridge_width: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub bridge_number: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub bridge_depth: TemplateChild<SpinButtonObject>,
}

impl FrameOutline {
    pub fn set_database(db: Mutex<Database>) {}

    pub fn set_bridge_enable(&self, is_enable: bool) {
        self.bridge_depth.set_can_target(is_enable);
        self.bridge_number.set_can_target(is_enable);
        self.bridge_width.set_can_target(is_enable);

        if is_enable {
            self.bridge_depth.remove_css_class(READONLY_CSS_CLASS);
            self.bridge_number.remove_css_class(READONLY_CSS_CLASS);
            self.bridge_width.remove_css_class(READONLY_CSS_CLASS);
        } else {
            self.bridge_depth.add_css_class(READONLY_CSS_CLASS);
            self.bridge_number.add_css_class(READONLY_CSS_CLASS);
            self.bridge_width.add_css_class(READONLY_CSS_CLASS);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FrameOutline {
    const NAME: &'static str = "FrameOutline";
    type Type = super::FrameOutline;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        SpinButtonObject::ensure_type();
        SelectToolObject::ensure_type();
        InfoToolTipObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for FrameOutline {
    fn constructed(&self) {
        self.parent_constructed();

        self.set_bridge_enable(false);
    }

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
impl WidgetImpl for FrameOutline {}
impl BoxImpl for FrameOutline {}
