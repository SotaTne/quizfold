// Stable model-conversion error taxonomy and its structured diagnostics.
// QFM codes are public API; dynamic context stays in the diagnostic message.
use crate::diagnostics::Severity;
use crate::errors::define_errors;
use crate::source::SourceRange;

define_errors! {
pub enum ModelError {
    FoldBlankNotAllowed => "QFM001", "Fold blank is not allowed in this location.";
    InvalidFoldShape => "QFM002", "Fold quiz content must contain exactly one paragraph block.";
    MissingBlank => "QFM003", "Fold content must contain at least one blank.";
    BlankOutOfOrder => "QFM004", "Blank references must appear in zero-based index order.";
    MissingBlankReference => "QFM005", "Blank reference does not exist.";
    UnusedBlanks => "QFM006", "Every blank answer must be referenced exactly once.";
    TooManyBlanks => "QFM007", "Document contains too many blanks to index with u32.";
}
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ModelDiagnostic {
    pub error: ModelError,
    pub severity: Severity,
    #[cfg_attr(feature = "tsify", tsify(type = "ModelErrorCode"))]
    pub code: Box<str>,
    pub message: Box<str>,
    #[cfg_attr(feature = "tsify", tsify(optional))]
    pub source_range: Option<SourceRange>,
}

impl ModelDiagnostic {
    pub fn new(
        error: ModelError,
        message: impl Into<Box<str>>,
        source_range: Option<SourceRange>,
    ) -> Self {
        Self {
            error,
            severity: error.severity(),
            code: error.code().into(),
            message: message.into(),
            source_range,
        }
    }
}

impl std::fmt::Display for ModelDiagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ModelDiagnostic {}
