#![allow(unreachable_code)]
use std::cell::Cell;

use gtk::glib;
use gtk::subclass::prelude::*;

use crate::tools::ToolType;

// Object holding the state
#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/object/db_label_object.ui")]
#[properties(wrapper_type = super::DBLabelObject)]
pub struct DBLabelObject {
    pub db_id: Cell<u32>,
    pub tool_type: Cell<Option<ToolType>>,

    #[template_child]
    pub label: TemplateChild<gtk::Label>,
}

impl DBLabelObject {}

#[glib::object_subclass]
impl ObjectSubclass for DBLabelObject {
    const NAME: &'static str = "DBLabelObject";
    type Type = super::DBLabelObject;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for DBLabelObject {}
impl WidgetImpl for DBLabelObject {}
impl BoxImpl for DBLabelObject {}
