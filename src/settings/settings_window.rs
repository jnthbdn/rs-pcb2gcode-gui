use getset::{CopyGetters, Getters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, CopyGetters, Getters, Setters, Default, Serialize, Deserialize)]
pub struct SettingsWindow {
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) width: i32,

    #[getset(get_copy = "pub", set = "pub")]
    pub(super) height: i32,

    #[getset(get_copy = "pub", set = "pub")]
    pub(super) maximized: bool,

    #[getset(get = "pub", set = "pub")]
    pub(super) default_folder: Option<String>,
}
