use anathema::{component::Component, prelude::TuiBackend, runtime::RuntimeBuilder};

struct TitledBorder {
    #[allow(dead_code)]
    title: String,
}

impl Component for TitledBorder {
    type State = ();

    type Message = ();
}

pub(crate) fn register(runtime_builder: &mut RuntimeBuilder<TuiBackend, ()>) -> anyhow::Result<()> {
    runtime_builder.register_prototype(
        "titled_border",
        "src/components/templates/titled_border.aml",
        || TitledBorder {
            title: "".to_string(),
        },
        || (),
    )?;
    Ok(())
}
