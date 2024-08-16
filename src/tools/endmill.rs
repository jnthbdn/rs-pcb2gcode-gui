use super::{basetool::BaseTool, tool::Tool};

#[derive(Default, Debug)]
pub struct Endmill {
    base_tool: BaseTool,
}

impl Endmill {
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
            base_tool: BaseTool::new(
                name,
                shaft_diameter,
                tool_diameter,
                spindle_speed,
                single_pass_depth,
                vertical_speed,
                horizontal_speed,
            ),
        }
    }
}

impl Tool for Endmill {
    fn name(&self) -> String {
        self.base_tool.name.clone()
    }

    fn offset(&self, _depth_cut: f32) -> f32 {
        self.base_tool.tool_diameter / 2.0
    }

    fn shaft_diameter(&self) -> f32 {
        self.base_tool.shaft_diameter
    }

    fn single_pass_depth(&self) -> f32 {
        self.base_tool.single_pass_depth
    }

    fn spindle_speed(&self) -> f32 {
        self.base_tool.spindle_speed
    }

    fn vertical_speed(&self) -> f32 {
        self.base_tool.vertical_speed
    }

    fn horizontal_speed(&self) -> f32 {
        self.base_tool.horizontal_speed
    }
}
