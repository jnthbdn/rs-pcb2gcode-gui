mod basetool;
pub mod drill;
pub mod endmill;
pub mod tool;
pub mod vbit;

#[derive(Debug, Clone, Copy)]
pub enum ToolType {
    Drill,
    Endmill,
    VBit,
}
