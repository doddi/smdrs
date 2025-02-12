use anathema::{
    component::Component,
    runtime::Builder,
    state::{State, Value},
};

struct MenuItem {
    #[allow(dead_code)]
    description: String,
}

impl Component for MenuItem {
    type State = MenuItemState;

    type Message = ();
}

#[derive(State)]
struct MenuItemState {
    active: Value<bool>,
}

pub(crate) fn register(runtime_builder: &mut Builder) -> anyhow::Result<()> {
    runtime_builder.prototype(
        "menu_item",
        "src/templates/extras/menu_item.aml",
        || MenuItem {
            description: "".to_string(),
        },
        || MenuItemState {
            active: Value::new(false),
        },
    )?;
    Ok(())
}
