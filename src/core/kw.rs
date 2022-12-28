use super::value::{object::Object, Value};

pub enum Kw {
    Use(Use),
    Let(Let),
    Transaction(Vec<Statement>),
    Begin,
    Cancel,
    Commit,
    Ifelse(Vec<(Condition, Vec<Statement>)>),
    Select(What, From, SelectOptions),
    Insert(Content, Into, InsertOptions),
    Create(Target, Content, CreateOptions),
    Update(Target, Content, UpdateOptions),
    Relate(Relation, RelateOptions),
    Delete(Thing, DeleteOptions),
    Define,
    Remove,
    Info(For),
}

pub enum Use {
    Ns(String),
    Db(String),
}

pub struct Let {
    pub name: Param,
    pub value: Value,
}

pub struct Param(pub String);

pub struct Condition {
    pub left: Value,
    pub right: Value,
    pub op: Op,
}

pub enum Op {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    Is,
    IsNot,
}

pub enum Content {
    Content(Object),
    Merge(Object),
}
