use std::{
    fmt::Display,
    hash::{Hash, Hasher},
    ops::Deref,
    sync::Arc,
};

use serde::Serialize;

use crate::Text;

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
pub enum Value {
    Number(i32),
    String(Text),
    Boolean(bool),
    Object(Object),
    Indeterminate,
}

impl Value {
    pub fn from_serde(s: impl Serialize) -> Result<Self, serde_json::Error> {
        Object::from_serde(s).map(Value::Object)
    }
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct EvaluationError(String);

impl<D: Display> From<D> for EvaluationError {
    fn from(value: D) -> Self {
        EvaluationError(value.to_string())
    }
}

/// A reference-counted [`serde_json::Value`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Object(Arc<serde_json::Value>);

impl Object {
    pub fn from_serde(s: impl Serialize) -> Result<Self, serde_json::Error> {
        serde_json::to_value(s).map(|v| Object(Arc::new(v)))
    }
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        fn hash_value(value: &serde_json::Value, state: &mut impl Hasher) {
            match value {
                serde_json::Value::Null => ().hash(state),
                serde_json::Value::Bool(b) => b.hash(state),
                serde_json::Value::Number(n) => n.hash(state),
                serde_json::Value::String(s) => s.hash(state),
                serde_json::Value::Array(a) => {
                    a.len().hash(state);
                    for value in a {
                        hash_value(value, state);
                    }
                }
                serde_json::Value::Object(obj) => {
                    for (key, value) in obj {
                        key.hash(state);
                        hash_value(value, state);
                    }
                }
            }
        }

        hash_value(&self.0, state);
    }
}

impl Deref for Object {
    type Target = serde_json::Value;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<serde_json::Value> for Object {
    fn from(value: serde_json::Value) -> Self {
        Object(Arc::new(value))
    }
}

impl<'a> From<&'a serde_json::Value> for Object {
    fn from(value: &'a serde_json::Value) -> Self {
        value.clone().into()
    }
}
