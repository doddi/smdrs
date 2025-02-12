use anathema::runtime::Builder;

mod repository_dashboard;
mod repository_list;
mod repository_manager_list;

pub(crate) fn register(runtime_builder: &mut Builder) -> anyhow::Result<()> {
    repository_dashboard::register(runtime_builder)?;
    repository_manager_list::register(runtime_builder)?;
    repository_list::register(runtime_builder)?;
    Ok(())
}
