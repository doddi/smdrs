use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct RepositoryList {}

impl Component for RepositoryList {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "repository_list",
        "src/templates/configuration/repository/repository_list.aml",
        RepositoryList::default(),
        (),
    )?;
    Ok(())
}
