use std::collections::HashMap;

use super::{
    quarantine_message::QuarantinedComponentData, FirewallDashboardMetric,
    FirewallDashboardMetrics, QuarantinedComponentResponse, SMDClient,
};

pub(crate) struct DummpyClient {}

impl DummpyClient {
    fn add_metric(
        &self,
        map: &mut HashMap<String, FirewallDashboardMetric>,
        item: String,
        value: u64,
    ) {
        map.insert(
            item,
            FirewallDashboardMetric {
                firewall_metrics_value: value,
                latest_updated_time: None,
            },
        );
    }
}

impl SMDClient for DummpyClient {
    fn get_quarantined_components(
        &self,
        _request: super::QuarantinedComponentRequestList,
    ) -> anyhow::Result<super::QuarantinedComponentResponse> {
        let data = QuarantinedComponentData {
            threat_level: 7,
            policy_name: "dummy policy".to_string(),
            display_name: "dummy name".to_string(),
            repository_name: "dummrepo".to_string(),
            quarantine_date: "11-10-2025 11:00:00".to_string(),
            quarantined: true,
        };
        let response = QuarantinedComponentResponse {
            total: 1,
            page: 1,
            page_size: 1,
            page_count: 1,
            results: vec![data],
        };
        Ok(response)
    }

    fn get_dashboard_metrics(&self) -> anyhow::Result<super::FirewallDashboardMetrics> {
        let mut map: HashMap<String, FirewallDashboardMetric> = HashMap::new();
        self.add_metric(&mut map, "SUPPLY_CHAIN_ATTACKS_BLOCKED".to_string(), 1);
        self.add_metric(&mut map, "NAMESPACE_ATTACKS_BLOCKED".to_string(), 2);
        self.add_metric(&mut map, "COMPONENTS_QUARANTINED".to_string(), 3);
        self.add_metric(&mut map, "WAIVED_COMPONENTS".to_string(), 4);
        self.add_metric(&mut map, "COMPONENTS_AUTO_RELEASED".to_string(), 7);
        self.add_metric(
            &mut map,
            "SAFE_VERSIONS_SELECTED_AUTOMATICALLY".to_string(),
            8,
        );

        let response = FirewallDashboardMetrics { map };
        Ok(response)
    }
}
