use anathema::{
    prelude::{Document, TuiBackend},
    runtime::{Runtime, RuntimeBuilder},
};

use crate::components::{self};

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

        let mut runtime_builder = Runtime::builder(Document::new("@smd"), tui);
        self.register_components(&mut runtime_builder)?;
        let mut runtime = runtime_builder.finish()?;
        runtime.run();

        Ok(())
    }

    fn register_components(
        &self,
        runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    ) -> anyhow::Result<()> {
        components::smd::register(runtime_builder)?;
        Ok(())
    }
}

pub(super) fn start() -> anyhow::Result<()> {
    Application::new().run()
}
