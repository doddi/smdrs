use anathema::{component::Component, runtime::Builder};

#[derive(Default)]
#[allow(dead_code)]
struct RepositoryList {}

impl Component for RepositoryList {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut Builder<()>) -> anyhow::Result<()> {
    runtime_builder.component(
        "repository_list",
        "src/templates/configuration/repository/repository_list.aml",
        RepositoryList::default(),
        (),
    )?;
    Ok(())
}
