use std::collections::BTreeMap;

use super::value::Value;

pub struct Object {
    pub name: Option<String>,
    pub fields: BTreeMap<String, Value>,
}
