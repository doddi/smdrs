use anathema::component::ComponentId;

use crate::components::metrics::metric_dashboard::MetricsDashboardMessage;
use crate::components::quarantine::quarantine_table::QuarantineTableMessage;

pub struct ComponentBucket {
    pub quarantine_table: Option<ComponentId<QuarantineTableMessage>>,
    pub metrics_dashboard: Option<ComponentId<MetricsDashboardMessage>>,
}

impl ComponentBucket {
    pub fn new() -> Self {
        Self {
            quarantine_table: None,
            metrics_dashboard: None,
        }
    }
}
