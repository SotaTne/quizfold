// Wasm-facing DTOs and generated TypeScript declarations.
// These types mirror the public JavaScript API shape, not the parser internals.
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPES: &'static str = r#"
export type ErrorCode =
  | "QF001"
  | "QF002"
  | "QF003"
  | "QF004"
  | "QF005"
  | "QF006"
  | "QF007"
  | "QF008"
  | "QF009"
  | "QF010";
"#;

#[derive(serde::Serialize, tsify_next::Tsify)]
#[tsify(into_wasm_abi)]
pub struct ParseResult {
    pub document: quizfold_parser::ast::QuizFoldDocument,
    pub diagnostics: Vec<Diagnostic>,
    pub references: quizfold_parser::References,
    pub stats: quizfold_parser::ParseStats,
}

#[derive(serde::Serialize, tsify_next::Tsify)]
#[tsify(into_wasm_abi)]
pub struct Diagnostic {
    pub error: quizfold_parser::errors::ParseError,
    pub severity: quizfold_parser::diagnostics::Severity,
    #[tsify(type = "ErrorCode")]
    pub code: Box<str>,
    pub message: Box<str>,
    pub source_range: quizfold_parser::source::SourceRange,
}

impl From<quizfold_parser::ParseResult> for ParseResult {
    fn from(value: quizfold_parser::ParseResult) -> Self {
        Self {
            document: value.document,
            diagnostics: value.diagnostics.iter().map(Diagnostic::from).collect(),
            references: value.references,
            stats: value.stats,
        }
    }
}

impl From<&quizfold_parser::diagnostics::Diagnostic> for Diagnostic {
    fn from(value: &quizfold_parser::diagnostics::Diagnostic) -> Self {
        Self {
            error: value.error,
            severity: value.severity(),
            code: value.code().into(),
            message: value.message().into(),
            source_range: value.source_range,
        }
    }
}
