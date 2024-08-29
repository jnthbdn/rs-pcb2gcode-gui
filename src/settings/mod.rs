pub mod settings_frame_autolevel;
pub mod settings_frame_common;
pub mod settings_frame_drill;
pub mod settings_frame_mill;
pub mod settings_frame_outline;
pub mod settings_window;

use std::{error::Error, fs::File, io::BufReader};

use getset::{Getters, MutGetters};
use settings_frame_autolevel::SettingsFrameAutolevel;
use settings_frame_common::SettingsFrameCommon;
use settings_frame_drill::SettingsFrameDrill;
use settings_frame_mill::SettingsFrameMill;
use settings_frame_outline::SettingsFrameOutline;
use settings_window::SettingsWindow;

use crate::{dirs, tools::ToolType};
use serde::{Deserialize, Serialize};

static SETTING_FILENAME: &str = "settings.json";

#[derive(Debug, MutGetters, Getters, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    #[getset(get_mut = "pub", get = "pub")]
    window: SettingsWindow,

    #[serde(default)]
    #[getset(get_mut = "pub", get = "pub")]
    frame_common: SettingsFrameCommon,

    #[serde(default)]
    #[getset(get_mut = "pub", get = "pub")]
    frame_mill: SettingsFrameMill,

    #[serde(default)]
    #[getset(get_mut = "pub", get = "pub")]
    frame_drill: SettingsFrameDrill,

    #[serde(default)]
    #[getset(get_mut = "pub", get = "pub")]
    frame_outline: SettingsFrameOutline,

    #[serde(default)]
    #[getset(get_mut = "pub", get = "pub")]
    frame_autolevel: SettingsFrameAutolevel,
}

impl Settings {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let path = dirs::get_config_path_to(SETTING_FILENAME);
        let mut settings: Self = Default::default();

        if path.exists() {
            log::info!("Setting file found.");
            let file = File::open(path)?;

            settings.load_from_file(&file)?;
        } else {
            log::info!("No settings file.");
        }

        Ok(settings)
    }

    pub fn save_settings(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(dirs::get_config_path_to(SETTING_FILENAME))?;

        Ok(serde_json::to_writer_pretty(file, &self)?)
    }

    pub fn save_to_file(&self, file: &File) -> Result<(), Box<dyn Error>> {
        Ok(serde_json::to_writer_pretty(file, &self)?)
    }

    pub fn load_from_file(&mut self, file: &File) -> Result<(), Box<dyn Error>> {
        let settings: Self = serde_json::from_reader(BufReader::new(file))?;

        self.window = settings.window;
        self.frame_common = settings.frame_common;
        self.frame_mill = settings.frame_mill;
        self.frame_drill = settings.frame_drill;
        self.frame_outline = settings.frame_outline;
        self.frame_autolevel = settings.frame_autolevel;

        Ok(())
    }
}

fn true_default() -> bool {
    true
}

fn false_default() -> bool {
    false
}

fn one_default() -> f64 {
    1.0
}

fn u32_max_default() -> u32 {
    u32::MAX
}

fn vbit_default() -> ToolType {
    ToolType::VBit
}

fn drill_default() -> ToolType {
    ToolType::Drill
}

fn endmill_default() -> ToolType {
    ToolType::Endmill
}
