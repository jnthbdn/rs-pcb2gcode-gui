mod imp;

use std::fs::File;

use gtk::{gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{
    settings::Settings, ui::show_alert_dialog, units::UnitString, window_tool_db::WindowToolDB,
};

use super::window_command::WindowCommand;

glib::wrapper! {
    pub struct WindowMain(ObjectSubclass<imp::WindowMain>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl WindowMain {
    pub fn new<P: IsA<gtk::Application>>(app: &P, settings: Settings) -> Self {
        let win: Self = glib::Object::builder().property("application", app).build();

        *win.imp().settings.borrow_mut() = settings;
        win.load_settings();

        win.setup_actions();

        win
    }

    fn load_settings(&self) {
        let settings = self.imp().settings.borrow();

        self.set_default_size(settings.window().width(), settings.window().height());
        self.set_maximized(settings.window().maximized());

        self.imp()
            .frame_common
            .load_frame_settings(settings.frame_common());

        self.imp()
            .frame_mill
            .load_frame_settings(settings.frame_mill());

        self.imp()
            .frame_drill
            .load_frame_settings(settings.frame_drill());

        self.imp()
            .frame_outline
            .load_frame_settings(settings.frame_outline());

        self.imp()
            .frame_autolevel
            .load_frame_settings(settings.frame_autolevel());
    }

    fn save_window_settings(&self) {
        let mut settings = self.imp().settings.borrow_mut();

        settings.window_mut().set_width(self.width());
        settings.window_mut().set_height(self.height());
        settings.window_mut().set_maximized(self.is_maximized());

        match settings.save_settings() {
            Ok(_) => log::info!("Window settings saved."),
            Err(e) => {
                show_alert_dialog(
                    // TODO Translation....
                    "Fail save window settings",
                    &e.to_string(),
                    self.upcast_ref::<gtk::Window>(),
                );
                log::error!("Failed to save window settings ({e})")
            }
        };
    }

    fn save_default_settings(&self) {
        self.update_settings();

        match self.imp().settings.borrow().save_settings() {
            Ok(_) => log::info!("Settings saved."),
            Err(e) => {
                show_alert_dialog(
                    // TODO Translation....
                    "Fail save default settings",
                    &e.to_string(),
                    self.upcast_ref::<gtk::Window>(),
                );
                log::error!("Failed to save default settings ({e})")
            }
        };
    }

    fn update_settings(&self) {
        let mut settings = self.imp().settings.borrow_mut();

        self.imp()
            .frame_common
            .save_frame_settings(settings.frame_common_mut());

        self.imp()
            .frame_mill
            .save_frame_settings(settings.frame_mill_mut());

        self.imp()
            .frame_drill
            .save_frame_settings(settings.frame_drill_mut());

        self.imp()
            .frame_outline
            .save_frame_settings(settings.frame_outline_mut());

        self.imp()
            .frame_autolevel
            .save_frame_settings(settings.frame_autolevel_mut());
    }

    #[template_callback]
    pub fn open_tool_db(&self, _button: &gtk::Button) {
        let win_tool_db = &self.imp().win_tool_db;

        if win_tool_db.borrow().is_none() || win_tool_db.borrow().as_ref().unwrap().ref_count() == 1
        {
            let win = WindowToolDB::new(self.imp().database.clone());

            let self_clone = self.clone();
            win.connect_local("tools-changed", false, move |_| {
                self_clone.imp().frame_mill.refresh_tools();
                self_clone.imp().frame_drill.refresh_tools();
                self_clone.imp().frame_outline.refresh_tools();
                None
            });

            self.imp().win_tool_db.replace(Some(win));
        }

        win_tool_db.borrow().as_ref().unwrap().present();
    }

    fn get_pcb2gcode_params(&self) -> Result<String, String> {
        let mut params = String::new();

        params += &match self.imp().frame_input_output.get_string_param() {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return Err(e);
            }
        };

        params += &match self.imp().frame_common.get_string_param() {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return Err(e);
            }
        };

        params += &match self
            .imp()
            .frame_mill
            .get_string_param(self.imp().database.clone())
        {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return Err(e);
            }
        };

        if self.imp().frame_input_output.is_drill_file_available() {
            params += &match self
                .imp()
                .frame_drill
                .get_string_param(self.imp().database.clone())
            {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{e}");
                    return Err(e);
                }
            };
        }

        if self.imp().frame_input_output.is_outline_file_available() {
            params += &match self
                .imp()
                .frame_outline
                .get_string_param(self.imp().database.clone())
            {
                Ok(s) => s,
                Err(e) => {
                    log::error!("{e}");
                    return Err(e);
                }
            };
        }

        params += &match self.imp().frame_autolevel.get_string_param() {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return Err(e);
            }
        };

        Ok(params)
    }

    #[template_callback]
    fn run_pcb2gcode(&self, _button: &gtk::Button) {
        let params = self.get_pcb2gcode_params();

        if params.is_err() {
            //TODO Translation...
            show_alert_dialog(
                "Failed to run pcb2gcode",
                &params.err().unwrap(),
                self.upcast_ref::<gtk::Window>(),
            );
            return;
        }

        log::info!("Params: {:?}", params);
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

    fn setup_actions(&self) {
        let save_as_default = gio::ActionEntry::builder("save_as_default")
            .activate(move |win: &Self, _, _| {
                win.save_default_settings();
            })
            .build();

        let save_config = gio::ActionEntry::builder("save_config")
            .activate(move |win: &Self, _, _| {
                let filters = gio::ListStore::new::<gtk::FileFilter>();

                let filter_json = gtk::FileFilter::new();
                // TODO Translation...
                filter_json.set_name(Some("JSON file"));
                filter_json.add_pattern("*.json");

                let filter_all = gtk::FileFilter::new();
                // TODO Translation...
                filter_all.set_name(Some("All files"));
                filter_all.add_pattern("*.*");

                filters.extend_from_slice(&[filter_json, filter_all]);

                let dialog = gtk::FileDialog::builder()
                    .filters(&filters)
                    //TODO Translation...
                    .title("Save configuration file")
                    .initial_name("settings.json")
                    .modal(true)
                    .build();

                let win_clone = win.clone();
                dialog.save(
                    Some(win.upcast_ref::<gtk::Window>()),
                    None::<&gio::Cancellable>,
                    move |res| {
                        match res {
                            Ok(file) => {
                                let path = file.path().unwrap();

                                match File::create(path) {
                                    Ok(file) => {
                                        win_clone.update_settings();
                                        match win_clone.imp().settings.borrow().save_to_file(&file)
                                        {
                                            Ok(_) => (),
                                            Err(e) => {
                                                show_alert_dialog(
                                                    //TODO Translation...
                                                    "Fail saving settings",
                                                    &e.to_string(),
                                                    win_clone.upcast_ref::<gtk::Window>(),
                                                );
                                                log::error!("Failed to save settings file ({e})")
                                            }
                                        }
                                    }

                                    Err(e) => {
                                        show_alert_dialog(
                                            //TODO Translation...
                                            "Fail open file",
                                            &e.to_string(),
                                            win_clone.upcast_ref::<gtk::Window>(),
                                        );
                                        log::error!("Failed to open file ({e})")
                                    }
                                };
                            }
                            Err(e) => log::warn!("{e}"),
                        };
                    },
                );
            })
            .build();

        let load_config = gio::ActionEntry::builder("load_config")
            .activate(move |win: &Self, _, _| {
                let filters = gio::ListStore::new::<gtk::FileFilter>();

                let filter_json = gtk::FileFilter::new();
                // TODO Translation...
                filter_json.set_name(Some("JSON file"));
                filter_json.add_pattern("*.json");

                let filter_all = gtk::FileFilter::new();
                // TODO Translation...
                filter_all.set_name(Some("All files"));
                filter_all.add_pattern("*.*");

                filters.extend_from_slice(&[filter_json, filter_all]);

                let dialog = gtk::FileDialog::builder()
                    .filters(&filters)
                    //TODO Translation...
                    .title("Open configuration file")
                    .initial_name("settings.json")
                    .modal(true)
                    .build();

                let win_clone = win.clone();
                dialog.open(
                    Some(win.upcast_ref::<gtk::Window>()),
                    None::<&gio::Cancellable>,
                    move |res| {
                        match res {
                            Ok(file) => {
                                let path = file.path().unwrap();

                                match File::open(path) {
                                    Ok(file) => {
                                        let load = win_clone
                                            .imp()
                                            .settings
                                            .borrow_mut()
                                            .load_from_file(&file);
                                        match load {
                                            Ok(_) => win_clone.load_settings(),
                                            Err(e) => {
                                                show_alert_dialog(
                                                    //TODO Translation...
                                                    "Fail loading settings",
                                                    &e.to_string(),
                                                    win_clone.upcast_ref::<gtk::Window>(),
                                                );
                                                log::error!("Failed to load settings file ({e})")
                                            }
                                        }
                                    }

                                    Err(e) => {
                                        show_alert_dialog(
                                            //TODO Translation...
                                            "Fail open file",
                                            &e.to_string(),
                                            win_clone.upcast_ref::<gtk::Window>(),
                                        );
                                        log::error!("Failed to open file ({e})")
                                    }
                                };
                            }
                            Err(e) => log::warn!("{e}"),
                        };
                    },
                );
            })
            .build();

        let show_commanbd = gio::ActionEntry::builder("show_command")
            .activate(move |win: &Self, _, _| {
                let w = WindowCommand::new(win.upcast_ref::<gtk::Window>());
                w.open(
                    win.get_pcb2gcode_params()
                        .or_else(|e| Ok::<String, String>("ERROR: ".to_string() + &e))
                        .unwrap(),
                );
            })
            .build();

        let force_preview = gio::ActionEntry::builder("force_preview")
            .activate(move |_win: &Self, _, _| log::warn!("TODO..."))
            .build();

        let about = gio::ActionEntry::builder("about")
            .activate(move |_win: &Self, _, _| log::warn!("TODO..."))
            .build();

        let about_pcb2gcode = gio::ActionEntry::builder("about_pcb2gcode")
            .activate(move |_win: &Self, _, _| log::warn!("TODO..."))
            .build();

        let about_doc = gio::ActionEntry::builder("about_doc")
            .activate(move |_win: &Self, _, _| log::warn!("TODO..."))
            .build();

        self.add_action_entries([
            save_as_default,
            save_config,
            load_config,
            show_commanbd,
            force_preview,
            about,
            about_pcb2gcode,
            about_doc,
        ]);
    }
}
