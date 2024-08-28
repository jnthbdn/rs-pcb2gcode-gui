use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, Getters, CopyGetters, Setters, Serialize, Deserialize)]
pub struct SettingsFrameAutolevel {
    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_enable_front: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_enable_back: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) software: u32,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) distance_probe_x: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) distance_probe_y: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) feed: f64,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    pub(super) probe_on: String,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    pub(super) probe_off: String,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    pub(super) probe_code: String,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    pub(super) probe_variable: String,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    pub(super) probe_set_zero: String,
}

impl Default for SettingsFrameAutolevel {
    fn default() -> Self {
        Self {
            is_enable_front: Default::default(),
            is_enable_back: Default::default(),
            software: Default::default(),
            distance_probe_x: Default::default(),
            distance_probe_y: Default::default(),
            feed: Default::default(),
            probe_on: Default::default(),
            probe_off: Default::default(),
            probe_code: Default::default(),
            probe_variable: Default::default(),
            probe_set_zero: Default::default(),
        }
    }
}
