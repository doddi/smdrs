use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct PolicyRow {}

impl Component for PolicyRow {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_prototype(
        "policy_row",
        "src/templates/configuration/policy/policy_row.aml",
        PolicyRow::default,
        || (),
    )?;
    Ok(())
}
