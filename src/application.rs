use anathema::{
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};

use crate::components::{self, metrics};

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

        let mut runtime_builder = Runtime::builder(Document::new("@app"), tui);
        self.register_components(&mut runtime_builder)?;
        let mut runtime = runtime_builder.finish()?;
        runtime.run();

        Ok(())
    }

    fn register_components(
        &self,
        runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        components::app::register(runtime_builder)?;
        components::titled_border::register(runtime_builder)?;
        components::quarantine::quarantine_table::register(runtime_builder)?;
        components::quarantine::quarantine_header::register(runtime_builder)?;
        components::quarantine::quarantine_row::register(runtime_builder)?;

        components::metrics::metric_card::register(runtime_builder)?;
        components::metrics::metric_dashboard::register(runtime_builder)?;

        Ok(())
    }
}

pub(super) fn start() -> anyhow::Result<()> {
    Application::new().run()
}
