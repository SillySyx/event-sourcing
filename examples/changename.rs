use event_sourcing::{EventLog, Event, State};

#[derive(Clone)]
struct MyNameState {
    name: String,
}

impl State for MyNameState {
    fn execute(&self, event: &Event) -> Self
    {
        if event.event_type == "ChangeName" {
            return change_name(self, event);
        }

        self.clone()
    }
}

fn change_name(state: &MyNameState, event: &Event) -> MyNameState {
    let mut state = state.clone();

    let name = event.data.clone();
    let name = String::from_utf8(name).unwrap();

    state.name = name;

    state
}

fn main() {
    let event = Event {
        event_type: String::from("ChangeName"),
        data: String::from("Hello").as_bytes().to_owned(),
    };
    
    let mut eventlog = EventLog::new();
    eventlog.events.push(event);
    
    let state = MyNameState {
        name: String::from("Me"),
    };
    
    let state = eventlog.project(state);

    println!("{}", state.name);
}