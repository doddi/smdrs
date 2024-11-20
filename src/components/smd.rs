use anathema::{component::Component, prelude::TuiBackend, runtime::RuntimeBuilder};

use super::{configuration, extras, metrics, quarantine};

struct App {}

impl Component for App {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component("smd", "src/templates/smd.aml", App {}, ())?;
    extras::register(runtime_builder)?;
    quarantine::register(runtime_builder)?;
    metrics::register(runtime_builder)?;
    configuration::register(runtime_builder)?;

    Ok(())
}
