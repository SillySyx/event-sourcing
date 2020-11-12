use std::error::Error;

pub enum EventMode {
    New,
    Replay,
}

pub trait Event<TState> {
    fn execute(&self, state: TState, mode: EventMode) -> Result<TState, Box<dyn Error>>;
}