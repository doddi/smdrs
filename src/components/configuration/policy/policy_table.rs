use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct PolicyTable {}

impl Component for PolicyTable {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "policy_table",
        "src/templates/configuration/policy/policy_table.aml",
        PolicyTable::default(),
        (),
    )?;
    Ok(())
}
