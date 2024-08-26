use getset::{CopyGetters, Getters, MutGetters, Setters};
use gtk::glib;

use crate::dirs;

static SETTING_FILENAME: &str = "settings.ini";

#[derive(Debug, CopyGetters, Setters, Default)]
pub struct SettingsWindow {
    #[getset(get_copy = "pub", set = "pub")]
    width: i32,

    #[getset(get_copy = "pub", set = "pub")]
    height: i32,

    #[getset(get_copy = "pub", set = "pub")]
    maximized: bool,
}

#[derive(Debug, MutGetters, Getters, Default)]
pub struct Settings {
    #[getset(get_mut = "pub", get = "pub")]
    window: SettingsWindow,
}

impl Settings {
    pub fn new() -> Result<Self, glib::Error> {
        let key_file = glib::KeyFile::new();

        match Self::open_file(&key_file) {
            Ok(_) => log::info!("Settings file ready."),
            Err(e) => return Err(e),
        };

        Ok(Self::load_settings(&key_file))
    }

    fn open_file(key_file: &glib::KeyFile) -> Result<(), glib::Error> {
        let path = dirs::get_config_path_to(SETTING_FILENAME);

        if path.exists() {
            log::info!("Setting file found. Try loading");
            key_file.load_from_file(path, glib::KeyFileFlags::empty())?;
        }

        Ok(())
    }

    fn load_settings(key_file: &glib::KeyFile) -> Self {
        Self {
            window: SettingsWindow {
                width: key_file.integer("window", "width").unwrap_or(-1),
                height: key_file.integer("window", "height").unwrap_or(-1),
                maximized: key_file.boolean("window", "maximized").unwrap_or(false),
            },
        }
    }

    pub fn save_settings(&self) -> Result<(), glib::Error> {
        let key_file = glib::KeyFile::new();

        match Self::open_file(&key_file) {
            Ok(_) => log::info!("Settings file ready."),
            Err(e) => return Err(e),
        };

        key_file.set_integer("window", "width", self.window.width);
        key_file.set_integer("window", "height", self.window.height);
        key_file.set_boolean("window", "maximized", self.window.maximized);

        key_file.save_to_file(dirs::get_config_path_to(SETTING_FILENAME))?;

        Ok(())
    }
}
