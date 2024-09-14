use super::basetool::BaseTool;

#[derive(Default, Debug)]
pub struct VBit {
    pub base_tool: BaseTool,
    pub feed_rate: f64,
    pub tool_angle: f64,
    pub tip_diameter: f64,
}

impl VBit {
    pub fn new_metric(
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
            base_tool: BaseTool::new_metric(
                id,
                name,
                note,
                shaft_diameter,
                tool_diameter,
                spindle_speed,
                pass_depth,
                plunge_rate,
            ),
            feed_rate,
            tool_angle,
            tip_diameter,
        }
    }

    pub fn new_imperial(
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
            base_tool: BaseTool::new_imperial(
                id,
                name,
                note,
                shaft_diameter,
                tool_diameter,
                spindle_speed,
                pass_depth,
                plunge_rate,
            ),
            feed_rate,
            tool_angle,
            tip_diameter,
        }
    }

    pub fn new_from_base_tool(
        base_tool: BaseTool,
        feed_rate: f64,
        tool_angle: f64,
        tip_diameter: f64,
    ) -> Self {
        Self {
            base_tool,
            feed_rate,
            tool_angle,
            tip_diameter,
        }
    }

    pub fn diameter(&self, depth_cut: f64) -> f64 {
        let alpha = self.tool_angle / 2.0;
        let opposite = alpha.to_radians().tan() * depth_cut;

        2.0 * opposite + self.tip_diameter
    }
}
