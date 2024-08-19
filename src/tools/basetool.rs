#[derive(Default, Debug)]
pub struct BaseTool {
    pub id: u32,
    pub name: String,
    pub note: String,

    pub shaft_diameter: f64,
    pub tool_diameter: f64,

    pub spindle_speed: f64,
    pub pass_depth: f64,

    pub plunge_rate: f64,
    pub feed_rate: f64,
}

impl BaseTool {
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
            id,
            name,
            note,

            shaft_diameter,
            tool_diameter,

            spindle_speed,
            pass_depth,

            plunge_rate,
            feed_rate,
        }
    }
}
