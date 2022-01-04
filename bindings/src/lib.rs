mod expression;
mod node;
mod value;

use std::cell::RefCell;

use crate::{
    expression::Expression,
    node::Node,
    value::{laskea_value, Value},
};

use laskea_engine::{Evaluate, EvaluateStorage, Inputs, InputsStorage, Sequence};
use wasm_bindgen::prelude::*;

/// A high-level wrapper around the [`laskea_engine`].
#[wasm_bindgen]
#[derive(Default)]
pub struct Laskea(RefCell<Database>);

#[wasm_bindgen]
impl Laskea {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Laskea::default()
    }

    pub fn evaluate(&self, items: Vec<Node>) -> Result<Vec<Value>, JsValue> {
        let nodes = items
            .into_iter()
            .map(|n| n.to_rust())
            .collect::<Result<Sequence<_>, _>>()?;
        self.0.borrow_mut().set_nodes(nodes);

        Ok(self
            .0
            .borrow()
            .evaluate()
            .iter()
            .cloned()
            .map(|r| laskea_value(r))
            .collect())
    }
}

#[salsa::database(InputsStorage, EvaluateStorage)]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for Database {}
