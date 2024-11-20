use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

use super::{policy_header, policy_row, policy_table};

#[derive(Default)]
#[allow(dead_code)]
struct PolicyDashboard {}

impl Component for PolicyDashboard {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "policy_dashboard",
        "src/templates/configuration/policy/policy_dashboard.aml",
        PolicyDashboard::default(),
        (),
    )?;

    policy_table::register(runtime_builder)?;
    policy_header::register(runtime_builder)?;
    policy_row::register(runtime_builder)?;
    Ok(())
}
