use anathema::{component::Component, runtime::Builder};
use crate::core::component_bucket::ComponentBucket;

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
    runtime_builder: &mut Builder,
    _component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    runtime_builder.prototype(
        "metric_card",
        "src/templates/metrics/metric_card.aml",
        MetricCard::default,
        || (),
    )?;
    Ok(())
}
