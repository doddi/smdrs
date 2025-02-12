use anathema::{component::Component, runtime::Builder};

struct QuarantineDashboard {}

impl Component for QuarantineDashboard {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut Builder) -> anyhow::Result<()> {
    runtime_builder.component(
        "quarantine_dashboard",
        "src/templates/quarantine/quarantine_dashboard.aml",
        QuarantineDashboard {},
        (),
    )?;
    Ok(())
}
