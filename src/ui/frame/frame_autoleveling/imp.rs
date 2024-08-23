#![allow(unreachable_code)]

use gtk::{glib, prelude::*, subclass::prelude::*};

use crate::ui::object::{entry_object::EntryObject, spin_button_object::SpinButtonObject};

#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/frame/frame_autoleveling.ui")]
#[properties(wrapper_type=super::FrameAutoleveling)]
pub struct FrameAutoleveling {}

impl FrameAutoleveling {}

#[glib::object_subclass]
impl ObjectSubclass for FrameAutoleveling {
    const NAME: &'static str = "FrameAutoleveling";
    type Type = super::FrameAutoleveling;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        SpinButtonObject::ensure_type();
        EntryObject::ensure_type();

        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for FrameAutoleveling {
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
impl WidgetImpl for FrameAutoleveling {}
impl BoxImpl for FrameAutoleveling {}
