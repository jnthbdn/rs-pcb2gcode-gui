mod imp;

use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct WindowExecute(ObjectSubclass<imp::WindowExecute>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

#[gtk::template_callbacks]
impl WindowExecute {
    pub fn new(parent: &gtk::Window) -> Self {
        glib::Object::builder()
            .property("transient-for", parent)
            .build()
    }

    pub fn open(&self, params: String) {
        self.present();
        self.imp().run(params);
    }

    #[template_callback]
    fn quit_button(&self, _: gtk::Button) {
        self.close();
    }

    pub fn add_error_line(&self, line: &String) {
        self.imp().add_error_line(line);
    }

    pub fn add_line(&self, line: &String) {
        self.imp().add_line(line);
    }
}

unsafe impl Send for WindowExecute {}
