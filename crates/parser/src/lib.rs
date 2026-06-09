pub mod ast;
pub mod diagnostics;
pub mod errors;
pub mod lexer;
mod parse;
pub mod source;

use ast::{AttachmentKey, ExternalImageUrl, QuizFoldDocument, StoredImageId};
use diagnostics::Diagnostic;

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct ParseStats {
    pub byte_len: usize,
}

#[derive(Debug, Clone, Default, serde::Serialize)]
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
