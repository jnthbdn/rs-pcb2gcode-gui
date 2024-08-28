use getset::{CopyGetters, Setters};
use serde::{Deserialize, Serialize};

#[derive(Debug, CopyGetters, Setters, Default, Serialize, Deserialize)]
pub struct SettingsWindow {
    #[getset(get_copy = "pub", set = "pub")]
    pub(super) width: i32,

    #[getset(get_copy = "pub", set = "pub")]
    pub(super) height: i32,

    #[getset(get_copy = "pub", set = "pub")]
    pub(super) maximized: bool,
}
