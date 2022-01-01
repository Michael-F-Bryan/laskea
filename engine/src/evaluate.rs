use std::sync::Arc;

use crate::{EvaluationError, Expression, Inputs, Value};

#[salsa::query_group(EvaluateStorage)]
pub trait Evaluate: Inputs {
    fn evaluate(&self, expr: Arc<Expression>) -> Result<Value, EvaluationError>;
}

fn evaluate(db: &dyn Evaluate, expr: Arc<Expression>) -> Result<Value, EvaluationError> {
    todo!()
}
