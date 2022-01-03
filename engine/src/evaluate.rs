use crate::{EvaluationError, Expression, Inputs, Node, Object, Sequence, Text, Value};
use std::{
    collections::{btree_map::Entry, BTreeMap},
    sync::Arc,
};

#[salsa::query_group(EvaluateStorage)]
pub trait Evaluate: Inputs {
    fn evaluate(&self) -> Sequence<NodeResult>;
    fn eval(&self, name: Text, expr: Arc<Expression>) -> Result<Value, EvaluationError>;
    fn named_expressions(&self) -> BTreeMap<Text, NamedExpression>;
    fn reference_cycle(&self, name: Text) -> Option<Sequence<Text>>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeResult {
    pub name: Text,
    pub value: Result<Value, EvaluationError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamedExpression {
    pub index: usize,
    pub expression: Arc<Expression>,
}

fn evaluate(db: &dyn Evaluate) -> Sequence<NodeResult> {
    let mut results = Vec::new();

    for node in db.nodes().iter().cloned() {
        let Node { name, expr } = node;
        let value = db.eval(name.clone(), expr);
        results.push(NodeResult { name, value });
    }

    results.into()
}

fn eval(db: &dyn Evaluate, name: Text, expr: Arc<Expression>) -> Result<Value, EvaluationError> {
    if let Some(cycle) = db.reference_cycle(name) {
        let msg = format!("Cycle detected: {}", cycle.join(" → "));
        return Err(EvaluationError::from(msg));
    }

    match Expression::clone(&expr) {
        Expression::StringConstant(s) => Ok(Value::String(s)),
        Expression::Request { error: Some(e), .. } => Err(e),
        Expression::Request {
            response: Some(response),
            error: None,
            ..
        } => Value::from_serde(&response).map_err(Into::into),
        Expression::Request {
            response: None,
            error: None,
            ..
        } => Ok(Value::Indeterminate),
        Expression::Equals { target, value } => equals(db, target, value),
        Expression::GetProperty { target, field } => get_property(db, target, field),
    }
}

fn equals(db: &dyn Evaluate, target: Text, value: Value) -> Result<Value, EvaluationError> {
    let expressions = db.named_expressions();

    let NamedExpression { expression, .. } = expressions
        .get(&target)
        .ok_or_else(|| format!("No \"{}\" input found", target))?;

    match db.eval(target, Arc::clone(expression)) {
        Ok(target_value) => Ok(Value::from(target_value == value)),
        Err(_) => Ok(Value::Indeterminate),
    }
}

fn get_property(db: &dyn Evaluate, target: Text, field: Text) -> Result<Value, EvaluationError> {
    let expressions = db.named_expressions();
    let NamedExpression { expression, .. } = expressions
        .get(&target)
        .ok_or_else(|| format!("No \"{}\" input found", target))?;

    match db.eval(target, Arc::clone(expression)) {
        Ok(Value::Object(obj)) => match obj.get(&*field) {
            Some(field_value) => Ok(Value::from(Object::from(field_value))),
            None => todo!(),
        },
        Ok(_) => todo!(),
        Err(_) => Ok(Value::Indeterminate),
    }
}

fn named_expressions(db: &dyn Evaluate) -> BTreeMap<Text, NamedExpression> {
    let mut expressions = BTreeMap::new();

    for (
        index,
        Node {
            name,
            expr: expression,
        },
    ) in db.nodes().iter().cloned().enumerate()
    {
        let named = NamedExpression { index, expression };

        if let Entry::Vacant(entry) = expressions.entry(name) {
            entry.insert(named);
        }
    }

    expressions
}

fn reference_cycle(db: &dyn Evaluate, name: Text) -> Option<Sequence<Text>> {
    let expressions = db.named_expressions();

    let mut dependencies = vec![name.clone()];
    let mut item = &name;

    while let Some(dep) = expressions
        .get(item)
        .and_then(|e| dependency(&e.expression))
    {
        dependencies.push(dep.clone());

        if *dep == name {
            return Some(dependencies.into());
        }

        item = dep;
    }

    None
}

fn dependency(expr: &Expression) -> Option<&Text> {
    match expr {
        Expression::StringConstant(_) | Expression::Request { .. } => None,
        Expression::Equals { target, .. } | Expression::GetProperty { target, .. } => Some(target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{inputs::InputsStorage, Response, Text};

    #[salsa::database(InputsStorage, EvaluateStorage)]
    #[derive(Default)]
    struct Database {
        storage: salsa::Storage<Self>,
    }

    impl salsa::Database for Database {}

    #[test]
    fn constant_expression() {
        let mut db = Database::default();
        db.set_nodes(Sequence::empty());
        let expr = Arc::new(Expression::StringConstant("asdf".into()));

        let got = db.eval("".into(), expr).unwrap();

        assert_eq!(got, Value::from("asdf"));
    }

    #[test]
    fn unfulfilled_request() {
        let mut db = Database::default();
        db.set_nodes(Sequence::empty());
        let expr = Arc::new(Expression::Request {
            url: "".into(),
            response: None,
            error: None,
        });

        let got = db.eval("".into(), expr).unwrap();

        assert_eq!(got, Value::Indeterminate);
    }

    #[test]
    fn successful_request() {
        let mut db = Database::default();
        db.set_nodes(Sequence::empty());
        let response = Response {
            url: "http://example.com/".into(),
            status: 200,
            status_text: Text::from("OK"),
            body: Value::Number(42),
        };
        let expr = Arc::new(Expression::Request {
            url: "".into(),
            response: Some(response.clone()),
            error: None,
        });
        let should_be = Value::from_serde(&response).unwrap();

        let got = db.eval("".into(), expr).unwrap();

        assert_eq!(got, should_be);
    }

    #[test]
    fn failed_request() {
        let mut db = Database::default();
        db.set_nodes(Sequence::empty());
        let error = EvaluationError::from("an error occurred");
        let expr = Arc::new(Expression::Request {
            url: "".into(),
            response: None,
            error: Some(error.clone()),
        });

        let got = db.eval("".into(), expr).unwrap_err();

        assert_eq!(got, error);
    }

    #[test]
    fn string_equals() {
        let mut db = Database::default();
        let equals = Arc::new(Expression::Equals {
            target: "input".into(),
            value: Value::from("Hello, World!"),
        });
        let nodes = vec![
            Node {
                name: "input".into(),
                expr: Arc::new(Expression::StringConstant(Text::from("Hello, World!"))),
            },
            Node {
                name: "equals".into(),
                expr: Arc::clone(&equals),
            },
        ];
        db.set_nodes(nodes.into());

        let got = db.eval("".into(), equals).unwrap();

        assert_eq!(got, Value::from(true));
    }

    #[test]
    fn string_equals_unknown_input() {
        let mut db = Database::default();
        let equals = Arc::new(Expression::Equals {
            target: "input".into(),
            value: Value::from("Hello, World!"),
        });
        db.set_nodes(Vec::new().into());

        let err = db.eval("".into(), equals).unwrap_err();

        assert_eq!(err, EvaluationError::from("No \"input\" input found"));
    }

    #[test]
    fn cycles_are_errors() {
        let mut db = Database::default();
        let nodes: Sequence<_> = vec![
            Node {
                name: "first".into(),
                expr: Expression::equals("second", 42).into(),
            },
            Node {
                name: "second".into(),
                expr: Expression::equals("first", 42).into(),
            },
        ]
        .into();
        db.set_nodes(nodes.clone());

        let should_be = vec![
            NodeResult {
                name: "first".into(),
                value: Err(EvaluationError::from(
                    "Cycle detected: first → second → first",
                )),
            },
            NodeResult {
                name: "second".into(),
                value: Err(EvaluationError::from(
                    "Cycle detected: second → first → second",
                )),
            },
        ];

        let got = db.evaluate();

        assert_eq!(got, should_be);
    }
}
