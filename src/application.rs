use anathema::{
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};
use smol::channel::Sender;

use crate::{
    components::{self},
    core::{
        component_bucket,
        middleware::{self, FirewalClientMessageHandler},
    },
};

use self::component_bucket::ComponentBucket;

pub(super) struct Application {}

impl Application {
    fn new() -> Self {
        Self {}
    }

    fn run(&self) -> anyhow::Result<()> {
        let tui = TuiBackend::builder()
            .enable_alt_screen()
            .enable_raw_mode()
            .hide_cursor()
            .finish()?;

        let (tx, rx) = smol::channel::unbounded::<FirewalClientMessageHandler>();

        let mut runtime_builder = Runtime::builder(Document::new("@smd"), tui);

        let mut component_bucket = ComponentBucket::new();

        self.register_components(&mut runtime_builder, tx, &mut component_bucket)?;

        middleware::Middleware::serve(runtime_builder.emitter(), rx, component_bucket);

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

pub(super) fn start() -> anyhow::Result<()> {
    Application::new().run()
}
