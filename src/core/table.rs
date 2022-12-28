use super::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub name: String,
    pub schema: Option<Schema>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Schema(pub Vec<Column>);

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: Value,
}

impl Into<Statement> for Table {
    todo!()
}