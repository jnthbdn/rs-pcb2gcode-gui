use super::{basetool::BaseTool, tool::Tool};

#[derive(Default, Debug)]
pub struct Drill {
    pub base_tool: BaseTool,
}

impl Drill {
    pub fn new(
        id: u32,
        name: String,
        note: String,
        shaft_diameter: f32,
        tool_diameter: f32,
        spindle_speed: f32,
        pass_depth: f32,
        plunge_rate: f32,
        feed_rate: f32,
    ) -> Self {
        Self {
            base_tool: BaseTool::new(
                id,
                name,
                note,
                shaft_diameter,
                tool_diameter,
                spindle_speed,
                pass_depth,
                plunge_rate,
                feed_rate,
            ),
        }
    }
}

impl Tool for Drill {
    // fn name(&self) -> String {
    //     self.base_tool.name.clone()
    // }

    fn offset(&self, _depth_cut: f32) -> f32 {
        self.base_tool.tool_diameter / 2.0
    }

    // fn shaft_diameter(&self) -> f32 {
    //     self.base_tool.shaft_diameter
    // }

    // fn single_pass_depth(&self) -> f32 {
    //     self.base_tool.single_pass_depth
    // }

    // fn spindle_speed(&self) -> f32 {
    //     self.base_tool.spindle_speed
    // }

    // fn vertical_speed(&self) -> f32 {
    //     self.base_tool.vertical_speed
    // }

    // fn horizontal_speed(&self) -> f32 {
    //     self.base_tool.horizontal_speed
    // }
}
