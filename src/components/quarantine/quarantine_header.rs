use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

struct QuarantineHeader {}

impl Component for QuarantineHeader {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "quarantine_header",
        "src/components/templates/quarantine/quarantine_header.aml",
        QuarantineHeader {},
        (),
    )?;
    Ok(())
}
