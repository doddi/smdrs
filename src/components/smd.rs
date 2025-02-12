use anathema::component::{Children, Context};
use anathema::runtime::Builder;
use anathema::{
    component::{Component, KeyCode, KeyEvent},
    state::{State, Value},
};
use smol::channel::Sender;
use tracing::{debug, trace};

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
        mut _elements: Children<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        trace!("page is: {:?}", *state.page.to_ref());
        trace!("on_key called");
        // TODO: Maybe these should be part of the global event handler
        match key {
            KeyEvent {
                code: KeyCode::Char('h'),
                ctrl: _,
                state: _,
            } => *state.page.to_mut() = "homepage".to_string(),
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
    runtime_builder: &mut Builder,
    tx: Sender<FirewalClientMessageHandler>,
    component_bucket: &mut ComponentBucket,
) -> anyhow::Result<()> {
    runtime_builder.component(
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
