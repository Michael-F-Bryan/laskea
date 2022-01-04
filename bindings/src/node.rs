use std::sync::Arc;

use crate::Expression;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &str = r#"
type Node = {
    name: string;
    expression: Expression;
};
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Node")]
    pub type Node;

    #[wasm_bindgen(structural, method, getter)]
    pub fn name(this: &Node) -> String;

    #[wasm_bindgen(structural, method, getter)]
    pub fn expression(this: &Node) -> Expression;
}

impl Node {
    pub fn to_rust(&self) -> Result<laskea_engine::Node, JsValue> {
        let expr = self.expression().to_rust()?;
        Ok(laskea_engine::Node {
            name: self.name().into(),
            expr: Arc::new(expr),
        })
    }
}
