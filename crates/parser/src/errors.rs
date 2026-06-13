use crate::diagnostics::Severity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum ParseError {
    MissingAnswerSeparator,
    FoldQuizWithoutBlank,
    UnclosedFoldBlank,
    UnclosedMathInline,
    UnclosedBlock,
    EmptyImageAlt,
    InvalidImageReference,
    UnclosedMemo,
    UnexpectedMemoEnd,
    NestedMemo,
}

impl ParseError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::MissingAnswerSeparator => "QF001",
            Self::FoldQuizWithoutBlank => "QF002",
            Self::UnclosedFoldBlank => "QF003",
            Self::UnclosedMathInline => "QF004",
            Self::UnclosedBlock => "QF005",
            Self::EmptyImageAlt => "QF006",
            Self::InvalidImageReference => "QF007",
            Self::UnclosedMemo => "QF008",
            Self::UnexpectedMemoEnd => "QF009",
            Self::NestedMemo => "QF010",
        }
    }

    pub const fn message(self) -> &'static str {
        match self {
            Self::MissingAnswerSeparator => "Q/A block requires an answer separator (`---`).",
            Self::FoldQuizWithoutBlank => "Fold quiz requires at least one `${...}` answer.",
            Self::UnclosedFoldBlank => "Fold answer is not closed with `}`.",
            Self::UnclosedMathInline => "Inline math is not closed with `$`.",
            Self::UnclosedBlock => "Block is not closed.",
            Self::EmptyImageAlt => "Image alt text must not be empty.",
            Self::InvalidImageReference => {
                "Image reference must use `qf-attachment:`, `qf-stored:`, `http://`, or `https://`."
            }
            Self::UnclosedMemo => "Memo block is not closed with `@end`.",
            Self::UnexpectedMemoEnd => "`@end` does not have a matching `@memo`.",
            Self::NestedMemo => "Memo blocks cannot be nested.",
        }
    }

    pub const fn severity(self) -> Severity {
        Severity::Error
    }
}
