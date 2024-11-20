use anathema::{prelude::TuiBackend, runtime::RuntimeBuilder};

mod metric_card;
mod metric_dashboard;

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    metric_dashboard::register(runtime_builder)?;
    metric_card::register(runtime_builder)?;
    Ok(())
}
