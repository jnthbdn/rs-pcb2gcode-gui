#![allow(unreachable_code)]

use std::cell::RefCell;

use gtk::{glib, prelude::*, subclass::prelude::*, CompositeTemplate};

// Object holding the state
#[derive(Default, glib::Properties, CompositeTemplate)]
#[template(
    resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/object/info_tooltip_object.ui"
)]
#[properties(wrapper_type = super::InfoToolTipObject)]
pub struct InfoToolTipObject {
    #[template_child]
    image: TemplateChild<gtk::Image>,

    #[template_child]
    pub popover: TemplateChild<gtk::Popover>,

    #[template_child]
    label: TemplateChild<gtk::Label>,

    #[template_child]
    link_button: TemplateChild<gtk::LinkButton>,

    #[property(set, get)]
    text_markup: RefCell<String>,

    #[property(set = Self::set_link, get, nullable)]
    link: RefCell<Option<String>>,
}

impl InfoToolTipObject {
    fn set_link(&self, link: Option<String>) {
        self.link_button.set_visible(link.is_some());
        self.link.set(link);
    }
}

#[glib::object_subclass]
impl ObjectSubclass for InfoToolTipObject {
    const NAME: &'static str = "InfoToolTipObject";
    type Type = super::InfoToolTipObject;
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
impl ObjectImpl for InfoToolTipObject {
    fn constructed(&self) {
        self.parent_constructed();

        self.obj()
            .bind_property::<gtk::Label>("text_markup", &self.label, "label")
            .build();

        self.obj()
            .bind_property::<gtk::LinkButton>("link", &self.link_button, "uri")
            .build();
    }
}
impl WidgetImpl for InfoToolTipObject {}
impl BoxImpl for InfoToolTipObject {}
