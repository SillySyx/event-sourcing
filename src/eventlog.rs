use std::error::Error;

use super::{Event, State};

pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub fn new() -> Self {
        Self {
            events: vec![],
        }
    }

    pub fn add_event(&mut self, event: &Event) {
        let event = event.to_owned();
        
        self.events.push(event);
    }

    pub fn project<TState: State>(&self, state: TState) -> Result<TState, Box<dyn Error>> {
        let state = self.events
            .iter()
            .fold(state, |state, event| {
                state.execute(event)
            });

        Ok(state)
    }
}