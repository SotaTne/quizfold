use std::collections::BTreeSet;

use quizfold_parser::diagnostics::Severity;
use quizfold_parser::errors::ParseError;
use quizfold_parser::model::ModelError;

#[test]
fn parse_errors_have_unique_codes_and_messages() {
    let errors = [
        ParseError::MissingAnswerSeparator,
        ParseError::FoldQuizWithoutBlank,
        ParseError::UnclosedFoldBlank,
        ParseError::UnclosedMathInline,
        ParseError::UnclosedBlock,
        ParseError::EmptyImageAlt,
        ParseError::InvalidImageReference,
        ParseError::UnclosedMemo,
        ParseError::UnexpectedMemoEnd,
        ParseError::NestedMemo,
        ParseError::QaSectionIsMemoOnly,
        ParseError::FoldBlankOutsideAnswer,
    ];

    let codes: BTreeSet<_> = errors.iter().map(|error| error.code()).collect();
    assert_eq!(codes.len(), errors.len());

    for error in errors {
        assert!(!error.code().is_empty());
        assert!(!error.message().is_empty());
        assert_eq!(error.severity(), Severity::Error);
    }
}

#[test]
fn model_errors_have_unique_qfm_codes_and_messages() {
    let errors = [
        ModelError::FoldBlankNotAllowed,
        ModelError::InvalidFoldShape,
        ModelError::MissingBlank,
        ModelError::BlankOutOfOrder,
        ModelError::MissingBlankReference,
        ModelError::UnusedBlanks,
        ModelError::TooManyBlanks,
    ];

    let codes: BTreeSet<_> = errors.iter().map(|error| error.code()).collect();
    assert_eq!(codes.len(), errors.len());

    for error in errors {
        assert!(error.code().starts_with("QFM"));
        assert!(!error.message().is_empty());
        assert_eq!(error.severity(), Severity::Error);
    }
}
