use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

struct QuarantineDashboard {}

impl Component for QuarantineDashboard {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "quarantine_dashboard",
        "src/templates/quarantine/quarantine_dashboard.aml",
        QuarantineDashboard {},
        (),
    )?;
    Ok(())
}
