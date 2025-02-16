use anathema::{
    component::Component,
    runtime::Builder,
    state::{State, Value},
};

#[derive(Default)]
#[allow(dead_code)]
struct QuarantineRow {
    threat: u8,
    policy: String,
    quarantine_time: String,
    component_name: String,
    repository_name: String,
}

#[derive(State)]
struct QuarantineRowState {
    active: Value<bool>,
}

impl Component for QuarantineRow {
    type State = QuarantineRowState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}

pub(crate) fn register(runtime_builder: &mut Builder<()>) -> anyhow::Result<()> {
    runtime_builder.prototype(
        "quarantine_row",
        "src/templates/quarantine/quarantine_row.aml",
        QuarantineRow::default,
        || QuarantineRowState {
            active: Value::new(false),
        },
    )?;
    Ok(())
}
