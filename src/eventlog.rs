use super::{Event, State};

#[derive(Debug, Clone)]
pub struct EventLog {
    pub events: Vec<Event>,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            events: vec![],
        }
    }

    pub fn project<TState: State>(&self, state: TState) -> TState {
        self.events
            .iter()
            .fold(state, |state, event| {
                state.execute(event)
            })
    }
}