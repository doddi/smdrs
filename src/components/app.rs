use anathema::{component::Component, prelude::TuiBackend, runtime::RuntimeBuilder};

struct App {}

impl Component for App {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component("app", "src/components/templates/app.aml", App {}, ())?;
    Ok(())
}
