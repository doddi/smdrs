use anathema::runtime::Builder;

mod menu_item;
mod titled_border;

pub(crate) fn register(runtime_builder: &mut Builder<()>) -> anyhow::Result<()> {
    titled_border::register(runtime_builder)?;
    menu_item::register(runtime_builder)?;

    Ok(())
}
