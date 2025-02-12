use anathema::runtime::Builder;

mod policy;
mod repository;

pub(crate) fn register(runtime_builder: &mut Builder) -> anyhow::Result<()> {
    repository::register(runtime_builder)?;

    policy::register(runtime_builder)?;
    Ok(())
}
