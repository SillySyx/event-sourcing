use super::Event;

pub trait State {
    fn execute(self, event: &Event) -> Self;
}