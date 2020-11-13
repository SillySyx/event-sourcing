pub trait BinaryState {
    fn from_bytes(bytes: Vec<u8>) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait JsonState {
    fn from_json(json: String) -> Self;
    fn to_json(&self) -> String;
}