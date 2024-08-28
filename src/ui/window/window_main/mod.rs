mod imp;

use gtk::{gio, glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{settings::Settings, units::UnitString, window_tool_db::WindowToolDB};

glib::wrapper! {
    pub struct WindowMain(ObjectSubclass<imp::WindowMain>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl WindowMain {
    pub fn new<P: IsA<gtk::Application>>(app: &P, settings: Settings) -> Self {
        let win: Self = glib::Object::builder().property("application", app).build();

        win.imp()
            .frame_common
            .load_frame_settings(settings.frame_common());

        win.imp()
            .frame_mill
            .load_frame_settings(settings.frame_mill());

        win.imp()
            .frame_drill
            .load_frame_settings(settings.frame_drill());

        win.imp()
            .frame_outline
            .load_frame_settings(settings.frame_outline());

        win.imp()
            .frame_autolevel
            .load_frame_settings(settings.frame_autolevel());

        *win.imp().settings.borrow_mut() = settings;
        win.load_window_settings();

        win.setup_actions();

        win
    }

    fn load_window_settings(&self) {
        let settings = self.imp().settings.borrow();

        self.set_default_size(settings.window().width(), settings.window().height());
        self.set_maximized(settings.window().maximized());
    }

    fn save_window_settings(&self) {
        let mut settings = self.imp().settings.borrow_mut();

        settings.window_mut().set_width(self.width());
        settings.window_mut().set_height(self.height());
        settings.window_mut().set_maximized(self.is_maximized());

        match settings.save_settings() {
            Ok(_) => log::info!("Settings saved !"),
            //TODO Improve error (dialog ?)
            Err(e) => log::error!("Failed to save settings ({e})"),
        };
    }

    fn save_default_settings(&self) {
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

        match settings.save_settings() {
            Ok(_) => log::info!("Settings saved !"),
            //TODO Improve error (dialog ?)
            Err(e) => log::error!("Failed to save settings ({e})"),
        };
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

    #[template_callback]
    fn run_pcb2gcode(&self, _button: &gtk::Button) {
        let mut params = String::new();

        params += &match self.imp().frame_input_output.get_string_param() {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return;
            }
        };

        params += &match self.imp().frame_common.get_string_param() {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return;
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
                return;
            }
        };

        params += &match self
            .imp()
            .frame_drill
            .get_string_param(self.imp().database.clone())
        {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return;
            }
        };

        params += &match self
            .imp()
            .frame_outline
            .get_string_param(self.imp().database.clone())
        {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return;
            }
        };

        params += &match self.imp().frame_autolevel.get_string_param() {
            Ok(s) => s,
            Err(e) => {
                log::error!("{e}");
                return;
            }
        };

        log::info!("Params: {params}");
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
            .activate(move |_win: &Self, _, _| log::warn!("TODO..."))
            .build();

        let load_config = gio::ActionEntry::builder("load_config")
            .activate(move |_win: &Self, _, _| log::warn!("TODO..."))
            .build();

        let show_commanbd = gio::ActionEntry::builder("show_commanbd")
            .activate(move |_win: &Self, _, _| log::warn!("TODO..."))
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
