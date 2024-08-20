#![allow(unreachable_code)]

use std::{cell::RefCell, cmp::min};

use gtk::{gio, glib, prelude::*, subclass::prelude::*};

// Object holding the state
#[derive(Default, gtk::CompositeTemplate, glib::Properties)]
#[template(resource = "/com/github/jnthbdn/rs-pcb2gcode-gui/templates/browse_file_object.ui")]
#[properties(wrapper_type = super::BrowseFileObject)]
pub struct BrowseFileObject {
    #[template_child]
    pub entry: TemplateChild<gtk::Entry>,

    #[property(name="title", set = Self::set_dialog_title, get = Self::get_dialog_title)]
    _dialog_title: RefCell<String>,

    #[property(set = Self::set_file_patterns, get)]
    file_pattern: RefCell<Vec<String>>,

    #[property(set = Self::set_file_patterns_name, get)]
    file_pattern_name: RefCell<Vec<String>>,

    dialog: RefCell<gtk::FileDialog>,
}

impl BrowseFileObject {
    pub fn open_dialog(&self) {
        let clone_self = self.obj().clone();
        self.dialog.borrow().open(
            Some(
                &self
                    .obj()
                    .ancestor(gtk::Window::static_type())
                    .and_downcast::<gtk::Window>()
                    .unwrap(),
            ),
            None::<&gio::Cancellable>,
            move |result| match result {
                Ok(file) => clone_self
                    .imp()
                    .entry
                    .set_text(file.path().unwrap().to_str().unwrap()),
                Err(e) => log::warn!("{}", e),
            },
        );
    }
    fn get_dialog_title(&self) -> String {
        self.dialog.borrow().title().into()
    }

    fn set_dialog_title(&self, title: String) {
        self.dialog.borrow().set_title(title.as_str());
    }

    fn set_file_patterns_name(&self, names: Vec<String>) {
        self.file_pattern_name
            .set(Self::trim_and_remove_empty(&names));
        self.update_patterns();
    }

    fn set_file_patterns(&self, patterns: Vec<String>) {
        self.file_pattern
            .set(Self::trim_and_remove_empty(&patterns));
        self.update_patterns();
    }

    fn trim_and_remove_empty(strings: &Vec<String>) -> Vec<String> {
        strings
            .iter()
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    }

    fn update_patterns(&self) {
        let list = gio::ListStore::new::<gtk::FileFilter>();
        let pattern = self.file_pattern.borrow();
        let pattern_name = self.file_pattern_name.borrow();

        let len = min(pattern.len(), pattern_name.len());

        for i in 0..len {
            let filter = gtk::FileFilter::new();
            filter.set_name(Some(
                format!("{} ({})", pattern_name[i], pattern[i]).as_str(),
            ));

            for pattern in pattern[i].split(',') {
                filter.add_pattern(pattern.trim());
            }

            list.append(&filter);
        }

        self.dialog.borrow().set_filters(Some(&list));
    }
}

#[glib::object_subclass]
impl ObjectSubclass for BrowseFileObject {
    const NAME: &'static str = "BrowseFileObject";
    type Type = super::BrowseFileObject;
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
impl ObjectImpl for BrowseFileObject {
    fn constructed(&self) {
        self.parent_constructed();

        self.dialog.borrow().set_modal(true);
    }
}
impl WidgetImpl for BrowseFileObject {}
impl BoxImpl for BrowseFileObject {}
