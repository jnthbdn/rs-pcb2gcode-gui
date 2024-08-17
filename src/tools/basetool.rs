#[derive(Default, Debug)]
pub struct BaseTool {
    pub id: u32,
    pub name: String,
    pub note: String,

    pub shaft_diameter: f32,
    pub tool_diameter: f32,

    pub spindle_speed: f32,
    pub pass_depth: f32,

    pub plunge_rate: f32,
    pub feed_rate: f32,
}

impl BaseTool {
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
