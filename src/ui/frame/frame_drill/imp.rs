#![allow(unreachable_code)]

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::{
    object::{
        info_tooltip_object::InfoToolTipObject, select_tool_object::SelectToolObject,
        spin_button_object::SpinButtonObject,
    },
    READONLY_CSS_CLASS,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_drill.ui")]
#[properties(wrapper_type=super::FrameDrill)]
pub struct FrameDrill {
    #[template_child]
    pub drill_tool: TemplateChild<SelectToolObject>,

    #[template_child]
    pub depth: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub side: TemplateChild<gtk::DropDown>,

    #[template_child]
    pub enable_milldrilling: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub milldrilling_tool: TemplateChild<SelectToolObject>,

    #[template_child]
    pub milldrilling_min_diameter: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub milldrilling_depth: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub no_g91_1: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub no_g81: TemplateChild<gtk::CheckButton>,
}

impl FrameDrill {
    pub fn set_enable_milldrilling(&self, is_enable: bool) {
        self.milldrilling_tool.set_can_target(is_enable);
        self.milldrilling_min_diameter.set_can_target(is_enable);
        self.milldrilling_depth.set_can_target(is_enable);

        if is_enable {
            self.milldrilling_tool.remove_css_class(READONLY_CSS_CLASS);
            self.milldrilling_min_diameter
                .remove_css_class(READONLY_CSS_CLASS);
            self.milldrilling_depth.remove_css_class(READONLY_CSS_CLASS);
        } else {
            self.milldrilling_tool.add_css_class(READONLY_CSS_CLASS);
            self.milldrilling_min_diameter
                .add_css_class(READONLY_CSS_CLASS);
            self.milldrilling_depth.add_css_class(READONLY_CSS_CLASS);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FrameDrill {
    const NAME: &'static str = "FrameDrill";
    type Type = super::FrameDrill;
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
impl ObjectImpl for FrameDrill {
    fn constructed(&self) {
        self.parent_constructed();

        self.set_enable_milldrilling(false);
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
impl WidgetImpl for FrameDrill {}
impl BoxImpl for FrameDrill {}
