use anathema::{
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};
use reqwest::Url;
use smol::channel::Sender;

use crate::{
    components::{self},
    core::{
        api::{dummy_client::DummpyClient, firewall_client::FirewallClient, SMDClient},
        component_bucket,
        middleware::{self, FirewalClientMessageHandler},
    },
    ClientType,
};

use self::component_bucket::ComponentBucket;

pub(super) struct Application {}

impl Application {
    fn new() -> Self {
        Self {}
    }

    fn run(&self, client_type: ClientType) -> anyhow::Result<()> {
        let tui = TuiBackend::builder()
            .enable_alt_screen()
            .enable_raw_mode()
            .hide_cursor()
            .finish()?;

        let (tx, rx) = smol::channel::unbounded::<FirewalClientMessageHandler>();

        let mut runtime_builder = Runtime::builder(Document::new("@smd"), tui);

        let mut component_bucket = ComponentBucket::new();

        self.register_components(&mut runtime_builder, tx, &mut component_bucket)?;

        let client: Box<dyn SMDClient> = match client_type {
            ClientType::Dummy => Box::new(DummpyClient {}),
            ClientType::Firewall => {
                let url: Url = Url::parse("http://localhost:8070").expect("valid url");
                let client = FirewallClient::new(url);
                Box::new(client)
            }
        };

        middleware::Middleware::serve(runtime_builder.emitter(), rx, component_bucket, client);

        let mut runtime = runtime_builder.finish()?;
        runtime.run();

        Ok(())
    }

    fn register_components(
        &self,
        runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
        tx: Sender<FirewalClientMessageHandler>,
        component_bucket: &mut ComponentBucket,
    ) -> anyhow::Result<()> {
        components::smd::register(runtime_builder, tx.clone(), component_bucket)?;
        Ok(())
    }
}

pub(super) fn start(client_type: ClientType) -> anyhow::Result<()> {
    Application::new().run(client_type)
}
