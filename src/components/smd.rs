use anathema::{
    component::{Component, KeyCode, KeyEvent},
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::{State, Value},
};
use smol::channel::Sender;
use tracing::debug;

use crate::core::{component_bucket, middleware::FirewalClientMessageHandler};

use self::component_bucket::ComponentBucket;

use super::{configuration, extras, metrics, quarantine};

struct App {}

impl Component for App {
    type State = AppState;

    type Message = ();

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        // TODO: Maybe these should be part of the global event handler
        match key {
            KeyEvent {
                code: KeyCode::Char('h'),
                ctrl: _,
                state: _,
            } => *state.page.to_mut() = "home".to_string(),
            KeyEvent {
                code: KeyCode::Char('m'),
                ctrl: _,
                state: _,
            } => *state.page.to_mut() = "metrics".to_string(),
            KeyEvent {
                code: KeyCode::Char('q'),
                ctrl: _,
                state: _,
            } => *state.page.to_mut() = "quarantine".to_string(),
            val => debug!("Invalid key {:?} pressed", val),
        };
    }

    fn accept_focus(&self) -> bool {
        true
    }
}

#[derive(State)]
pub(crate) struct AppState {
    page: Value<String>,
}

pub(crate) fn register(
    runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>,
    tx: Sender<FirewalClientMessageHandler>,
    component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    runtime_builder.register_component(
        "smd",
        "src/templates/smd.aml",
        App {},
        AppState {
            page: Value::new("homepage".to_string()),
        },
    )?;

    extras::register(runtime_builder)?;
    quarantine::register(runtime_builder, tx.clone(), component_bucket)?;
    metrics::register(runtime_builder, tx.clone(), component_bucket)?;
    configuration::register(runtime_builder)?;

    Ok(())
}
