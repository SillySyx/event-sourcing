use event_sourcing::{EventLog, Event, State};

#[derive(Debug, Clone)]
struct TestState {
    name: String,
}

impl State for TestState {
    fn execute(self, event: &Event) -> TestState
    {
        let state = self.clone();

        match event.event_type.as_str() {
            "ChangeName" => change_name(state, event),
            _ => state,
        }
    }
}

fn change_name(state: TestState, event: &Event) -> TestState {
    let mut state = state.clone();

    let name = event.data.clone();
    let name = String::from_utf8(name).unwrap();

    state.name = name;

    state
}

fn main() {
    let mut events = EventLog::new();
    let state = TestState {
        name: String::from(""),
    };

    let event = Event {
        event_type: String::from("ChangeName"),
        data: String::from("Hello").as_bytes().to_owned(),
    };
    events.add_event(&event);

    let state = events
        .project(state)
        .expect("test");

    println!("{}", state.name);
}