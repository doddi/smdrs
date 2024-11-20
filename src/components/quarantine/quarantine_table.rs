use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

use super::{quarantine_header, quarantine_row};

struct QuarantineTable {}

impl Component for QuarantineTable {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "quarantine_table",
        "src/templates/quarantine/quarantine_table.aml",
        QuarantineTable {},
        (),
    )?;
    quarantine_header::register(runtime_builder)?;
    quarantine_row::register(runtime_builder)?;
    Ok(())
}
