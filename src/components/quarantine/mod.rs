use anathema::{prelude::TuiBackend, runtime::RuntimeBuilder};

mod quarantine_header;
mod quarantine_row;
mod quarantine_table;

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    quarantine_table::register(runtime_builder)?;
    quarantine_row::register(runtime_builder)?;
    quarantine_header::register(runtime_builder)?;
    Ok(())
}
