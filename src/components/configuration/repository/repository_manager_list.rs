use anathema::{component::Component, runtime::Builder};

#[derive(Default)]
#[allow(dead_code)]
struct RepositoryManagerList {}

impl Component for RepositoryManagerList {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut Builder<()>) -> anyhow::Result<()> {
    runtime_builder.component(
        "repository_manager_list",
        "src/templates/configuration/repository/repository_manager_list.aml",
        RepositoryManagerList::default(),
        (),
    )?;
    Ok(())
}
