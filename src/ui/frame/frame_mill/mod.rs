mod imp;

use std::sync::{Arc, Mutex};

use gtk::{glib, prelude::CheckButtonExt, subclass::prelude::ObjectSubclassIsExt};

use crate::{database::database::Database, units::UnitString};

glib::wrapper! {
    pub struct FrameMill(ObjectSubclass<imp::FrameMill>)
        @extends gtk::Widget, gtk::Box,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl FrameMill {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_database(&self, db: Arc<Mutex<Database>>) {
        self.imp().mill_tool.set_database(db.clone());
    }

    pub fn set_units_postfixes(&self, unit: &UnitString) {
        self.imp().depth.set_postfix(unit.measure());
        self.imp().isolation.set_postfix(unit.measure());
    }

    #[template_callback]
    pub fn voronoi_toggled(&self, check: gtk::CheckButton) {
        self.imp().set_enable_voronoi(check.is_active());
    }
}
