use super::basetool::BaseTool;

#[derive(Default, Debug)]
pub struct Endmill {
    pub base_tool: BaseTool,
    pub feed_rate: f64,
}

impl Endmill {
    pub fn new_metric(
        id: u32,
        name: String,
        note: String,
        shaft_diameter: f64,
        tool_diameter: f64,
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
        }
    }

    pub fn new_imperial(
        id: u32,
        name: String,
        note: String,
        shaft_diameter: f64,
        tool_diameter: f64,
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
        }
    }

    pub fn new_from_base_tool(base_tool: BaseTool, feed_rate: f64) -> Self {
        Self {
            base_tool,
            feed_rate,
        }
    }
}
