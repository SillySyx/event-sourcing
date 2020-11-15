Allows you to implement basic event sourcing into your applications.


## Features
* Uses simple event stucture to represent your events (type: `String` and data: `Vec<u8>`)
* Lets you define your own states
* Use event logs to project your events into any state
* Have full control over how states interpret events


## Todo
* Change Event struct to trait to allow custom implementations of the event model
* Implement Result for EventLog::project and State::execute for better error handling


## Installation
This library has not been published to cargo as a package, to use it you can add it as a submodule for your project.

Add as a submodule
```
git submodule add https://github.com/SillySyx/event-sourcing.git
```

Add to your dependencies
```
event-sourcing = { path = "./event-sourcing" }
```


## Example

Define your state
```
#[derive(Clone)]
struct MyNameState {
    name: String,
}
```

Implement required trait for your state
```
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
```

Load/initialize your event log and project it into your state
```
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
```