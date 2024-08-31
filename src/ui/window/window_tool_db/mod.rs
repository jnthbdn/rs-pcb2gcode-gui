mod imp;

use std::sync::{Arc, Mutex};

use gtk::{
    gio,
    glib::{self, property::PropertySet},
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
};

use crate::{
    database::database::{Database, DatabaseColumn},
    tools::{drill::Drill, endmill::Endmill, vbit::VBit, ToolType},
    ui::show_alert_dialog,
};

glib::wrapper! {
    pub struct WindowToolDB(ObjectSubclass<imp::WindowToolDB>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl WindowToolDB {
    pub fn new(db_mutex: Arc<Mutex<Database>>) -> Self {
        let win: WindowToolDB = glib::Object::builder().build();

        win.imp().database.set(Some(db_mutex));

        let clone = win.clone();
        let db = clone.imp().database.borrow();

        win.imp()
            .tree_tool
            .imp()
            .refresh_tree(db.as_ref().unwrap().clone());

        win.setup_actions();

        win
    }

    #[template_callback]
    fn refresh_tools(&self, _: gtk::Button) {
        self.imp().refresh_model();
    }

    #[template_callback]
    fn setting_changed(
        &self,
        tool_type: ToolType,
        col: DatabaseColumn,
        new_value: glib::GString,
        id: u32,
    ) {
        log::info!("Setting changed => {:?} {:?} = {new_value}", tool_type, col);
        let db = self.imp().database.borrow();
        let db = db.as_ref().unwrap().lock().unwrap();

        let result = match tool_type {
            ToolType::Drill => db.set_drill_column(col, new_value.to_string(), id),
            ToolType::Endmill => db.set_endmill_column(col, new_value.to_string(), id),
            ToolType::VBit => db.set_vbit_column(col, new_value.to_string(), id),
        };

        if result.is_err() {
            log::error!(
                "[setting_changed] Failed to update setting ({})",
                result.err().unwrap()
            );
        } else {
            match col {
                DatabaseColumn::Name => {
                    self.imp()
                        .tree_tool
                        .update_name_model(tool_type, id, &new_value);
                }
                _ => (),
            };

            drop(db);
            self.emit_by_name::<()>("tools-changed", &[]);
        }
    }

    #[template_callback]
    fn row_selected(&self, db_id: u32, tool_type: ToolType) {
        let db = self.imp().database.borrow();
        let db = db.as_ref().unwrap().lock().unwrap();

        self.imp().tool_settings.set_visible(true);
        match tool_type {
            ToolType::Drill => match db.get_drill(db_id) {
                Ok(tool) => match tool {
                    Some(tool) => self.imp().tool_settings.show_drill(&tool),
                    None => log::error!("[row_selected] No drill with id #{db_id}"),
                },
                Err(e) => log::error!("[row_selected] Fail to get drill ({e})"),
            },

            ToolType::Endmill => match db.get_endmill(db_id) {
                Ok(tool) => match tool {
                    Some(tool) => self.imp().tool_settings.show_endmill(&tool),
                    None => log::error!("[row_selected] No endmill with id #{db_id}"),
                },
                Err(e) => log::error!("[row_selected] Fail to get endmill ({e})"),
            },

            ToolType::VBit => match db.get_vbit(db_id) {
                Ok(tool) => match tool {
                    Some(tool) => self.imp().tool_settings.show_vbit(&tool),
                    None => log::error!("[row_selected] No VBit with id #{db_id}"),
                },
                Err(e) => log::error!("[row_selected] Fail to get VBit ({e})"),
            },
        };
    }

    #[template_callback]
    fn close_request(&self, _win: gtk::ApplicationWindow) -> bool {
        self.imp().tool_settings.imp().check_setting_change();
        false
    }

    #[template_callback]
    fn delete_selected_tool(&self, _: gtk::Button) {
        if self.imp().tool_settings.imp().current_tooltype().is_none() {
            log::info!("No tool selected.");
            return;
        }

        let db = self.imp().database.borrow();
        let db = db.as_ref().unwrap().lock().unwrap();

        let id = self.imp().tool_settings.imp().current_id();
        let tool = self.imp().tool_settings.imp().current_tooltype().unwrap();

        let result = match tool {
            ToolType::Drill => db.remove_drill(id),
            ToolType::Endmill => db.remove_endmill(id),
            ToolType::VBit => db.remove_vbit(id),
        };

        if result.is_err() {
            log::error!(
                "Failed to delete tool '{:?}' with id #{id} ({:?})",
                tool,
                result.err()
            )
        }

        drop(db);
        self.imp().refresh_model();
        self.emit_by_name::<()>("tools-changed", &[]);
    }

    fn setup_actions(&self) {
        let db = self.imp().database.borrow();
        let db = db.as_ref().unwrap();

        let db_clone = db.clone();
        let action_add_endmill_metric = gio::ActionEntry::builder("new_endmill_metric")
            .activate(move |win: &Self, _, _| {
                let db = db_clone.lock().unwrap();
                match db.add_endmill(&Endmill::new_metric(
                    0,
                    "New Endmill".to_string(),
                    "".to_string(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                )) {
                    Ok(_) => {
                        drop(db);
                        win.imp().refresh_model();
                        win.emit_by_name::<()>("tools-changed", &[]);
                    }
                    Err(e) => {
                        show_alert_dialog(
                            // TODO Translation....
                            "Add endmill error",
                            &e.to_string(),
                            win.upcast_ref::<gtk::Window>(),
                        );
                        log::error!("Failed to add metric endmill ({e})")
                    }
                };
            })
            .build();

        let db_clone = db.clone();
        let action_add_drill_metric = gio::ActionEntry::builder("new_drill_metric")
            .activate(move |win: &Self, _, _| {
                let db = db_clone.lock().unwrap();
                match db.add_drill(&Drill::new_metric(
                    0,
                    "New Drill".to_string(),
                    "".to_string(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                )) {
                    Ok(_) => {
                        drop(db);
                        win.imp().refresh_model();
                        win.emit_by_name::<()>("tools-changed", &[]);
                    }
                    Err(e) => {
                        show_alert_dialog(
                            // TODO Translation....
                            "Add drill error",
                            &e.to_string(),
                            win.upcast_ref::<gtk::Window>(),
                        );
                        log::error!("Failed to add metric drill ({e})")
                    }
                };
            })
            .build();

        let db_clone = db.clone();
        let action_add_vbit_metric = gio::ActionEntry::builder("new_vbit_metric")
            .activate(move |win: &Self, _, _| {
                let db = db_clone.lock().unwrap();
                match db.add_vbit(&VBit::new_metric(
                    0,
                    "New V-Bit".to_string(),
                    "".to_string(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                )) {
                    Ok(_) => {
                        drop(db);
                        win.imp().refresh_model();
                        win.emit_by_name::<()>("tools-changed", &[]);
                    }
                    Err(e) => {
                        show_alert_dialog(
                            // TODO Translation....
                            "Add V-Bit error",
                            &e.to_string(),
                            win.upcast_ref::<gtk::Window>(),
                        );
                        log::error!("Failed to add metric vbit ({e})")
                    }
                };
            })
            .build();

        let db_clone = db.clone();
        let action_add_endmill_imperial = gio::ActionEntry::builder("new_endmill_imperial")
            .activate(move |win: &Self, _, _| {
                let db = db_clone.lock().unwrap();
                match db.add_endmill(&Endmill::new_imperial(
                    0,
                    "New Endmill".to_string(),
                    "".to_string(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                )) {
                    Ok(_) => {
                        drop(db);
                        win.imp().refresh_model();
                        win.emit_by_name::<()>("tools-changed", &[]);
                    }
                    Err(e) => {
                        show_alert_dialog(
                            // TODO Translation....
                            "Add endmill error",
                            &e.to_string(),
                            win.upcast_ref::<gtk::Window>(),
                        );
                        log::error!("Failed to add imperial endmill ({e})")
                    }
                };
            })
            .build();

        let db_clone = db.clone();
        let action_add_drill_imperial = gio::ActionEntry::builder("new_drill_imperial")
            .activate(move |win: &Self, _, _| {
                let db = db_clone.lock().unwrap();
                match db.add_drill(&Drill::new_imperial(
                    0,
                    "New Drill".to_string(),
                    "".to_string(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                )) {
                    Ok(_) => {
                        drop(db);
                        win.imp().refresh_model();
                        win.emit_by_name::<()>("tools-changed", &[]);
                    }
                    Err(e) => {
                        show_alert_dialog(
                            // TODO Translation....
                            "Add drill error",
                            &e.to_string(),
                            win.upcast_ref::<gtk::Window>(),
                        );
                        log::error!("Failed to add imperial drill ({e})")
                    }
                };
            })
            .build();

        let db_clone = db.clone();
        let action_add_vbit_imperial = gio::ActionEntry::builder("new_vbit_imperial")
            .activate(move |win: &Self, _, _| {
                let db = db_clone.lock().unwrap();
                match db.add_vbit(&VBit::new_imperial(
                    0,
                    "New V-Bit".to_string(),
                    "".to_string(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                )) {
                    Ok(_) => {
                        drop(db);
                        win.imp().refresh_model();
                        win.emit_by_name::<()>("tools-changed", &[]);
                    }
                    Err(e) => {
                        show_alert_dialog(
                            // TODO Translation....
                            "Add V-Bit error",
                            &e.to_string(),
                            win.upcast_ref::<gtk::Window>(),
                        );
                        log::error!("Failed to add imperial v-bit ({e})")
                    }
                };
            })
            .build();

        self.add_action_entries([
            action_add_endmill_metric,
            action_add_drill_metric,
            action_add_vbit_metric,
            action_add_endmill_imperial,
            action_add_drill_imperial,
            action_add_vbit_imperial,
        ]);
    }
}
