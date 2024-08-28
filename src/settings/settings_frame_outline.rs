use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::tools::ToolType;

#[derive(Debug, Getters, CopyGetters, Setters, Serialize, Deserialize)]
pub struct SettingsFrameOutline {
    #[serde(default = "super::endmill_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tool_type: ToolType,

    #[serde(default = "super::u32_max_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tool_id: u32,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) side: u32,

    #[serde(default = "super::true_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_fill_outline: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_enable_bridge: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) bridge_width: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) bridge_number: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) bridge_depth: f64,
}

impl Default for SettingsFrameOutline {
    fn default() -> Self {
        Self {
            tool_type: ToolType::Endmill,
            tool_id: u32::MAX,
            side: Default::default(),
            is_fill_outline: true,
            is_enable_bridge: Default::default(),
            bridge_width: Default::default(),
            bridge_number: Default::default(),
            bridge_depth: Default::default(),
        }
    }
}
