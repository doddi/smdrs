use anathema::{
    component::Component,
    prelude::TuiBackend,
    runtime::RuntimeBuilder,
    state::{State, Value},
};

struct TitledBorder {
    #[allow(dead_code)]
    title: String,
}

impl Component for TitledBorder {
    type State = TitledBorderState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_blur(
        &mut self,
        _state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        // *state.active.to_mut() = 0;
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut _elements: anathema::widgets::Elements<'_, '_>,
        mut _context: anathema::prelude::Context<'_, Self::State>,
    ) {
        *state.active.to_mut() = 1;
    }
}

#[derive(State)]
pub struct TitledBorderState {
    active: Value<u8>,
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_prototype(
        "titled_border",
        "src/templates/extras/titled_border.aml",
        || TitledBorder {
            title: "".to_string(),
        },
        || TitledBorderState {
            active: Value::new(0),
        },
    )?;
    Ok(())
}
