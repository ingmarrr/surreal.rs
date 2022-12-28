use super::Value;

pub struct Array(pub Vec<dyn Into<Value>>);
