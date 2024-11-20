use anathema::{prelude::TuiBackend, runtime::RuntimeBuilder};

mod policy;
mod repository;

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    repository::register(runtime_builder)?;

    policy::register(runtime_builder)?;
    Ok(())
}
