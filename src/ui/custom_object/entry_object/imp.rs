#![allow(unreachable_code)]

use std::cell::RefCell;
use std::sync::OnceLock;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::EntryObject)]
pub struct EntryObject {
    pub old_text: RefCell<glib::GString>,
}

impl EntryObject {
    fn get_text_child(&self) -> Option<gtk::Text> {
        let mut child = self.obj().first_child();

        while child.is_some() {
            let widget = child.unwrap();
            if widget.downcast_ref::<gtk::Text>().is_some() {
                return Some(widget.downcast::<gtk::Text>().unwrap());
            }

            child = widget.next_sibling();
        }

        None
    }

    pub fn check_change(&self) {
        let current_text = self.obj().text();

        if !self.old_text.borrow().eq(&current_text) {
            self.old_text.set(current_text);
            self.obj()
                .emit_by_name::<()>("value-changed", &[&self.obj().to_value()]);
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for EntryObject {
    const NAME: &'static str = "EntryObject";
    type Type = super::EntryObject;
    type ParentType = gtk::Entry;
}

#[glib::derived_properties]
impl ObjectImpl for EntryObject {
    fn constructed(&self) {
        self.parent_constructed();

        self.old_text.set(self.obj().text());

        self.get_text_child()
            .unwrap()
            .connect_notify(Some("has-focus"), |text, _param| {
                if !text.has_focus() {
                    text.ancestor(EntryObject::type_())
                        .unwrap()
                        .downcast_ref::<super::EntryObject>()
                        .unwrap()
                        .imp()
                        .check_change();
                }
            });
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("value-changed")
                .param_types([super::EntryObject::static_type()])
                .build()]
        })
    }
}
impl WidgetImpl for EntryObject {}
impl EntryImpl for EntryObject {}
