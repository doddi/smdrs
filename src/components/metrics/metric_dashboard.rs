use anathema::{component::Component, prelude::*, runtime::RuntimeBuilder};

#[derive(Default)]
#[allow(dead_code)]
struct MetricDashboard {}

impl Component for MetricDashboard {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_prototype(
        "metric_dashboard",
        "src/components/templates/metrics/metric_dashboard.aml",
        MetricDashboard::default,
        || (),
    )?;
    Ok(())
}
