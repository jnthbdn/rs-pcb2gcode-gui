mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{units::UnitString, window_tool_db::WindowToolDB};

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

        if win_tool_db.borrow().is_none() || win_tool_db.borrow().as_ref().unwrap().ref_count() == 1
        {
            let win = WindowToolDB::new();
            self.imp().win_tool_db.replace(Some(win));
        }

        win_tool_db.borrow().as_ref().unwrap().present();
    }

    #[template_callback]
    fn run_pcb2gcode(&self, _button: &gtk::Button) {
        todo!()
    }

    #[template_callback]
    pub fn output_unit_change(&self, is_metric: bool) {
        let unit = if is_metric {
            UnitString::new_metric()
        } else {
            UnitString::new_imperial()
        };

        self.imp().frame_common.set_units_postfixes(&unit);
        self.imp().frame_mill.set_units_postfixes(&unit);
        self.imp().frame_drill.set_units_postfixes(&unit);
        self.imp().frame_outline.set_units_postfixes(&unit);
    }
}
