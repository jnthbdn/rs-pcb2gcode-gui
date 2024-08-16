#[derive(Default, Debug)]
pub struct BaseTool {
    pub name: String,

    pub shaft_diameter: f32,
    pub tool_diameter: f32,

    pub spindle_speed: f32,
    pub single_pass_depth: f32,

    pub vertical_speed: f32,
    pub horizontal_speed: f32,
}

impl BaseTool {
    pub fn new(
        name: String,
        shaft_diameter: f32,
        tool_diameter: f32,
        spindle_speed: f32,
        single_pass_depth: f32,
        vertical_speed: f32,
        horizontal_speed: f32,
    ) -> Self {
        Self {
            name,

            shaft_diameter,
            tool_diameter,

            spindle_speed,
            single_pass_depth,

            vertical_speed,
            horizontal_speed,
        }
    }
}

// macro_rules! builder_base_tool {
//     ($field_tool:ident, $tool_type:ident) => {
//         pub fn builder() -> Self {
//             Self::default()
//         }

//         pub fn name(mut self, name: String) -> Self {
//             self.$field_tool.base_tool.name = name;
//             self
//         }

//         pub fn shaft_diameter(mut self, shaft_diameter: f32) -> Self {
//             self.$field_tool.base_tool.shaft_diameter = shaft_diameter;
//             self
//         }

//         pub fn tool_diameter(mut self, tool_diameter: f32) -> Self {
//             self.$field_tool.base_tool.tool_diameter = tool_diameter;
//             self
//         }

//         pub fn spindle_speed(mut self, spindle_speed: f32) -> Self {
//             self.$field_tool.base_tool.spindle_speed = spindle_speed;
//             self
//         }

//         pub fn single_pass_depth(mut self, single_pass_depth: f32) -> Self {
//             self.$field_tool.base_tool.single_pass_depth = single_pass_depth;
//             self
//         }

//         pub fn vertical_speed(mut self, vertical_speed: f32) -> Self {
//             self.$field_tool.base_tool.vertical_speed = vertical_speed;
//             self
//         }

//         pub fn horizontal_speed(mut self, horizontal_speed: f32) -> Self {
//             self.$field_tool.base_tool.horizontal_speed = horizontal_speed;
//             self
//         }

//         pub fn build(self) -> Result<$tool_type, ()> {
//             if self.check_tool_is_valid() {
//                 Ok(self.$field_tool)
//             } else {
//                 Err(())
//             }
//         }
//     };
// }
// pub(crate) use builder_base_tool;
