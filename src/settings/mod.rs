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

        if path.exists() {
            log::info!("Setting file found. Try loading");
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            Ok(serde_json::from_reader(reader)?)
        } else {
            log::info!("No settings file.");
            Ok(Default::default())
        }
    }

    pub fn save_settings(&self) -> Result<(), Box<dyn Error>> {
        let file = File::create(dirs::get_config_path_to(SETTING_FILENAME))?;

        Ok(serde_json::to_writer_pretty(file, &self)?)
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
