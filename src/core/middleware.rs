use std::collections::HashMap;

use anathema::component::ComponentId;
use reqwest::Url;
use smol::channel::{Receiver, Sender};
use tracing::trace;

use self::component_bucket::ComponentBucket;

use super::api::firewall_client::{
    FirewallClient, FirewallDashboardMetric, FirewallDashboardMetrics,
};
use super::api::quarantine_message::{
    QuarantinedComponentData, QuarantinedComponentRequestList, QuarantinedComponentResponse,
};
use super::component_bucket;
use crate::components::metrics::metric_dashboard::MetricsDashboardMessage;
use crate::components::quarantine::quarantine_table::{
    QuarantineRowMessage, QuarantineTableMessage,
};

pub(crate) struct Middleware {
    client: FirewallClient,
    tx: Sender<FirewalClientMessageHandler>,
}

impl Middleware {
    pub(crate) fn new() -> Self {
        let (tx, rx) = smol::channel::unbounded::<FirewalClientMessageHandler>();
        let url = Url::parse("http://192.168.1.181:8070/").expect("valid url");

        Self {
            client: FirewallClient::new(url, rx),
            tx,
        }
    }

    pub async fn get_quarantined_components(
        &self,
        request: QuarantinedComponentRequestList,
    ) -> anyhow::Result<QuarantinedComponentResponse> {
        self.client.get_quarantined_components(request).await
    }

    pub async fn get_metrics(&self) -> anyhow::Result<FirewallDashboardMetrics> {
        self.client.get_dashboard_metrics().await
    }

    pub(crate) fn serve(
        emitter: anathema::component::Emitter,
        rx: Receiver<FirewalClientMessageHandler>,
        component_bucket: ComponentBucket,
    ) {
        trace!("Starting middleware listener");
        smol::spawn(async move {
            let middleware = Middleware::new();
            trace!("Started middleware listener");

            while let Ok(message) = rx.recv().await {
                trace!("Received firewall request: '{:?}' from component", message);

                match message {
                    FirewalClientMessageHandler::GetQuarantinedComponents(request) => {
                        match middleware.get_quarantined_components(request).await {
                            Ok(response) => {
                                let component_id =
                                    component_bucket.quarantine_table.expect("quarantine_table");
                                let _ = emitter.emit(component_id, response.into());
                            }
                            Err(_) => panic!(),
                        }
                    }
                    FirewalClientMessageHandler::Metrics => match middleware.get_metrics().await {
                        Ok(response) => {
                            let component_id = component_bucket
                                .metrics_dashboard
                                .expect("metrics dashboard id");
                            let _ = emitter.emit(component_id, response.into());
                        }
                        Err(_) => todo!(),
                    },
                }
            }
            trace!("middleware listener finised");
        })
        .detach();
    }
}

impl From<FirewallDashboardMetrics> for MetricsDashboardMessage {
    fn from(value: FirewallDashboardMetrics) -> Self {
        trace!(
            "converting received value for Dashboard metrics: {:?}",
            value
        );
        let converted = MetricsDashboardMessage {
            supply_chain_attacks: get_from_metric_map(&value.map, "SUPPLY_CHAIN_ATTACKS_BLOCKED"),
            namespace_attacks: get_from_metric_map(&value.map, "NAMESPACE_ATTACKS_BLOCKED"),
            components_quarantined: get_from_metric_map(&value.map, "COMPONENTS_QUARANTINED"),
            components_auto_released: get_from_metric_map(&value.map, "COMPONENTS_AUTO_RELEASED"),
            components_auto_selected: get_from_metric_map(
                &value.map,
                "SAFE_VERSIONS_SELECTED_AUTOMATICALLY",
            ),
            components_waived: get_from_metric_map(&value.map, "WAIVED_COMPONENTS"),
        };
        trace!(
            "converted received value for Dashboard metrics: {:?}",
            converted
        );
        converted
    }
}

fn get_from_metric_map(map: &HashMap<String, FirewallDashboardMetric>, item: &'static str) -> u64 {
    map.get(item)
        .expect("metric available")
        .firewall_metrics_value
}

impl From<QuarantinedComponentResponse> for QuarantineTableMessage {
    fn from(value: QuarantinedComponentResponse) -> Self {
        let row_count = value.results.len();
        let rows = value
            .results
            .into_iter()
            .map(|row| QuarantineRowMessage {
                threat: row.threat_level,
                policy_name: row.policy_name.clone(),
                quarantine_time: row.quarantine_date,
                component_name: row.display_name,
                repository_name: row.repository_name,
            })
            .collect();
        QuarantineTableMessage { row_count, rows }
    }
}

#[derive(Debug)]
pub(crate) enum FirewalClientMessageHandler {
    GetQuarantinedComponents(QuarantinedComponentRequestList),
    Metrics,
}
