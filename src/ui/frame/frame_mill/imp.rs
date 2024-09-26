#![allow(unreachable_code)]
use std::{
    cell::{Cell, RefCell},
    sync::{Arc, Mutex},
};

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::{
    database::database::Database,
    ui::{
        object::{
            browse_file_object::BrowseFileObject, info_tooltip_object::InfoToolTipObject,
            select_tool_object::SelectToolObject, spin_button_object::SpinButtonObject,
            textview_object::TextViewObject,
        },
        READONLY_CSS_CLASS,
    },
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
    pub isolation_width_tool: TemplateChild<gtk::CheckButton>,

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

    database: RefCell<Option<Arc<Mutex<Database>>>>,
}

impl FrameMill {
    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.database.set(Some(db.clone()));
        self.mill_tool
            .set_database(db.clone(), self.is_unit_metric.get());
    }
    pub fn set_enable_voronoi(&self, is_enable: bool) {
        self.thermal_region.set_can_target(is_enable);

        if is_enable {
            self.thermal_region.remove_css_class(READONLY_CSS_CLASS);
        } else {
            self.thermal_region.add_css_class(READONLY_CSS_CLASS);
        }
    }

    pub fn set_enable_isloation_tool_width(&self, is_enable: bool) {
        self.isolation.set_can_target(!is_enable);

        if is_enable {
            self.isolation.add_css_class(READONLY_CSS_CLASS);
            self.set_isolation_with_tool_diameter();
        } else {
            self.isolation.remove_css_class(READONLY_CSS_CLASS);
        }
    }

    pub fn set_isolation_with_tool_diameter(&self) {
        if self.database.borrow().is_none() {
            log::warn!("No database. Skip.");
            return;
        }

        log::debug!("Lock database.");
        let db = self.database.borrow();
        let db = db.as_ref().unwrap().lock().unwrap();

        match self.mill_tool.get_selected() {
            Some(tool) => {
                if tool.get_tool_type().is_some() && tool.get_tool_id().is_some() {
                    let diameter = match tool.get_tool_type().unwrap() {
                        crate::tools::ToolType::Drill => 0.0,
                        crate::tools::ToolType::Endmill => {
                            db.get_endmill(tool.get_tool_id().unwrap())
                                .unwrap()
                                .unwrap()
                                .base_tool
                                .tool_diameter
                        }
                        crate::tools::ToolType::VBit => db
                            .get_vbit(tool.get_tool_id().unwrap())
                            .unwrap()
                            .unwrap()
                            .diameter(self.depth.value()),
                    };
                    self.isolation.set_value(diameter);
                }
            }
            None => (),
        };

        log::debug!("Drop lock database.");
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
}
impl WidgetImpl for FrameMill {}
impl BoxImpl for FrameMill {}
