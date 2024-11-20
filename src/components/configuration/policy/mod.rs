mod policy_dashboard;
mod policy_header;
mod policy_row;
mod policy_table;

use anathema::{prelude::TuiBackend, runtime::RuntimeBuilder};

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    policy_dashboard::register(runtime_builder)?;
    policy_table::register(runtime_builder)?;
    policy_header::register(runtime_builder)?;
    policy_row::register(runtime_builder)?;
    Ok(())
}
