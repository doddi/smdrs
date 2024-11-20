use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct PolicyHeader {}

impl Component for PolicyHeader {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "policy_header",
        "src/templates/configuration/policy/policy_header.aml",
        PolicyHeader::default(),
        (),
    )?;
    Ok(())
}
