use anathema::component::ComponentId;

use crate::components::quarantine::quarantine_table::QuarantineTableMessage;

pub struct ComponentBucket {
    pub quarantine_table: Option<ComponentId<QuarantineTableMessage>>,
}

impl ComponentBucket {
    pub fn new() -> Self {
        Self {
            quarantine_table: None,
        }
    }
}
