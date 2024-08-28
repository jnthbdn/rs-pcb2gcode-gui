use getset::{CopyGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, CopyGetters, Setters, Default, Serialize, Deserialize)]
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

// impl SettingsWindow {
//     pub(crate) fn from_keyfile(key_file: &gtk::glib::KeyFile) -> Self {
//         Self {
//             is_input_metric: key_file
//                 .boolean("common_frame", "is_input_metric")
//                 .unwrap_or(true),

//             is_output_metric: key_file
//                 .boolean("common_frame", "is_output_metric")
//                 .unwrap_or(true),

//             safe_z: Self::get_float(key_file, 0.0, "safe_z"),

//             tool_change: Self::get_float(key_file, 0.0, "tool_change"),

//             tolerance: Self::get_float(key_file, 0.0, "tolerance"),

//             optimization: Self::get_float(key_file, 0.0, "optimization"),

//             tile_x: key_file.uint64("common_frame", "width").unwrap_or(1),

//             tile_y: key_file.uint64("common_frame", "width").unwrap_or(1),

//             offset_x: Self::get_float(key_file, 0.0, "offset_x"),

//             offset_y: Self::get_float(key_file, 0.0, "offset_y"),

//             mirror_x: Self::get_float(key_file, 0.0, "mirror_x"),

//             is_mirror_y: key_file
//                 .boolean("common_frame", "is_mirror_y")
//                 .unwrap_or(true),

//             is_zero_start: key_file
//                 .boolean("common_frame", "is_zero_start")
//                 .unwrap_or(true),
//         }
//     }

//     fn get_float(key_file: &gtk::glib::KeyFile, default: f64, name: &str) -> f64 {
//         key_file
//             .string("common_frame", name)
//             .unwrap_or(default.to_string().into())
//             .parse::<f64>()
//             .unwrap_or(default)
//     }
// }
