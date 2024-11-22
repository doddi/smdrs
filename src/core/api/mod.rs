use std::collections::HashMap;

use quarantine_message::{QuarantinedComponentRequestList, QuarantinedComponentResponse};
use serde::Deserialize;

pub(crate) mod dummy_client;
pub(crate) mod firewall_client;
pub(crate) mod quarantine_message;

pub(crate) trait SMDClient: Send + Sync {
    fn get_quarantined_components(
        &self,
        request: QuarantinedComponentRequestList,
    ) -> anyhow::Result<QuarantinedComponentResponse>;

    fn get_dashboard_metrics(&self) -> anyhow::Result<FirewallDashboardMetrics>;
}

#[derive(Debug, Deserialize)]
pub struct FirewallDashboardMetrics {
    pub map: HashMap<String, FirewallDashboardMetric>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct FirewallDashboardMetric {
    #[serde(rename = "firewallMetricsValue")]
    pub firewall_metrics_value: u64,
    #[serde(rename = "latestUpdatedTime")]
    latest_updated_time: Option<String>,
}
