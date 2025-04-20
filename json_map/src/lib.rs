use serde_json::Value;
use serde_wasm_bindgen::{self};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type RecursiveJsonMap =
  | string
  | number
  | boolean
  | null
  | RecursiveJsonMap[]
  | Map<string, RecursiveJsonMap>
"#;

/// Recursively parse JSON, preserving property order
/// @returns a nested ES6 Map structure
///
/// ```ts
/// import { assertEquals } from "jsr:@std/assert/equals"
///
/// const result = parse(`{ ".": 1, "$": 2, "1": 3 }`) // Map(3) { "." => 1, "$" => 2, "1" => 3 }
/// assertEquals(result, new Map([[".", 1], ["$", 2], ["1", 3]]))
/// ```
#[wasm_bindgen(unchecked_return_type = "RecursiveJsonMap")]
pub fn parse(string: JsValue) -> Result<JsValue, JsValue> {
  let string: String = string
    .as_string()
    .ok_or_else(|| JsValue::from_str("Expected a string"))?;

  let string: Value = serde_json::from_str(&string)
    .map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

  let js_value = serde_wasm_bindgen::to_value(&string).map_err(|e| {
    JsValue::from_str(&format!("Failed to serialize to JsValue: {e}"))
  })?;

  Ok(js_value)
}

/// Serialize given javascript object to a JSON string, preserving property order
/// @returns a JSON string
///
/// ```ts
/// import { assertEquals } from "jsr:@std/assert/equals"
///
/// const result = stringify(new Map([
///      [".", 1],
///      ["$", 2],
///      ["1", 3],
/// ]))
/// assertEquals(result, '{".":1,"$":2,"1":3}')
/// ```
#[wasm_bindgen(unchecked_return_type = "string")]
pub fn stringify(
  #[wasm_bindgen(unchecked_param_type = "unknown")] value: JsValue,
) -> Result<JsValue, JsValue> {
  let value = from_value(value)?;
  let string = serde_json::to_string(&value).map_err(format_error)?;

  Ok(JsValue::from_str(&string))
}

/// Serialize given javascript object to a JSON string, preserving property order
/// @returns a pretty-formatted JSON string
///
/// ```ts
/// import { assertEquals } from "jsr:@std/assert/equals"
///
/// const result = prettyPrint(new Map([
///      [".", 1],
///      ["$", 2],
///      ["1", 3],
/// ]))
/// assertEquals(result, '{\n  ".": 1,\n  "$": 2,\n  "1": 3\n}')
/// ```
#[wasm_bindgen(unchecked_return_type = "string", js_name = "prettyPrint")]
pub fn pretty_print(
  #[wasm_bindgen(unchecked_param_type = "unknown")] value: JsValue,
) -> Result<JsValue, JsValue> {
  let value = from_value(value)?;
  let string = serde_json::to_string_pretty(&value).map_err(format_error)?;

  Ok(JsValue::from_str(&string))
}

fn from_value(value: JsValue) -> Result<Value, JsValue> {
  serde_wasm_bindgen::from_value(value).map_err(|e| {
    JsValue::from_str(&format!("Failed to deserialize from JsValue: {e}"))
  })
}

fn format_error(e: serde_json::Error) -> JsValue {
  JsValue::from_str(&format!("Failed to serialize to JSON: {e}"))
}
