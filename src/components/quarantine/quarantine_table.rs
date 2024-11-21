use anathema::{
    component::Component,
    prelude::*,
    runtime::RuntimeBuilder,
    state::{List, State, Value},
};
use smol::channel::Sender;
use tracing::trace;

use crate::core::{
    api::quarantine_message::{OrderBy, QuarantineOrderBy, QuarantinedComponentRequestList},
    component_bucket::ComponentBucket,
    middleware::FirewalClientMessageHandler,
};

struct QuarantineTable {
    tx: Sender<FirewalClientMessageHandler>,
}

impl QuarantineTable {
    pub(crate) fn new(tx: Sender<FirewalClientMessageHandler>) -> Self {
        Self { tx }
    }
}

impl Component for QuarantineTable {
    type State = QuarantineTableState;

    type Message = QuarantineTableMessage;

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
                    trace!("Sending a quarantine component request from key press");
                    let _ =
                        self.tx
                            .try_send(FirewalClientMessageHandler::GetQuarantinedComponents(
                                QuarantinedComponentRequestList {
                                    page: Some(0),
                                    page_size: Some(100),
                                    sort_by: Some(QuarantineOrderBy::Component(OrderBy::Ascending)),
                                },
                            ));
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
        state.rows = List::empty();
        trace!("received count of {}", message.row_count);
        for ele in message.rows {
            let row: QuarantineRowState = ele.into();
            state.rows.push(row);
        }
    }

    fn receive(
        &mut self,
        _ident: &str,
        _value: anathema::state::CommonVal<'_>,
        _state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
    }

    fn accept_focus(&self) -> bool {
        true
    }
}

pub(crate) fn register(
    runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    tx: Sender<FirewalClientMessageHandler>,
    bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    let component_id = runtime_builder.register_component(
        "quarantine_table",
        "src/templates/quarantine/quarantine_table.aml",
        QuarantineTable::new(tx),
        QuarantineTableState::new(),
    )?;

    bucket.quarantine_table = Some(component_id);
    Ok(())
}

#[derive(State)]
pub struct QuarantineTableState {
    rows: Value<List<QuarantineRowState>>,
}

impl QuarantineTableState {
    fn new() -> Self {
        Self {
            rows: Self::build_initial_rows(),
        }
    }

    fn build_initial_rows() -> Value<List<QuarantineRowState>> {
        let mut list = List::empty();

        for ele in 0..4 {
            let row = QuarantineRowState {
                threat: Value::new(10 - ele),
                policy_name: Value::new("mine".to_string()),
                quarantine_time: Value::new("some time".to_string()),
                component_name: Value::new("component".to_string()),
                repository_name: Value::new("npm.js".to_string()),
            };
            list.push(row);
        }
        list
    }
}

#[derive(State, Debug)]
pub struct QuarantineRowState {
    pub threat: Value<u8>,
    pub policy_name: Value<String>,
    pub quarantine_time: Value<String>,
    pub component_name: Value<String>,
    pub repository_name: Value<String>,
}

impl From<QuarantineRowMessage> for QuarantineRowState {
    fn from(value: QuarantineRowMessage) -> Self {
        QuarantineRowState {
            threat: Value::new(value.threat),
            policy_name: Value::new(value.policy_name),
            quarantine_time: Value::new(value.quarantine_time),
            component_name: Value::new(value.component_name),
            repository_name: Value::new(value.repository_name),
        }
    }
}

#[derive(Debug)]
pub struct QuarantineTableMessage {
    pub row_count: usize,
    pub rows: Vec<QuarantineRowMessage>,
}

#[derive(Debug)]
pub struct QuarantineRowMessage {
    pub threat: u8,
    pub policy_name: String,
    pub quarantine_time: String,
    pub component_name: String,
    pub repository_name: String,
}
