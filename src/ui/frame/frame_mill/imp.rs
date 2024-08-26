#![allow(unreachable_code)]
use std::cell::Cell;

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::{
    object::{
        browse_file_object::BrowseFileObject, info_tooltip_object::InfoToolTipObject,
        select_tool_object::SelectToolObject, spin_button_object::SpinButtonObject,
        textview_object::TextViewObject,
    },
    READONLY_CSS_CLASS,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_mill.ui")]
#[properties(wrapper_type=super::FrameMill)]
pub struct FrameMill {
    #[template_child]
    pub mill_tool: TemplateChild<SelectToolObject>,

    #[template_child]
    pub overlap: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub depth: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub direction: TemplateChild<gtk::DropDown>,

    #[template_child]
    pub isolation: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub invert_gerber: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub voronoi: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub thermal_region: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub pre_milling: TemplateChild<TextViewObject>,

    #[template_child]
    pub post_milling: TemplateChild<TextViewObject>,

    pub is_unit_metric: Cell<bool>,
}

impl FrameMill {
    pub fn set_enable_voronoi(&self, is_enable: bool) {
        self.thermal_region.set_can_target(is_enable);

        if is_enable {
            self.thermal_region.remove_css_class(READONLY_CSS_CLASS);
        } else {
            self.thermal_region.add_css_class(READONLY_CSS_CLASS);
        }
    }
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
    fn constructed(&self) {
        self.parent_constructed();

        self.set_enable_voronoi(false);
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
impl WidgetImpl for FrameMill {}
impl BoxImpl for FrameMill {}
