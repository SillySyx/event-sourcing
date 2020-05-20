use event_sourcing::*;

#[derive(Debug)]
struct ChangeNameEvent {
    new_name: String,
}

#[derive(Debug, Clone)]
struct TestState {
    name: String,
}

impl Event<TestState> for ChangeNameEvent {
    fn execute(&self, state: TestState, _mode: EventMode) -> TestState {
        let mut state = state.clone();

        state.name = self.new_name.clone();

        state
    }
}

#[test]
fn can_change_name_in_state_via_event() {
    let state = TestState {
        name: String::from("1"),
    };

    let event = ChangeNameEvent {
        new_name: String::from("2"),
    };
    
    let state = event.execute(state, EventMode::New);

    assert!(state.name == String::from("2"));
}