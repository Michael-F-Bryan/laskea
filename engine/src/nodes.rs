use std::{collections::BTreeMap, sync::Arc};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: Arc<str>,
    pub expression: Arc<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    StringConstant(String),
    Request {
        url: String,
        response: Option<Response>,
        error: Option<EvaluationError>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Response {
    status: i32,
    status_test: String,
    url: String,
    body: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Number(i32),
    String(String),
    Boolean(bool),
    Object(BTreeMap<String, Value>),
    Indeterminate,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvaluationError(String);
