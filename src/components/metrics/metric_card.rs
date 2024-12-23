use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

use self::component_bucket::ComponentBucket;

use super::component_bucket;

#[derive(Default)]
#[allow(dead_code)]
struct MetricCard {
    title: String,
    value: u64,
    legend: String,
}

impl Component for MetricCard {
    type State = ();

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}

pub(crate) fn register(
    runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    _component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    runtime_builder.register_prototype(
        "metric_card",
        "src/templates/metrics/metric_card.aml",
        MetricCard::default,
        || (),
    )?;
    Ok(())
}
