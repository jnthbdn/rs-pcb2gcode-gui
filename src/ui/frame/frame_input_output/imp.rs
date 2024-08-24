#![allow(unreachable_code)]
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
}

impl FrameInputOutput {}

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
impl ObjectImpl for FrameInputOutput {
    // fn constructed(&self) {
    //     self.parent_constructed();

    // }

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
impl WidgetImpl for FrameInputOutput {}
impl BoxImpl for FrameInputOutput {}
