use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

use super::metric_card;

#[derive(Default)]
#[allow(dead_code)]
struct MetricDashboard {}

impl Component for MetricDashboard {
    type State = ();

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_prototype(
        "metric_dashboard",
        "src/templates/metrics/metric_dashboard.aml",
        MetricDashboard::default,
        || (),
    )?;
    metric_card::register(runtime_builder)?;
    Ok(())
}
