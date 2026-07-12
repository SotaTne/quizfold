// WebAssembly boundary for the QuizFold parser.
// It adapts Rust parser results into JS values and generated TypeScript types.
mod types;

use self::types::{Diagnostic, ParseResult};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ParseResult")]
    pub type JsParseResult;

    #[wasm_bindgen(typescript_type = "Diagnostic[]")]
    pub type JsDiagnostics;

    #[wasm_bindgen(typescript_type = "QuizFoldDocument")]
    pub type JsQuizFoldDocument;
}

#[wasm_bindgen(js_name = parseQuizFold)]
pub fn parse_quizfold(input: &str) -> Result<JsParseResult, JsValue> {
    serialize_to_js(&ParseResult::from(quizfold_parser::parse_quizfold(input)))
        .map(JsValue::unchecked_into)
        .map_err(serialization_error)
}

#[wasm_bindgen(js_name = validateQuizFold)]
pub fn validate_quizfold(input: &str) -> Result<JsDiagnostics, JsValue> {
    let diagnostics = quizfold_parser::validate_quizfold(input)
        .iter()
        .map(Diagnostic::from)
        .collect::<Vec<_>>();

    serialize_to_js(&diagnostics)
        .map(JsValue::unchecked_into)
        .map_err(serialization_error)
}

#[wasm_bindgen(js_name = printQuizFold)]
pub fn print_quizfold(document: JsQuizFoldDocument) -> Result<String, JsValue> {
    let document =
        serde_wasm_bindgen::from_value(document.into()).map_err(deserialization_error)?;
    Ok(quizfold_parser::print_quizfold(&document))
}

fn serialize_to_js<T>(value: &T) -> Result<JsValue, serde_wasm_bindgen::Error>
where
    T: serde::Serialize,
{
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
    value.serialize(&serializer)
}

fn serialization_error(error: serde_wasm_bindgen::Error) -> JsValue {
    JsValue::from_str(&error.to_string())
}

fn deserialization_error(error: serde_wasm_bindgen::Error) -> JsValue {
    JsValue::from_str(&error.to_string())
}
