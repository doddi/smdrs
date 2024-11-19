use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

struct QuarantineTable {}

impl Component for QuarantineTable {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "quarantine_table",
        "src/components/templates/quarantine/quarantine_table.aml",
        QuarantineTable {},
        (),
    )?;
    Ok(())
}
