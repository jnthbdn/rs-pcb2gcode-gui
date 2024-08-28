use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::tools::ToolType;

#[derive(Debug, Getters, CopyGetters, Setters, Serialize, Deserialize)]
pub struct SettingsFrameDrill {
    #[serde(default = "super::drill_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) drill_tool_type: ToolType,

    #[serde(default = "super::u32_max_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) drill_tool_id: u32,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) depth: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) side: u32,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_enable_milldrilling: bool,

    #[serde(default = "super::endmill_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) milldrilling_tool_type: ToolType,

    #[serde(default = "super::u32_max_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) milldrilling_tool_id: u32,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) milldrilling_min_diameter: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) milldrilling_depth: f64,

    #[serde(default = "super::true_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_no_g91_1: bool,

    #[serde(default = "super::true_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_no_g81: bool,
}

impl Default for SettingsFrameDrill {
    fn default() -> Self {
        Self {
            drill_tool_type: ToolType::Drill,
            drill_tool_id: u32::MAX,
            depth: Default::default(),
            side: Default::default(),
            is_enable_milldrilling: Default::default(),
            milldrilling_tool_type: ToolType::Endmill,
            milldrilling_tool_id: u32::MAX,
            milldrilling_min_diameter: Default::default(),
            milldrilling_depth: Default::default(),
            is_no_g91_1: true,
            is_no_g81: true,
        }
    }
}
