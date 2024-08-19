#![allow(unreachable_code)]

use std::str::FromStr;
use std::{cell::Cell, sync::OnceLock};

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::tools::{basetool::BaseTool, drill::Drill, endmill::Endmill, vbit::VBit, ToolType};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/tool_setting_object.ui")]
#[properties(wrapper_type=super::ToolSettingObject)]
pub struct ToolSettingObject {
    #[template_child]
    general_id: TemplateChild<gtk::Entry>,
    #[template_child]
    general_name: TemplateChild<gtk::Entry>,
    #[template_child]
    general_note: TemplateChild<gtk::TextView>,

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

    current_tool: Cell<Option<ToolType>>,
}

impl ToolSettingObject {
    fn show_base_tool(&self, base_tool: &BaseTool) {
        self.general_id.set_text(&base_tool.id.to_string());
        self.general_name.set_text(&base_tool.name);
        self.general_note.buffer().set_text(&base_tool.note);

        self.diameter_shaft.set_value(base_tool.shaft_diameter);
        self.diameter_tool.set_value(base_tool.tool_diameter);

        self.pass_depth.set_value(base_tool.pass_depth);
        self.speed_spindle.set_value(base_tool.spindle_speed);
        self.speed_vertical.set_value(base_tool.plunge_rate);
        self.speed_horizontal.set_value(base_tool.feed_rate);
    }

    fn get_text_child(entry: &gtk::Entry) -> Option<gtk::Text> {
        let mut child = entry.first_child();

        while child.is_some() {
            let widget = child.unwrap();
            if widget.downcast_ref::<gtk::Text>().is_some() {
                return Some(widget.downcast::<gtk::Text>().unwrap());
            }

            child = widget.next_sibling();
        }

        None
    }

    pub fn show_endmill(&self, endmill: &Endmill) {
        self.current_tool.set(Some(ToolType::Endmill));
        self.show_base_tool(&endmill.base_tool);

        self.diameter_angle_label.set_visible(false);
        self.diameter_angle.set_visible(false);
        self.diameter_tip_label.set_visible(false);
        self.diameter_tip.set_visible(false);
    }

    pub fn show_drill(&self, drill: &Drill) {
        self.current_tool.set(Some(ToolType::Drill));
        self.show_base_tool(&drill.base_tool);

        self.diameter_angle_label.set_visible(false);
        self.diameter_angle.set_visible(false);
        self.diameter_tip_label.set_visible(false);
        self.diameter_tip.set_visible(false);
    }

    pub fn show_vbit(&self, vbit: &VBit) {
        self.current_tool.set(Some(ToolType::Endmill));
        self.show_base_tool(&vbit.base_tool);

        self.diameter_angle_label.set_visible(true);
        self.diameter_angle.set_visible(true);
        self.diameter_tip_label.set_visible(true);
        self.diameter_tip.set_visible(true);

        self.diameter_angle.set_value(vbit.tool_angle);
        self.diameter_tip.set_value(vbit.tip_diameter);
    }
}

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
impl ObjectImpl for ToolSettingObject {
    fn constructed(&self) {
        self.parent_constructed();

        Self::get_text_child(&self.general_name)
            .unwrap()
            .connect_notify(Some("has-focus"), |entry, _param| {
                entry
                    .ancestor(ToolSettingObject::type_())
                    .unwrap()
                    .emit_by_name::<()>("setting-changed", &[]);
            });
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| vec![glib::subclass::Signal::builder("setting-changed").build()])
    }
}
impl WidgetImpl for ToolSettingObject {}
impl BoxImpl for ToolSettingObject {}
