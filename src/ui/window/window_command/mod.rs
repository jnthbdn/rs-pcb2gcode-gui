mod imp;

use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
};

glib::wrapper! {
    pub struct WindowCommand(ObjectSubclass<imp::WindowCommand>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

#[gtk::template_callbacks]
impl WindowCommand {
    pub fn new(parent: &gtk::Window) -> Self {
        glib::Object::builder()
            .property("transient-for", parent)
            .build()
    }

    pub fn open(&self, text: String) {
        self.imp().textview.buffer().set_text(&text);
        self.present();
    }

    #[template_callback]
    fn quit_button(&self, _: gtk::Button) {
        self.close();
    }
}
