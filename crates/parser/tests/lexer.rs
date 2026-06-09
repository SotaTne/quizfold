use quizfold_parser::lexer::{lex, TokenKind};

#[test]
fn lexes_quiz_and_inline_tokens() {
    let tokens = lex("? Energy is $E = mc^2$ and ${energy}.\n");
    let kinds: Vec<_> = tokens.iter().map(|token| token.kind).collect();

    assert_eq!(
        kinds,
        vec![
            TokenKind::QuestionMarker,
            TokenKind::Raw,
            TokenKind::MathInlineDelimiter,
            TokenKind::Raw,
            TokenKind::MathInlineDelimiter,
            TokenKind::Raw,
            TokenKind::FoldBlankStart,
            TokenKind::Raw,
            TokenKind::FoldBlankEnd,
            TokenKind::Raw,
            TokenKind::Newline,
        ]
    );
}

#[test]
fn lexes_bare_code_fence() {
    let tokens = lex("```\nlet value = 1;\n```\n");

    assert!(matches!(
        tokens[0].kind,
        TokenKind::CodeFenceStart { info: None }
    ));
    assert!(tokens
        .iter()
        .any(|token| token.kind == TokenKind::CodeFenceEnd));
}

#[test]
fn lexes_mermaid_fence_info_range() {
    let source = "```mmd\nflowchart LR\n```\n";
    let tokens = lex(source);
    let TokenKind::CodeFenceStart { info: Some(range) } = tokens[0].kind else {
        panic!("expected fence info");
    };

    assert_eq!(&source[range.start..range.end], "mmd");
}

#[test]
fn lexes_math_block_delimiters() {
    let tokens = lex("$$\nE = mc^2\n$$\n");
    assert_eq!(
        tokens
            .iter()
            .filter(|token| token.kind == TokenKind::MathBlockDelimiter)
            .count(),
        2
    );
}

#[test]
fn lexes_markdown_image_ranges() {
    let source = "Before ![Cell](qf-attachment:cell) after";
    let tokens = lex(source);
    let image = tokens
        .iter()
        .find(|token| matches!(token.kind, TokenKind::Image { .. }))
        .expect("expected image token");
    let TokenKind::Image { alt, destination } = image.kind else {
        unreachable!();
    };

    assert_eq!(&source[alt.start..alt.end], "Cell");
    assert_eq!(
        &source[destination.start..destination.end],
        "qf-attachment:cell"
    );
    assert_eq!(
        &source[image.source_range.start..image.source_range.end],
        "![Cell](qf-attachment:cell)"
    );
}
