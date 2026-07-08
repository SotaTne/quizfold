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
    pub references: References,
    pub stats: ParseStats,
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

#[derive(serde::Serialize, tsify_next::Tsify)]
#[tsify(into_wasm_abi)]
pub struct ParseStats {
    pub byte_len: usize,
}

#[derive(serde::Serialize, tsify_next::Tsify)]
#[tsify(into_wasm_abi)]
pub struct References {
    pub request_attachments: Vec<quizfold_parser::ast::AttachmentKey>,
    pub stored_images: Vec<quizfold_parser::ast::StoredImageId>,
    pub external_images: Vec<quizfold_parser::ast::ExternalImageUrl>,
}

impl From<quizfold_parser::ParseResult> for ParseResult {
    fn from(value: quizfold_parser::ParseResult) -> Self {
        Self {
            document: value.document,
            diagnostics: value.diagnostics.iter().map(Diagnostic::from).collect(),
            references: References::from(value.references),
            stats: ParseStats::from(value.stats),
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

impl From<quizfold_parser::References> for References {
    fn from(value: quizfold_parser::References) -> Self {
        Self {
            request_attachments: value.request_attachments,
            stored_images: value.stored_images,
            external_images: value.external_images,
        }
    }
}

impl From<quizfold_parser::ParseStats> for ParseStats {
    fn from(value: quizfold_parser::ParseStats) -> Self {
        Self {
            byte_len: value.byte_len,
        }
    }
}
