mod basetool;
pub mod drill;
pub mod endmill;
pub mod tool;
pub mod vbit;

use crate::glib;

#[derive(Debug, Clone, Copy, gtk::glib::Enum)]
#[enum_type(name = "ToolType")]
pub enum ToolType {
    Drill,
    Endmill,
    VBit,
}
