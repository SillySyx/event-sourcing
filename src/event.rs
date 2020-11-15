#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: String,
    pub data: Vec<u8>,
}