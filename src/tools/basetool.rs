use crate::units::UnitString;

#[derive(Debug)]
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

    pub unit: UnitString,
}

impl BaseTool {
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
            id,
            name,
            note,

            shaft_diameter,
            tool_diameter,

            spindle_speed,
            pass_depth,

            plunge_rate,
            feed_rate,

            unit: UnitString::new_metric(),
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
            id,
            name,
            note,

            shaft_diameter,
            tool_diameter,

            spindle_speed,
            pass_depth,

            plunge_rate,
            feed_rate,

            unit: UnitString::new_imperial(),
        }
    }
}

impl Default for BaseTool {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            note: Default::default(),
            shaft_diameter: Default::default(),
            tool_diameter: Default::default(),
            spindle_speed: Default::default(),
            pass_depth: Default::default(),
            plunge_rate: Default::default(),
            feed_rate: Default::default(),
            unit: UnitString::new_metric(),
        }
    }
}
