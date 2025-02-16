use std::u8;

use anathema::component::{Children, KeyEvent};
use anathema::{
    component::Component,
    prelude::*,
    runtime::Builder,
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

    fn on_focus(
        &mut self,
        _state: &mut Self::State,
        mut _elements: Children<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        trace!("Quarantintable got focus");
        let _ = self
            .tx
            .try_send(FirewalClientMessageHandler::GetQuarantinedComponents(
                QuarantinedComponentRequestList {
                    page: Some(0),
                    page_size: Some(100),
                    sort_by: Some(QuarantineOrderBy::Component(OrderBy::Ascending)),
                },
            ));
    }

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        mut _elements: Children<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        let current_row: usize = state.active_row.copy_value().into();
        let number_of_rows = state.rows.len();

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
                } else if ch == 'j' || ch == 'J' {
                    trace!("Next row");
                    if current_row == number_of_rows - 1 {
                        *state.active_row.to_mut() = 0;
                    } else {
                        *state.active_row.to_mut() = (current_row + 1) as u8;
                    }
                } else if ch == 'k' || ch == 'K' {
                    trace!("Previous row");
                    if current_row == 0 {
                        *state.active_row.to_mut() = (number_of_rows - 1) as u8;
                    } else {
                        *state.active_row.to_mut() = (current_row - 1) as u8;
                    }
                }
            }
            None => todo!(),
        }
    }

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _elements: Children<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        trace!("received quarantine table update message");
        state.rows = List::empty().into();

        *state.page_count.to_mut() = message.page_count;
        *state.page_size.to_mut() = message.page_size;
        *state.page.to_mut() = message.page;
        *state.total.to_mut() = message.total;
        for ele in message.rows {
            let row: QuarantineRowState = ele.into();
            state.rows.push(row);
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}

pub(crate) fn register(
    runtime_builder: &mut Builder<()>,
    tx: Sender<FirewalClientMessageHandler>,
    bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    let component_id = runtime_builder.component(
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
    active_row: Value<u8>,

    total: Value<usize>,
    page: Value<usize>,
    page_size: Value<usize>,
    page_count: Value<usize>,
    rows: Value<List<QuarantineRowState>>,
}

impl QuarantineTableState {
    fn new() -> Self {
        Self {
            active_row: Value::new(0),

            total: Value::new(4),
            page: Value::new(1),
            page_size: Value::new(4),
            page_count: Value::new(1),
            rows: Self::build_initial_rows(4),
        }
    }

    fn build_initial_rows(count: usize) -> Value<List<QuarantineRowState>> {
        let mut list = Value::empty();

        for ele in 0..count {
            let row = QuarantineRowState {
                threat: Value::new(10 - ele as u8),
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
    pub total: usize,
    pub page: usize,
    pub page_size: usize,
    pub page_count: usize,

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
