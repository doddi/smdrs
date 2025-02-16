use anathema::runtime::Builder;
use smol::channel::Sender;

use crate::core::{component_bucket::ComponentBucket, middleware::FirewalClientMessageHandler};

mod quarantine_dashboard;
mod quarantine_header;
mod quarantine_row;
pub(crate) mod quarantine_table;

pub(crate) fn register(
    runtime_builder: &mut Builder<()>,
    tx: Sender<FirewalClientMessageHandler>,
    component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    quarantine_dashboard::register(runtime_builder)?;
    quarantine_table::register(runtime_builder, tx.clone(), component_bucket)?;
    quarantine_row::register(runtime_builder)?;
    quarantine_header::register(runtime_builder)?;
    Ok(())
}
