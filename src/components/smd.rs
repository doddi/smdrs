use anathema::{component::Component, prelude::TuiBackend, runtime::RuntimeBuilder};
use smol::channel::Sender;

use crate::core::{component_bucket, middleware::FirewalClientMessageHandler};

use self::component_bucket::ComponentBucket;

use super::{configuration, extras, metrics, quarantine};

struct App {}

impl Component for App {
    type State = ();

    type Message = ();
}

pub(crate) fn register(
    runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    tx: Sender<FirewalClientMessageHandler>,
    component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    runtime_builder.register_component("smd", "src/templates/smd.aml", App {}, ())?;
    extras::register(runtime_builder)?;
    quarantine::register(runtime_builder, tx.clone(), component_bucket)?;
    metrics::register(runtime_builder)?;
    configuration::register(runtime_builder)?;

    Ok(())
}
