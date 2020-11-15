!!!WIP!!!


short description of the repo/project


## Features
* Store events in an immutable event log (you can only add new events to the log)
* Project events into your own states
* Each state is responsible for interpeting the event and may have their own implementation of them


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


## Examples

```
fn main() {
    // load event log

    // project event log into new state

    // read data from state

    // add a new event to the event log

    // project event log into new state using previous state as inital state
}
```