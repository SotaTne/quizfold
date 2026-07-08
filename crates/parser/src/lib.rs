// Public entry point for the QuizFold parser crate.
// It wires AST, diagnostics, parser, formatter, and parse result types together.
pub mod ast;
mod constants;
pub mod diagnostics;
pub mod errors;
pub mod formatter;
pub mod lexer;
mod parse;
pub mod source;

use ast::{AttachmentKey, ExternalImageUrl, QuizFoldDocument, StoredImageId};
use diagnostics::Diagnostic;
pub use formatter::format_quizfold;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ParseStats {
    pub byte_len: usize,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct References {
    pub request_attachments: Vec<AttachmentKey>,
    pub stored_images: Vec<StoredImageId>,
    pub external_images: Vec<ExternalImageUrl>,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ParseResult {
    pub document: QuizFoldDocument,
    pub diagnostics: Vec<Diagnostic>,
    pub references: References,
    pub stats: ParseStats,
}

pub fn parse_quizfold(input: &str) -> ParseResult {
    parse::parse(input)
}

pub fn validate_quizfold(input: &str) -> Vec<Diagnostic> {
    parse_quizfold(input).diagnostics
}
