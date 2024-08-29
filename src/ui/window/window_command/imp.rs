#![allow(unreachable_code)]

use gtk::{glib, subclass::prelude::*};

#[derive(gtk::CompositeTemplate, glib::Properties, Default)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/window/window_command.ui")]
#[properties(wrapper_type=super::WindowCommand)]
pub struct WindowCommand {
    #[template_child]
    pub textview: TemplateChild<gtk::TextView>,
}

impl WindowCommand {}

#[glib::object_subclass]
impl ObjectSubclass for WindowCommand {
    const NAME: &'static str = "WindowCommand";
    type Type = super::WindowCommand;
    type ParentType = gtk::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_instance_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for WindowCommand {}
impl WidgetImpl for WindowCommand {}
impl WindowImpl for WindowCommand {}
impl ApplicationWindowImpl for WindowCommand {}
