pub mod basetool;
pub mod drill;
pub mod endmill;
pub mod vbit;

use serde::{Deserialize, Serialize};

use crate::glib;

#[derive(Debug, Clone, Copy, gtk::glib::Enum, PartialEq, Serialize, Deserialize)]
#[enum_type(name = "ToolType")]
pub enum ToolType {
    Drill,
    Endmill,
    VBit,
}
