mod imp;

use gtk::{glib, prelude::*, subclass::prelude::ObjectSubclassIsExt};

use crate::{settings::Settings, units::UnitString, window_tool_db::WindowToolDB};

glib::wrapper! {
    pub struct WindowMain(ObjectSubclass<imp::WindowMain>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow;
}

#[gtk::template_callbacks]
impl WindowMain {
    pub fn new<P: IsA<gtk::Application>>(app: &P, settings: Settings) -> Self {
        let s: Self = glib::Object::builder().property("application", app).build();

        *s.imp().settings.borrow_mut() = settings;

        s.load_window_settings();

        s
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
}
