use super::basetool::BaseTool;

#[derive(Default, Debug)]
pub struct Drill {
    pub base_tool: BaseTool,
}

impl Drill {
    pub fn new(
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

    pub fn new_from_base_tool(base_tool: BaseTool) -> Self {
        Self { base_tool }
    }
}
