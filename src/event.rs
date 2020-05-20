pub enum EventMode {
    New,
    Replay,
}

pub trait Event<TState> {
    fn execute(&self, state: TState, mode: EventMode) -> TState;
}