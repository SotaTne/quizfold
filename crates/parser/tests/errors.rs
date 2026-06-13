use std::collections::BTreeSet;

use quizfold_parser::diagnostics::Severity;
use quizfold_parser::errors::ParseError;

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
    ];

    let codes: BTreeSet<_> = errors.iter().map(|error| error.code()).collect();
    assert_eq!(codes.len(), errors.len());

    for error in errors {
        assert!(!error.code().is_empty());
        assert!(!error.message().is_empty());
        assert_eq!(error.severity(), Severity::Error);
    }
}
