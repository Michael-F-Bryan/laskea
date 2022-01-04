use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &str = r#"
type Expression =
    | { type: "string", value: string }
    | { type: "equals", target: string, value: any }
    | { type: "get-property", target: string, field: string }
    | {
        type: "request",
        response?: {
            status: number,
            status_text: string,
            url: string,
            body: any,
        },
        error?: string,
     };
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Expression")]
    pub type Expression;

    #[wasm_bindgen(method, js_name = "type")]
    fn ty(this: &Expression) -> String;

    #[wasm_bindgen(method, getter)]
    fn target(this: &Expression) -> String;

    #[wasm_bindgen(method, getter)]
    fn field(this: &Expression) -> String;

    #[wasm_bindgen(method, getter)]
    fn value(this: &Expression) -> JsValue;
}

impl Expression {
    pub fn to_rust(&self) -> Result<laskea_engine::Expression, JsValue> {
        let ty = self.ty();

        match ty.as_str() {
            "string" => {
                let value = self.value().as_string().ok_or("Missing \"value\" field")?;
                Ok(laskea_engine::Expression::string(value))
            }
            "equals" => {
                let target = self.target();
                let value: laskea_engine::Value =
                    self.value().into_serde().map_err(|e| e.to_string())?;
                Ok(laskea_engine::Expression::equals(target, value))
            }
            "get-property" => {
                let target = self.target();
                let field = self.field();
                Ok(laskea_engine::Expression::get(target, field))
            }
            "request" => todo!(),
            _ => Err(format!("Unknown type: {}", ty).into()),
        }
    }
}
