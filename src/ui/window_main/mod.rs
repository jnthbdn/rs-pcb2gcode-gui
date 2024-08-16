mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::window_tool_db::WindowToolDB;

glib::wrapper! {
    pub struct WindowMain(ObjectSubclass<imp::WindowMain>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

#[gtk::template_callbacks]
impl WindowMain {
    pub fn new<P: IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    #[template_callback]
    pub fn open_tool_db(&self, _button: &gtk::Button) {
        let win_tool_db = &self.imp().win_tool_db;

        if win_tool_db.borrow().is_none() || win_tool_db.borrow().as_ref().unwrap().is_active() {
            let win = WindowToolDB::new(self);
            self.imp().win_tool_db.replace(Some(win));
        }

        win_tool_db.borrow().as_ref().unwrap().present();
    }

    #[template_callback]
    fn run_pcb2gcode(&self, _button: &gtk::Button) {
        todo!()
    }
}
