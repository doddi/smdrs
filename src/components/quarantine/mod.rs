use anathema::{prelude::TuiBackend, runtime::RuntimeBuilder};
use smol::channel::Sender;

use crate::core::{component_bucket::ComponentBucket, middleware::FirewalClientMessageHandler};

mod quarantine_header;
mod quarantine_row;
pub(crate) mod quarantine_table;

pub(crate) fn register(
    runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    tx: Sender<FirewalClientMessageHandler>,
    component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    quarantine_table::register(runtime_builder, tx.clone(), component_bucket)?;
    quarantine_row::register(runtime_builder)?;
    quarantine_header::register(runtime_builder)?;
    Ok(())
}
