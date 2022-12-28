use self::{
    array::Array, datetime::DateTime, duration::Duration, number::Number, object::Object,
    table::Table,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    None,
    False,
    True,
    Number(Number),
    Duration(Duration),
    Datetime(DateTime),
    Uuid(Uuid),
    Array(Array),
    Table(Table),
    Thing(Thing),
    Object(Object),
}
