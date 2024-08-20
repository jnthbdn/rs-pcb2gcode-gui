#![allow(unreachable_code)]
use std::{cell::Cell, str::FromStr, sync::OnceLock};

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::database::database::DatabaseColumn;
use crate::tools::{basetool::BaseTool, drill::Drill, endmill::Endmill, vbit::VBit, ToolType};
use crate::ui::custom_object::entry_object::EntryObject;
use crate::ui::custom_object::spin_button_object::SpinButtonObject;
use crate::ui::custom_object::textview_object::TextViewObject;

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/tool_setting_object.ui")]
#[properties(wrapper_type=super::ToolSettingObject)]
pub struct ToolSettingObject {
    #[template_child]
    general_id: TemplateChild<gtk::Entry>,
    #[template_child]
    general_name: TemplateChild<EntryObject>,
    #[template_child]
    general_note: TemplateChild<TextViewObject>,

    #[template_child]
    diameter_shaft: TemplateChild<SpinButtonObject>,
    #[template_child]
    diameter_tool: TemplateChild<SpinButtonObject>,
    #[template_child]
    diameter_tip_label: TemplateChild<gtk::Label>,
    #[template_child]
    diameter_tip: TemplateChild<SpinButtonObject>,
    #[template_child]
    diameter_angle_label: TemplateChild<gtk::Label>,
    #[template_child]
    diameter_angle: TemplateChild<SpinButtonObject>,

    #[template_child]
    pass_depth: TemplateChild<SpinButtonObject>,
    #[template_child]
    speed_spindle: TemplateChild<SpinButtonObject>,
    #[template_child]
    speed_vertical: TemplateChild<SpinButtonObject>,
    #[template_child]
    speed_horizontal: TemplateChild<SpinButtonObject>,

    current_tool: Cell<Option<ToolType>>,
}

impl ToolSettingObject {
    fn show_base_tool(&self, base_tool: &BaseTool) {
        self.general_id.set_text(&base_tool.id.to_string());
        self.general_name.set_text(&base_tool.name);
        self.general_note.set_text(&base_tool.note);

        self.diameter_shaft.set_value(base_tool.shaft_diameter);
        self.diameter_tool.set_value(base_tool.tool_diameter);

        self.pass_depth.set_value(base_tool.pass_depth);
        self.speed_spindle.set_value(base_tool.spindle_speed);
        self.speed_vertical.set_value(base_tool.plunge_rate);
        self.speed_horizontal.set_value(base_tool.feed_rate);
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

    pub fn current_tooltype(&self) -> Option<ToolType> {
        self.current_tool.get()
    }

    pub fn current_id(&self) -> u32 {
        u32::from_str(&self.general_id.text()).unwrap_or(0)
    }

    pub fn check_setting_change(&self) {
        self.general_name.grab_focus();

        self.general_name.imp().check_change();
        self.general_note.imp().check_change();
        self.diameter_shaft.imp().check_change();
        self.diameter_tool.imp().check_change();
        self.pass_depth.imp().check_change();
        self.speed_spindle.imp().check_change();
        self.speed_vertical.imp().check_change();
        self.speed_horizontal.imp().check_change();

        match self.current_tool.get() {
            Some(tool) => match tool {
                ToolType::VBit => {
                    self.diameter_tip.imp().check_change();
                    self.diameter_angle.imp().check_change();
                }
                _ => (),
            },
            None => (),
        }
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
        EntryObject::ensure_type();
        TextViewObject::ensure_type();
        SpinButtonObject::ensure_type();

        self.general_id.add_css_class("read-only");
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("setting-changed")
                .param_types([
                    ToolType::static_type(),
                    DatabaseColumn::static_type(),
                    glib::GString::static_type(),
                    u32::static_type(),
                ])
                .build()]
        })
    }
}
impl WidgetImpl for ToolSettingObject {}
impl BoxImpl for ToolSettingObject {}
