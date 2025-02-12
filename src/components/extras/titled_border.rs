use anathema::component::{Children, Context};
use anathema::state::AnyState;
use anathema::{
    component::Component,
    runtime::Builder,
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
        true
    }

    fn on_blur(
        &mut self,
        _state: &mut Self::State,
        mut _elements: Children<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        // *state.active.to_mut() = 0;
    }

    fn on_focus(
        &mut self,
        _state: &mut Self::State,
        mut _elements: Children<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        // *state.active.to_mut() = 1;
    }

    fn receive(
        &mut self,
        ident: &str,
        _value: &dyn AnyState,
        state: &mut Self::State,
        mut _elements: Children<'_, '_>,
        mut _context: Context<'_, Self::State>,
    ) {
        if ident == "got_focus" {
            *state.active.to_mut() = 1;
        } else if ident == "lost_focus" {
            *state.active.to_mut() = 0;
        }
    }
}

#[derive(State)]
pub struct TitledBorderState {
    active: Value<u8>,
}

pub(crate) fn register(runtime_builder: &mut Builder) -> anyhow::Result<()> {
    runtime_builder.prototype(
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
