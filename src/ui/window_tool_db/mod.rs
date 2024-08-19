mod imp;
use gtk::{gio, glib, prelude::ActionMapExtManual, subclass::prelude::ObjectSubclassIsExt};

use crate::tools::{drill::Drill, endmill::Endmill, vbit::VBit, ToolType};

glib::wrapper! {
    pub struct WindowToolDB(ObjectSubclass<imp::WindowToolDB>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl WindowToolDB {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    #[template_callback]
    fn refresh_tools(&self, _: gtk::Button) {
        self.imp().refresh_model();
    }

    #[template_callback]
    fn setting_changed(&self) {
        println!("Setting changed");
    }

    #[template_callback]
    fn row_selected(&self, db_id: u32, tool_type: ToolType) {
        match tool_type {
            ToolType::Drill => match self.imp().database.get_drill(db_id) {
                Ok(tool) => match tool {
                    Some(tool) => self.imp().tool_settings.show_drill(&tool),
                    None => eprintln!("[row_selected] No drill with id #{db_id}"),
                },
                Err(e) => eprintln!("[row_selected] Fail to get drill ({e})"),
            },

            ToolType::Endmill => match self.imp().database.get_endmill(db_id) {
                Ok(tool) => match tool {
                    Some(tool) => self.imp().tool_settings.show_endmill(&tool),
                    None => eprintln!("[row_selected] No endmill with id #{db_id}"),
                },
                Err(e) => eprintln!("[row_selected] Fail to get endmill ({e})"),
            },

            ToolType::VBit => match self.imp().database.get_vbit(db_id) {
                Ok(tool) => match tool {
                    Some(tool) => self.imp().tool_settings.show_vbit(&tool),
                    None => eprintln!("[row_selected] No VBit with id #{db_id}"),
                },
                Err(e) => eprintln!("[row_selected] Fail to get VBit ({e})"),
            },
        };
    }

    fn setup_actions(&self) {
        let action_add_endmill = gio::ActionEntry::builder("new_endmill")
            .activate(|win: &Self, _, _| {
                match win.imp().database.add_endmill(&Endmill::new(
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
                    Ok(_) => win.imp().refresh_model(),
                    // TODO Improve this error (AlertDialog ?)
                    Err(e) => eprintln!("action_add_endmill error : {}", e),
                };
            })
            .build();

        let action_add_drill = gio::ActionEntry::builder("new_drill")
            .activate(|win: &Self, _, _| {
                match win.imp().database.add_drill(&Drill::new(
                    0,
                    "New Drill".to_string(),
                    "".to_string(),
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                )) {
                    Ok(_) => win.imp().refresh_model(),
                    // TODO Improve this error (AlertDialog ?)
                    Err(e) => eprintln!("action_add_drill error : {}", e),
                };
            })
            .build();

        let action_add_vbit = gio::ActionEntry::builder("new_vbit")
            .activate(|win: &Self, _, _| {
                match win.imp().database.add_vbit(&VBit::new(
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
                    Ok(_) => win.imp().refresh_model(),
                    // TODO Improve this error (AlertDialog ?)
                    Err(e) => eprintln!("action_add_vbit error : {}", e),
                };
            })
            .build();

        self.add_action_entries([action_add_endmill, action_add_drill, action_add_vbit]);
    }
}
