use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct RepositoryManagerList {}

impl Component for RepositoryManagerList {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "repository_manager_list",
        "src/templates/configuration/repository/repository_manager_list.aml",
        RepositoryManagerList::default(),
        (),
    )?;
    Ok(())
}
