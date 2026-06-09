use quizfold_parser::parse_quizfold;

#[test]
fn parses_empty_document() {
    let result = parse_quizfold("");
    assert_eq!(result.stats.byte_len, 0);
    assert!(result.diagnostics.is_empty());
}
