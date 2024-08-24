pub struct UnitString {
    measure: &'static str,
    feedrate: &'static str,
    optimization: &'static str,
}

impl UnitString {
    pub fn new_metric() -> Self {
        Self {
            measure: "mm",
            feedrate: "mm/min",
            optimization: "mm",
        }
    }

    pub fn new_imperial() -> Self {
        Self {
            measure: "inch",
            feedrate: "inch/min",
            optimization: "mils",
        }
    }

    pub fn measure(&self) -> &str {
        &self.measure
    }

    pub fn feedrate(&self) -> &str {
        &self.feedrate
    }

    pub fn optimization(&self) -> &str {
        &self.optimization
    }
}
