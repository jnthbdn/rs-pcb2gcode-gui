#![allow(unreachable_code)]
use std::{cell::Cell, str::FromStr, sync::OnceLock};

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::database::database::DatabaseColumn;
use crate::tools::{basetool::BaseTool, drill::Drill, endmill::Endmill, vbit::VBit, ToolType};
use crate::ui::object::entry_object::EntryObject;
use crate::ui::object::spin_button_object::SpinButtonObject;
use crate::ui::object::textview_object::TextViewObject;

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(
    resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_tool_settings.ui"
)]
#[properties(wrapper_type=super::FrameToolSettings)]
pub struct FrameToolSettings {
    #[template_child]
    pub general_id: TemplateChild<gtk::Entry>,
    #[template_child]
    pub general_name: TemplateChild<EntryObject>,
    #[template_child]
    pub general_note: TemplateChild<TextViewObject>,

    #[template_child]
    pub diameter_shaft: TemplateChild<SpinButtonObject>,
    #[template_child]
    pub diameter_tool: TemplateChild<SpinButtonObject>,
    #[template_child]
    pub diameter_tip_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub diameter_tip: TemplateChild<SpinButtonObject>,
    #[template_child]
    pub tool_angle_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub tool_angle: TemplateChild<SpinButtonObject>,

    #[template_child]
    pub pass_depth: TemplateChild<SpinButtonObject>,
    #[template_child]
    pub speed_spindle: TemplateChild<SpinButtonObject>,
    #[template_child]
    pub speed_vertical: TemplateChild<SpinButtonObject>,
    #[template_child]
    pub speed_horizontal_label: TemplateChild<gtk::Label>,
    #[template_child]
    pub speed_horizontal: TemplateChild<SpinButtonObject>,

    current_tool: Cell<Option<ToolType>>,
}

impl FrameToolSettings {
    fn show_base_tool(&self, base_tool: &BaseTool) {
        self.general_id.set_text(&base_tool.id.to_string());
        self.general_name.set_text(&base_tool.name);
        self.general_note.set_text(&base_tool.note);

        self.diameter_shaft.init_value(base_tool.shaft_diameter);
        self.diameter_tool.init_value(base_tool.tool_diameter);

        self.pass_depth.init_value(base_tool.pass_depth);
        self.speed_spindle.init_value(base_tool.spindle_speed);
        self.speed_vertical.init_value(base_tool.plunge_rate);
    }

    pub fn show_endmill(&self, endmill: &Endmill) {
        self.current_tool.set(Some(ToolType::Endmill));
        self.show_base_tool(&endmill.base_tool);
        self.speed_horizontal.init_value(endmill.feed_rate);

        self.tool_angle_label.set_visible(false);
        self.tool_angle.set_visible(false);
        self.diameter_tip_label.set_visible(false);
        self.diameter_tip.set_visible(false);
        self.speed_horizontal.set_visible(true);
        self.speed_horizontal_label.set_visible(true);
    }

    pub fn show_drill(&self, drill: &Drill) {
        self.current_tool.set(Some(ToolType::Drill));
        self.show_base_tool(&drill.base_tool);

        self.tool_angle_label.set_visible(false);
        self.tool_angle.set_visible(false);
        self.diameter_tip_label.set_visible(false);
        self.diameter_tip.set_visible(false);
        self.speed_horizontal.set_visible(false);
        self.speed_horizontal_label.set_visible(false);
    }

    pub fn show_vbit(&self, vbit: &VBit) {
        self.current_tool.set(Some(ToolType::VBit));
        self.show_base_tool(&vbit.base_tool);

        self.speed_horizontal.init_value(vbit.feed_rate);
        self.tool_angle.init_value(vbit.tool_angle);
        self.diameter_tip.init_value(vbit.tip_diameter);

        self.tool_angle_label.set_visible(true);
        self.tool_angle.set_visible(true);
        self.diameter_tip_label.set_visible(true);
        self.diameter_tip.set_visible(true);
        self.speed_horizontal.set_visible(true);
        self.speed_horizontal_label.set_visible(true);
    }

    pub fn current_tooltype(&self) -> Option<ToolType> {
        self.current_tool.get()
    }

    pub fn current_id(&self) -> u32 {
        u32::from_str(&self.general_id.text()).unwrap_or(0)
    }

    pub fn check_setting_change(&self) {
        self.general_name.grab_focus();
        self.general_note.grab_focus();
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FrameToolSettings {
    const NAME: &'static str = "FrameToolSettings";
    type Type = super::FrameToolSettings;
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
impl ObjectImpl for FrameToolSettings {
    fn constructed(&self) {
        self.parent_constructed();
        EntryObject::ensure_type();
        TextViewObject::ensure_type();
        SpinButtonObject::ensure_type();
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
impl WidgetImpl for FrameToolSettings {}
impl BoxImpl for FrameToolSettings {}
