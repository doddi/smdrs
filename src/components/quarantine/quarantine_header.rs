use anathema::{component::Component, runtime::Builder};

struct QuarantineHeader {}

impl Component for QuarantineHeader {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut Builder<()>) -> anyhow::Result<()> {
    runtime_builder.component(
        "quarantine_header",
        "src/templates/quarantine/quarantine_header.aml",
        QuarantineHeader {},
        (),
    )?;
    Ok(())
}
