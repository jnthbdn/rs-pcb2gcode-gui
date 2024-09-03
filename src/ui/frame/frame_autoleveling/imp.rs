#![allow(unreachable_code)]

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::{
    object::{
        entry_object::EntryObject, info_tooltip_object::InfoToolTipObject,
        spin_button_object::SpinButtonObject,
    },
    READONLY_CSS_CLASS,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_autoleveling.ui")]
#[properties(wrapper_type=super::FrameAutoleveling)]
pub struct FrameAutoleveling {
    #[template_child]
    pub enable_front: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub enable_back: TemplateChild<gtk::CheckButton>,

    #[template_child]
    pub software: TemplateChild<gtk::DropDown>,

    #[template_child]
    pub distance_probe_x: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub distance_probe_y: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub feed: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub probe_on: TemplateChild<EntryObject>,

    #[template_child]
    pub probe_off: TemplateChild<EntryObject>,

    #[template_child]
    pub probe_code: TemplateChild<EntryObject>,

    #[template_child]
    pub probe_variable: TemplateChild<EntryObject>,

    #[template_child]
    pub probe_set_zero: TemplateChild<EntryObject>,
}

impl FrameAutoleveling {
    pub fn set_autolevel_enable(&self, is_enable: bool) {
        self.software.set_can_target(is_enable);
        self.distance_probe_x.set_can_target(is_enable);
        self.distance_probe_y.set_can_target(is_enable);
        self.feed.set_can_target(is_enable);
        self.probe_on.set_can_target(is_enable);
        self.probe_off.set_can_target(is_enable);
        self.probe_code.set_can_target(is_enable);
        self.probe_variable.set_can_target(is_enable);
        self.probe_set_zero.set_can_target(is_enable);

        if is_enable {
            self.software.remove_css_class(READONLY_CSS_CLASS);
            self.distance_probe_x.remove_css_class(READONLY_CSS_CLASS);
            self.distance_probe_y.remove_css_class(READONLY_CSS_CLASS);
            self.feed.remove_css_class(READONLY_CSS_CLASS);
            self.probe_on.remove_css_class(READONLY_CSS_CLASS);
            self.probe_off.remove_css_class(READONLY_CSS_CLASS);
            self.probe_code.remove_css_class(READONLY_CSS_CLASS);
            self.probe_variable.remove_css_class(READONLY_CSS_CLASS);
            self.probe_set_zero.remove_css_class(READONLY_CSS_CLASS);
        } else {
            self.software.add_css_class(READONLY_CSS_CLASS);
            self.distance_probe_x.add_css_class(READONLY_CSS_CLASS);
            self.distance_probe_y.add_css_class(READONLY_CSS_CLASS);
            self.feed.add_css_class(READONLY_CSS_CLASS);
            self.probe_on.add_css_class(READONLY_CSS_CLASS);
            self.probe_off.add_css_class(READONLY_CSS_CLASS);
            self.probe_code.add_css_class(READONLY_CSS_CLASS);
            self.probe_variable.add_css_class(READONLY_CSS_CLASS);
            self.probe_set_zero.add_css_class(READONLY_CSS_CLASS);
        }
    }
    pub fn on_software_change(&self) {
        let is_custom = self.software.selected() == 3;

        self.probe_code.set_can_target(is_custom);
        self.probe_variable.set_can_target(is_custom);
        self.probe_set_zero.set_can_target(is_custom);

        if is_custom {
            self.probe_code.remove_css_class(READONLY_CSS_CLASS);
            self.probe_variable.remove_css_class(READONLY_CSS_CLASS);
            self.probe_set_zero.remove_css_class(READONLY_CSS_CLASS);
        } else {
            self.probe_code.add_css_class(READONLY_CSS_CLASS);
            self.probe_variable.add_css_class(READONLY_CSS_CLASS);
            self.probe_set_zero.add_css_class(READONLY_CSS_CLASS);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FrameAutoleveling {
    const NAME: &'static str = "FrameAutoleveling";
    type Type = super::FrameAutoleveling;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        SpinButtonObject::ensure_type();
        EntryObject::ensure_type();
        InfoToolTipObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for FrameAutoleveling {
    fn constructed(&self) {
        self.parent_constructed();

        self.set_autolevel_enable(false);

        let obj = self.obj().clone();
        self.software
            .connect_notify_local(Some("selected"), move |_, _| {
                obj.imp().on_software_change();
            });
    }
}
impl WidgetImpl for FrameAutoleveling {}
impl BoxImpl for FrameAutoleveling {}
