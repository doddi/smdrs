use anathema::{prelude::TuiBackend, runtime::RuntimeBuilder};
use smol::channel::Sender;

use crate::core::{component_bucket, middleware::FirewalClientMessageHandler};

use self::component_bucket::ComponentBucket;

mod metric_card;
pub(crate) mod metric_dashboard;

pub(crate) fn register(
    runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    tx: Sender<FirewalClientMessageHandler>,
    component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    metric_dashboard::register(runtime_builder, tx.clone(), component_bucket)?;
    metric_card::register(runtime_builder, component_bucket)?;
    Ok(())
}
