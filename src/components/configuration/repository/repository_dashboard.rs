use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct RepositoryDashboard {}

impl Component for RepositoryDashboard {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "repository_dashboard",
        "src/templates/configuration/repository/repository_dashboard.aml",
        RepositoryDashboard::default(),
        (),
    )?;
    Ok(())
}
