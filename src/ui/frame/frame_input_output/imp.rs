#![allow(unreachable_code)]
use std::{cell::Cell, str::FromStr};

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::object::{
    browse_file_object::BrowseFileObject, info_tooltip_object::InfoToolTipObject,
};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_input_output.ui")]
#[properties(wrapper_type=super::FrameInputOutput)]
pub struct FrameInputOutput {
    #[template_child]
    pub front_file: TemplateChild<BrowseFileObject>,

    #[template_child]
    pub back_file: TemplateChild<BrowseFileObject>,

    #[template_child]
    pub drill_file: TemplateChild<BrowseFileObject>,

    #[template_child]
    pub outline_file: TemplateChild<BrowseFileObject>,

    #[template_child]
    pub preamble_text_file: TemplateChild<BrowseFileObject>,

    #[template_child]
    pub preamble_file: TemplateChild<BrowseFileObject>,

    #[template_child]
    pub postamble_file: TemplateChild<BrowseFileObject>,

    #[template_child]
    pub output_folder: TemplateChild<BrowseFileObject>,

    last_folder: Cell<Option<String>>,
}

impl FrameInputOutput {
    pub fn set_default_openning_folder(&self, path: &str) {
        let path = String::from_str(path).unwrap();

        self.front_file.set_default_folder(&path);
        self.back_file.set_default_folder(&path);
        self.drill_file.set_default_folder(&path);
        self.outline_file.set_default_folder(&path);
        self.preamble_text_file.set_default_folder(&path);
        self.preamble_file.set_default_folder(&path);
        self.postamble_file.set_default_folder(&path);
        self.output_folder.set_default_folder(&path);

        self.last_folder.set(Some(path));
    }

    pub fn get_default_folder(&self) -> Option<String> {
        let result = self.last_folder.take();
        self.last_folder.set(result.clone());
        result
    }

    pub fn set_default_folder(&self, value: Option<String>) {
        self.last_folder.set(value.clone());

        match value {
            Some(folder) => self.set_default_openning_folder(&folder),
            None => (),
        };
    }
}

#[glib::object_subclass]
impl ObjectSubclass for FrameInputOutput {
    const NAME: &'static str = "FrameInputOutput";
    type Type = super::FrameInputOutput;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        BrowseFileObject::ensure_type();
        InfoToolTipObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for FrameInputOutput {}
impl WidgetImpl for FrameInputOutput {}
impl BoxImpl for FrameInputOutput {}
