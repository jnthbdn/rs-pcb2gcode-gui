use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

use crate::tools::ToolType;

#[derive(Debug, Getters, CopyGetters, Setters, Serialize, Deserialize)]
pub struct SettingsFrameMill {
    #[serde(default = "super::vbit_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tool_type: ToolType,

    #[serde(default = "super::u32_max_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) tool_id: u32,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) overlap: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) depth: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) direction: u32,

    #[serde(default = "super::true_default")]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_isolation_width_tool: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) isolation: f64,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_invert_gerber: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_voronoi: bool,

    #[serde(default)]
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) is_thermal_region: bool,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    pub(super) pre_milling: String,

    #[serde(default)]
    #[getset(get = "pub", set = "pub")]
    pub(super) post_milling: String,
}

impl Default for SettingsFrameMill {
    fn default() -> Self {
        Self {
            tool_type: ToolType::VBit,
            tool_id: u32::MAX,
            overlap: Default::default(),
            depth: Default::default(),
            direction: Default::default(),
            is_isolation_width_tool: true,
            isolation: Default::default(),
            is_invert_gerber: Default::default(),
            is_voronoi: Default::default(),
            is_thermal_region: Default::default(),
            pre_milling: Default::default(),
            post_milling: Default::default(),
        }
    }
}
