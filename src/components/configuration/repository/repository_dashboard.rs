use anathema::{component::Component, runtime::Builder};

#[derive(Default)]
#[allow(dead_code)]
struct RepositoryDashboard {}

impl Component for RepositoryDashboard {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut Builder) -> anyhow::Result<()> {
    runtime_builder.component(
        "repository_dashboard",
        "src/templates/configuration/repository/repository_dashboard.aml",
        RepositoryDashboard::default(),
        (),
    )?;
    Ok(())
}
