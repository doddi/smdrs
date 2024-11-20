use anathema::{prelude::TuiBackend, runtime::RuntimeBuilder};

mod menu_item;
mod titled_border;

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    titled_border::register(runtime_builder)?;
    menu_item::register(runtime_builder)?;

    Ok(())
}
