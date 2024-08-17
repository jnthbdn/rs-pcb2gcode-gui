#![allow(unreachable_code)]

use gtk::{glib, subclass::prelude::*};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/tool_setting_object.ui")]
#[properties(wrapper_type=super::ToolSettingObject)]
pub struct ToolSettingObject {
    #[template_child]
    general_id: TemplateChild<gtk::Entry>,
    #[template_child]
    general_name: TemplateChild<gtk::Entry>,

    #[template_child]
    diameter_shaft: TemplateChild<gtk::SpinButton>,
    #[template_child]
    diameter_tool: TemplateChild<gtk::SpinButton>,
    #[template_child]
    diameter_tip_label: TemplateChild<gtk::Label>,
    #[template_child]
    diameter_tip: TemplateChild<gtk::SpinButton>,
    #[template_child]
    diameter_angle_label: TemplateChild<gtk::Label>,
    #[template_child]
    diameter_angle: TemplateChild<gtk::SpinButton>,

    #[template_child]
    pass_depth: TemplateChild<gtk::SpinButton>,
    #[template_child]
    speed_spindle: TemplateChild<gtk::SpinButton>,
    #[template_child]
    speed_vertical: TemplateChild<gtk::SpinButton>,
    #[template_child]
    speed_horizontal: TemplateChild<gtk::SpinButton>,
}

impl ToolSettingObject {}

#[glib::object_subclass]
impl ObjectSubclass for ToolSettingObject {
    const NAME: &'static str = "ToolSettingObject";
    type Type = super::ToolSettingObject;
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
impl ObjectImpl for ToolSettingObject {}
impl WidgetImpl for ToolSettingObject {}
impl BoxImpl for ToolSettingObject {}
