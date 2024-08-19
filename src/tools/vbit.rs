use super::{basetool::BaseTool, tool::Tool};

#[derive(Default, Debug)]
pub struct VBit {
    pub base_tool: BaseTool,
    pub tool_angle: f64,
    pub tip_diameter: f64,
}

impl VBit {
    pub fn new(
        id: u32,
        name: String,
        note: String,
        shaft_diameter: f64,
        tool_diameter: f64,
        tool_angle: f64,
        tip_diameter: f64,
        spindle_speed: f64,
        pass_depth: f64,
        plunge_rate: f64,
        feed_rate: f64,
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
            tool_angle,
            tip_diameter,
        }
    }

    pub fn new_from_base_tool(base_tool: BaseTool, tool_angle: f64, tip_diameter: f64) -> Self {
        Self {
            base_tool,
            tool_angle,
            tip_diameter,
        }
    }
}

impl Tool for VBit {
    // fn name(&self) -> String {
    //     self.base_tool.name.clone()
    // }

    fn offset(&self, _depth_cut: f64) -> f64 {
        // self.base_tool.tool_diameter / 2.0
        todo!()
    }

    // fn shaft_diameter(&self) -> f64 {
    //     self.base_tool.shaft_diameter
    // }

    // fn single_pass_depth(&self) -> f64 {
    //     self.base_tool.single_pass_depth
    // }

    // fn spindle_speed(&self) -> f64 {
    //     self.base_tool.spindle_speed
    // }

    // fn vertical_speed(&self) -> f64 {
    //     self.base_tool.vertical_speed
    // }

    // fn horizontal_speed(&self) -> f64 {
    //     self.base_tool.horizontal_speed
    // }
}
