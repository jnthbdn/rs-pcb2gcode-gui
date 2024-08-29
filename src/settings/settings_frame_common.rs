use getset::{CopyGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, CopyGetters, Setters, Serialize, Deserialize)]
pub struct SettingsFrameCommon {
    #[serde(default = "super::true_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_input_metric: bool,

    #[serde(default = "super::true_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_output_metric: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) safe_z: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tool_change: f64,

    #[serde(default = "super::false_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_tool_change_as_machine_coord: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tolerance: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) optimization: f64,

    #[serde(default = "super::one_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tile_x: f64,

    #[serde(default = "super::one_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tile_y: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) offset_x: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) offset_y: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) mirror_x: f64,

    #[serde(default = "super::false_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_mirror_y: bool,

    #[serde(default = "super::false_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_zero_start: bool,
}

impl Default for SettingsFrameCommon {
    fn default() -> Self {
        Self {
            is_input_metric: true,
            is_output_metric: true,
            safe_z: Default::default(),
            tool_change: Default::default(),
            is_tool_change_as_machine_coord: Default::default(),
            tolerance: Default::default(),
            optimization: Default::default(),
            tile_x: 1.0,
            tile_y: 1.0,
            offset_x: Default::default(),
            offset_y: Default::default(),
            mirror_x: Default::default(),
            is_mirror_y: Default::default(),
            is_zero_start: Default::default(),
        }
    }
}
