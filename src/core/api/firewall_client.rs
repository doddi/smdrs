use anyhow::anyhow;
use reqwest::Url;
use smol::channel::Receiver;
use tracing::{debug, trace, warn};

use crate::core::{
    api::quarantine_message::QuarantinedComponentResponse, middleware::FirewalClientMessageHandler,
};

use super::quarantine_message::QuarantinedComponentRequestList;

pub(crate) struct FirewallClient {
    url: Url,
    http_client: reqwest::blocking::Client,
    rx: smol::channel::Receiver<FirewalClientMessageHandler>,
}

impl FirewallClient {
    pub(crate) fn new(url: Url, rx: Receiver<FirewalClientMessageHandler>) -> Self {
        Self {
            url,
            http_client: reqwest::blocking::Client::new(),
            rx,
        }
    }

    pub(crate) async fn get_quarantined_components(
        &self,
        request: QuarantinedComponentRequestList,
    ) -> anyhow::Result<QuarantinedComponentResponse> {
        trace!(
            "Sending get_quarantined_component request to Firewall: {}",
            self.url.clone()
        );

        let builder = self
            .http_client
            .get(
                self.url
                    .clone()
                    .join("/api/v2/firewall/components/quarantined")
                    .expect(""),
            )
            .header(
                "Authorization",
                "Basic ".to_owned() + "YWRtaW46YWRtaW4xMjM=",
            );

        match builder.send() {
            Ok(response) => {
                if !response.status().is_success() {
                    trace!("Bad response status: {}", response.status());
                    return Err(anyhow!("Bad response status: {}", response.status()));
                }

                match response.json::<QuarantinedComponentResponse>() {
                    Ok(data) => {
                        debug!("Decoded quarantine response");
                        Ok(data)
                    }
                    Err(err) => {
                        trace!("Unable to parse: {}", err);
                        Err(anyhow!("Unable to parse resonse: {}", err))
                    }
                }
            }
            Err(err) => {
                warn!("Error from backend {}", err);
                Err(anyhow!(
                    "Failed to send quarantine component request to backend: {}",
                    err
                ))
            }
        }
    }
}
