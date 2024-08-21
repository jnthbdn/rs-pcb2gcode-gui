#![allow(unreachable_code)]

use std::cell::RefCell;
use std::sync::OnceLock;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::TextViewObject)]
pub struct TextViewObject {
    pub old_text: RefCell<glib::GString>,
}

impl TextViewObject {
    pub fn check_change(&self) {
        let current_text = self.obj().all_text();

        if !self.old_text.borrow().eq(&current_text) {
            self.old_text.set(current_text);
            self.obj()
                .emit_by_name::<()>("value-changed", &[&self.obj().to_value()]);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for TextViewObject {
    const NAME: &'static str = "TextViewObject";
    type Type = super::TextViewObject;
    type ParentType = gtk::TextView;
}

#[glib::derived_properties]
impl ObjectImpl for TextViewObject {
    fn constructed(&self) {
        self.parent_constructed();

        self.old_text.set(self.obj().all_text());

        self.obj()
            .connect_notify(Some("has-focus"), |text, _param| {
                if !text.has_focus() {
                    text.imp().check_change();
                }
            });
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("value-changed")
                .param_types([super::TextViewObject::static_type()])
                .build()]
        })
    }
}
impl WidgetImpl for TextViewObject {}
impl TextViewImpl for TextViewObject {}
