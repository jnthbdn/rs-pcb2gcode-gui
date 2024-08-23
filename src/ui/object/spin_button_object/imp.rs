#![allow(unreachable_code)]

use std::cell::{Cell, RefCell};
use std::sync::OnceLock;

use gtk::{glib, prelude::*, subclass::prelude::*, CompositeTemplate};

// Object holding the state
#[derive(Default, glib::Properties, CompositeTemplate)]
#[template(
    resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/object/spin_button_object.ui"
)]
#[properties(wrapper_type = super::SpinButtonObject)]
pub struct SpinButtonObject {
    #[template_child]
    spin_button: TemplateChild<gtk::SpinButton>,
    #[template_child]
    label_postfix: TemplateChild<gtk::Label>,

    #[property(set = Self::set_digits, get)]
    digits: Cell<u32>,
    #[property(set = Self::set_min, get)]
    min: Cell<f64>,
    #[property(set = Self::set_max, get)]
    max: Cell<f64>,
    #[property(set = Self::set_step, get)]
    step: Cell<f64>,
    #[property(set = Self::set_value, get = Self::get_value)]
    _value: Cell<f64>,
    #[property(set, get)]
    postfix: RefCell<String>,

    pub old_value: Cell<f64>,
}

impl SpinButtonObject {
    pub fn value_str(&self) -> String {
        format!(
            "{:.*}",
            self.digits.get() as usize,
            self.spin_button.value()
        )
    }

    pub fn set_value(&self, value: f64) {
        self.old_value.set(value);
        self.spin_button.set_value(value);
    }

    pub fn get_value(&self) -> f64 {
        self.spin_button.value()
    }

    pub fn set_digits(&self, value: u32) {
        self.spin_button.set_digits(value.clamp(0, 10));
        self.digits.set(value.clamp(0, 10));
    }

    pub fn set_min(&self, value: f64) {
        self.spin_button.adjustment().set_lower(value);
        self.min.set(value);
    }

    pub fn set_max(&self, value: f64) {
        self.spin_button.adjustment().set_upper(value);
        self.max.set(value);
    }

    pub fn set_step(&self, value: f64) {
        self.spin_button.adjustment().set_step_increment(value);
        self.step.set(value);
    }

    fn get_text_child(&self) -> Option<gtk::Text> {
        let mut child = self.spin_button.first_child();

        while child.is_some() {
            let widget = child.unwrap();
            if widget.downcast_ref::<gtk::Text>().is_some() {
                return Some(widget.downcast::<gtk::Text>().unwrap());
            }

            child = widget.next_sibling();
        }

        None
    }

    pub fn has_focus(&self) -> bool {
        let text_child = self.get_text_child();

        text_child.unwrap().has_focus()
    }

    pub fn check_change(&self) {
        let current_value = self.spin_button.value();

        if !self.old_value.get().eq(&current_value) {
            self.old_value.set(current_value);
            self.obj()
                .emit_by_name::<()>("value-changed", &[&self.obj().to_value()]);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for SpinButtonObject {
    const NAME: &'static str = "SpinButtonObject";
    type Type = super::SpinButtonObject;
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
impl ObjectImpl for SpinButtonObject {
    fn constructed(&self) {
        self.parent_constructed();

        self.old_value.set(self.spin_button.value());

        let clone_self = self.obj().clone();
        self.get_text_child()
            .unwrap()
            .connect_has_focus_notify(move |text| {
                if !text.has_focus() {
                    clone_self.imp().check_change();
                }
            });

        let clone_self = self.obj().clone();
        self.spin_button.connect_value_changed(move |_spin| {
            if !clone_self.imp().has_focus() {
                clone_self.imp().check_change();
            }
        });

        self.obj()
            .bind_property::<gtk::Label>("postfix", self.label_postfix.as_ref(), "label")
            .bidirectional()
            .build();
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("value-changed")
                .param_types([super::SpinButtonObject::static_type()])
                .build()]
        })
    }
}
impl WidgetImpl for SpinButtonObject {}
impl BoxImpl for SpinButtonObject {}
