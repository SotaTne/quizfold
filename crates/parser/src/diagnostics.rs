use crate::errors::ParseError;
use crate::source::SourceRange;
use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum Severity {
    Fatal,
    Error,
    Warning,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub error: ParseError,
    pub source_range: SourceRange,
}

impl Diagnostic {
    pub const fn new(error: ParseError, source_range: SourceRange) -> Self {
        Self {
            error,
            source_range,
        }
    }

    pub const fn severity(&self) -> Severity {
        self.error.severity()
    }

    pub const fn code(&self) -> &'static str {
        self.error.code()
    }

    pub const fn message(&self) -> &'static str {
        self.error.message()
    }
}

impl Serialize for Diagnostic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Diagnostic", 5)?;
        state.serialize_field("error", &self.error)?;
        state.serialize_field("severity", &self.severity())?;
        state.serialize_field("code", self.code())?;
        state.serialize_field("message", self.message())?;
        state.serialize_field("source_range", &self.source_range)?;
        state.end()
    }
}
