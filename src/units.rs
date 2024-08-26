static METRIC_MEASURE: &str = "mm";
static METRIC_FEEDRATE: &str = "mm/minute";
static METRIC_OPTIMIZATION: &str = "mm";

static IMPERIAL_MEASURE: &str = "inch";
static IMPERIAL_FEEDRATE: &str = "inch/minute";
static IMPERIAL_OPTIMIZATION: &str = "inch";

#[derive(Debug)]
pub struct UnitString {
    measure: &'static str,
    feedrate: &'static str,
    optimization: &'static str,
    is_metric: bool,
}

impl UnitString {
    pub fn new_metric() -> Self {
        Self {
            measure: METRIC_MEASURE,
            feedrate: METRIC_FEEDRATE,
            optimization: METRIC_OPTIMIZATION,
            is_metric: true,
        }
    }

    pub fn new_imperial() -> Self {
        Self {
            measure: IMPERIAL_MEASURE,
            feedrate: IMPERIAL_FEEDRATE,
            optimization: IMPERIAL_OPTIMIZATION,
            is_metric: false,
        }
    }

    pub fn measure(&self) -> &str {
        self.measure
    }

    pub fn feedrate(&self) -> &str {
        self.feedrate
    }

    pub fn optimization(&self) -> &str {
        self.optimization
    }

    pub fn is_metric(&self) -> bool {
        self.is_metric
    }
}
