use crate::Text;
use std::{collections::BTreeMap, fmt::Display, hash::Hash, ops::Deref, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Node {
    pub name: Text,
    pub expr: Arc<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Expression {
    StringConstant(Text),
    Request {
        url: Text,
        response: Option<Response>,
        error: Option<EvaluationError>,
    },
    Equals {
        target: Text,
        value: Value,
    },
    GetProperty {
        target: Text,
        field: Text,
    },
}

impl Expression {
    pub fn string(s: impl Into<Text>) -> Self {
        Expression::StringConstant(s.into())
    }

    pub fn equals(target: impl Into<Text>, value: impl Into<Value>) -> Self {
        Expression::Equals {
            target: target.into(),
            value: value.into(),
        }
    }

    pub fn get(target: impl Into<Text>, field: impl Into<Text>) -> Self {
        Expression::GetProperty {
            target: target.into(),
            field: field.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Response {
    pub status: i32,
    pub status_text: Text,
    pub url: Text,
    pub body: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct EvaluationError(String);

impl<D: Display> From<D> for EvaluationError {
    fn from(value: D) -> Self {
        EvaluationError(value.to_string())
    }
}

/// A reference-counted JSON-like object.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Object(Arc<BTreeMap<Text, Value>>);

impl From<BTreeMap<Text, Value>> for Object {
    fn from(m: BTreeMap<Text, Value>) -> Self {
        Object(Arc::new(m))
    }
}

impl Deref for Object {
    type Target = BTreeMap<Text, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Number(i32),
    String(Text),
    Boolean(bool),
    Object(Object),
    Indeterminate,
}

macro_rules! impl_value_from {
    ($($type:ty => $variant:ident),* $(,)*) => {
        $(
            impl From<$type> for Value {
                fn from(value: $type) -> Value {
                    Value::$variant(value.into())
                }
            }
        )*
    };
}

impl_value_from! {
    i32 => Number,
    Text => String,
    String => String,
    bool => Boolean,
    Object => Object,
}

impl<'a> From<&'a str> for Value {
    fn from(s: &'a str) -> Self {
        s.to_string().into()
    }
}

impl From<Response> for Value {
    fn from(r: Response) -> Self {
        let Response {
            url,
            status,
            status_text,
            body,
        } = r;

        let mut obj = BTreeMap::default();
        obj.insert(Text::from("url"), url.into());
        obj.insert(Text::from("status"), status.into());
        obj.insert(Text::from("status_text"), status_text.into());
        obj.insert(Text::from("body"), body);

        Value::Object(obj.into())
    }
}
