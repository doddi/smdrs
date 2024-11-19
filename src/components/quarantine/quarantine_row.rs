use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct QuarantineRow {
    threat: u8,
    policy: String,
    quarantine_time: String,
    component_name: String,
    repositiry_name: String,
}

impl Component for QuarantineRow {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_prototype(
        "quarantine_row",
        "src/components/templates/quarantine/quarantine_row.aml",
        QuarantineRow::default,
        || (),
    )?;
    Ok(())
}
