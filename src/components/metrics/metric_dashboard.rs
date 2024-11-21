use anathema::{
    component::Component,
    prelude::*,
    runtime::RuntimeBuilder,
    state::{State, Value},
};
use smol::channel::Sender;
use tracing::trace;

use crate::core::middleware::FirewalClientMessageHandler;

use super::component_bucket::ComponentBucket;

#[allow(dead_code)]
struct MetricsDashboard {
    tx: Sender<FirewalClientMessageHandler>,
}

impl MetricsDashboard {
    pub(crate) fn new(tx: Sender<FirewalClientMessageHandler>) -> Self {
        Self { tx }
    }
}

impl Component for MetricsDashboard {
    type State = MetricsDashboardState;

    type Message = MetricsDashboardMessage;

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_focus(
        &mut self,
        _state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        trace!("Sending a dashboard metrics request from focus");
        let _ = self.tx.try_send(FirewalClientMessageHandler::Metrics);
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        match key.get_char() {
            Some(ch) => {
                if ch == 'r' || ch == 'R' {
                    trace!("Sending a dashbaord metrics request from key press");
                    let _ = self.tx.try_send(FirewalClientMessageHandler::Metrics);
                }
            }
            None => todo!(),
        }
    }

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        state.components_waived = Value::new(message.components_waived);
        state.components_auto_released = Value::new(message.components_auto_released);
        state.components_quarantined = Value::new(message.components_quarantined);
        state.supply_chain_attacks = Value::new(message.supply_chain_attacks);
        state.namespace_attacks = Value::new(message.namespace_attacks);
        state.components_auto_selected = Value::new(message.components_auto_selected);
    }
}

#[derive(State)]
pub struct MetricsDashboardState {
    pub supply_chain_attacks: Value<u64>,
    pub namespace_attacks: Value<u64>,
    pub components_quarantined: Value<u64>,
    pub components_auto_released: Value<u64>,
    pub components_auto_selected: Value<u64>,
    pub components_waived: Value<u64>,
}

#[derive(Debug)]
pub struct MetricsDashboardMessage {
    pub supply_chain_attacks: u64,
    pub namespace_attacks: u64,
    pub components_quarantined: u64,
    pub components_auto_released: u64,
    pub components_auto_selected: u64,
    pub components_waived: u64,
}

impl From<MetricsDashboardMessage> for MetricsDashboardState {
    fn from(value: MetricsDashboardMessage) -> Self {
        MetricsDashboardState {
            supply_chain_attacks: Value::new(value.supply_chain_attacks),
            namespace_attacks: Value::new(value.namespace_attacks),
            components_quarantined: Value::new(value.components_quarantined),
            components_auto_released: Value::new(value.components_auto_released),
            components_auto_selected: Value::new(value.components_auto_selected),
            components_waived: Value::new(value.components_waived),
        }
    }
}

pub(crate) fn register(
    runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    tx: Sender<FirewalClientMessageHandler>,
    component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    let component_id = runtime_builder.register_component(
        "metric_dashboard",
        "src/templates/metrics/metric_dashboard.aml",
        MetricsDashboard::new(tx),
        MetricsDashboardState {
            supply_chain_attacks: Value::new(1),
            namespace_attacks: Value::new(2),
            components_quarantined: Value::new(3),
            components_auto_released: Value::new(4),
            components_auto_selected: Value::new(5),
            components_waived: Value::new(6),
        },
    )?;

    component_bucket.metrics_dashboard = Some(component_id);
    Ok(())
}
