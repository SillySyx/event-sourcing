use super::Event;

/// Adds execute function to your state to allow events to make changes
pub trait State {
    /// Executes the event in the state
    /// 
    /// # Important
    /// Make sure you never mutate the state, 
    /// always clone the state before making any changes
    fn execute(&self, event: &Event) -> Self;
}