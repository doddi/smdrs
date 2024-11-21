use anathema::component::ComponentId;
use reqwest::Url;
use smol::channel::{Receiver, Sender};
use tracing::trace;

use self::component_bucket::ComponentBucket;

use super::api::firewall_client::FirewallClient;
use super::api::quarantine_message::{
    QuarantinedComponentData, QuarantinedComponentRequestList, QuarantinedComponentResponse,
};
use super::component_bucket;
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
                trace!("Received firewall request: {:?} from component", message);

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
                }
            }
            trace!("middleware listener finised");
        })
        .detach();
    }
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
}
