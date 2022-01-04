use js_sys::{Object, Reflect};
use laskea_engine::EvaluationError;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

#[wasm_bindgen(typescript_custom_section)]
const TYPES: &str = r#"
type Value =
    | { type: "number", value: number }
    | { type: "string", value: string }
    | { type: "boolean", value: boolean }
    | { type: "object", value: any }
    | { type: "indetermimate" }
    | { type: "error", value: string }
;
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Value")]
    pub type Value;
}

pub fn laskea_value(result: Result<laskea_engine::Value, EvaluationError>) -> Value {
    match result {
        Ok(laskea_engine::Value::Number(n)) => typed("number", n),
        Ok(laskea_engine::Value::String(s)) => typed("string", s.as_ref()),
        Ok(laskea_engine::Value::Boolean(b)) => typed("boolean", b),
        Ok(laskea_engine::Value::Object(obj)) => typed(
            "object",
            JsValue::from_serde(&obj).expect("Unable to serialize to JSON"),
        ),
        Ok(laskea_engine::Value::Indeterminate) => typed("indeterminate", JsValue::UNDEFINED),
        Err(e) => typed("error", e.to_string()),
    }
}

fn typed(ty: &str, value: impl Into<JsValue>) -> Value {
    let obj = Object::new();
    let ty = JsValue::from_str(ty);
    let value = value.into();

    let _ = Reflect::set(&obj, &JsValue::from_str("type"), &ty);
    let _ = Reflect::set(&obj, &JsValue::from_str("value"), &value);

    obj.unchecked_into()
}
