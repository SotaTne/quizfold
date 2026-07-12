use quizfold_parser::ast::{
    BlockKind, DocumentItemKind, FoldBlankInlineKind, ImageReference, InlineKind, QuizItemKind,
};
use quizfold_parser::diagnostics::Severity;
use quizfold_parser::errors::ParseError;
use quizfold_parser::parse_quizfold;

#[test]
fn parses_qa_quiz() {
    let result = parse_quizfold("? Capital of Japan?\n---\n${Tokyo}\n");

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Quiz(quiz) = &result.document.items[0].kind else {
        panic!("expected quiz item");
    };
    assert!(matches!(quiz.kind, QuizItemKind::Qa(_)));
}

#[test]
fn parses_fold_blank_as_ast_node() {
    let result = parse_quizfold("! Japan's capital is ${Tokyo}.\n");

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Quiz(quiz) = &result.document.items[0].kind else {
        panic!("expected quiz item");
    };
    let QuizItemKind::Fold(fold) = &quiz.kind else {
        panic!("expected fold quiz");
    };
    let paragraph = match &fold.content.blocks[0].kind {
        quizfold_parser::ast::BlockKind::Paragraph(paragraph) => paragraph,
        _ => panic!("expected paragraph"),
    };
    let blank = paragraph
        .inlines
        .iter()
        .find_map(|inline| match &inline.kind {
            InlineKind::FoldBlank(blank) => Some(blank),
            _ => None,
        })
        .expect("expected fold blank");

    assert!(matches!(
        blank.answer.inlines[0].kind,
        FoldBlankInlineKind::Raw(_)
    ));
}

#[test]
fn parses_multiple_fold_blanks_in_one_item_in_order() {
    let result = parse_quizfold("! The capital of ${France} is ${Paris}.\n");

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Quiz(quiz) = &result.document.items[0].kind else {
        panic!("expected quiz item");
    };
    let QuizItemKind::Fold(fold) = &quiz.kind else {
        panic!("expected fold quiz");
    };
    let paragraph = match &fold.content.blocks[0].kind {
        BlockKind::Paragraph(paragraph) => paragraph,
        _ => panic!("expected paragraph"),
    };
    let blanks: Vec<_> = paragraph
        .inlines
        .iter()
        .filter_map(|inline| match &inline.kind {
            InlineKind::FoldBlank(blank) => Some(blank),
            _ => None,
        })
        .collect();

    assert_eq!(blanks.len(), 2, "expected two independent fold blanks");
    // 出現順が保たれ、それぞれ独立したFoldBlankノードであること
    // (cloud側ではこの順序がposition_in_itemになり、穴ごとに独立して採点される)
    assert_eq!(fold_blank_raw_text(blanks[0]), "France");
    assert_eq!(fold_blank_raw_text(blanks[1]), "Paris");
}

#[test]
fn reports_fold_blank_in_qa_question_as_error() {
    let result = parse_quizfold("? What is ${this}?\n---\nAn answer\n");

    assert_diagnostic(&result, ParseError::FoldBlankOutsideAnswer, "QF012", 10, 17);
}

#[test]
fn reports_fold_blank_in_top_level_paragraph_as_error() {
    let result = parse_quizfold("Remember ${this}.\n");

    assert_diagnostic(&result, ParseError::FoldBlankOutsideAnswer, "QF012", 9, 16);
}

#[test]
fn reports_unclosed_blank_syntax_before_its_location() {
    let result = parse_quizfold("Remember ${this.\n");

    assert_diagnostic(&result, ParseError::UnclosedFoldBlank, "QF003", 9, 16);
    assert!(!result
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.error == ParseError::FoldBlankOutsideAnswer));
}

#[test]
fn allows_fold_blank_in_qa_answer() {
    let result = parse_quizfold("? Complete this\n---\nThe answer is ${this}.\n");

    assert!(result.diagnostics.is_empty());
}

fn fold_blank_raw_text(blank: &quizfold_parser::ast::FoldBlank) -> &str {
    match &blank.answer.inlines[0].kind {
        FoldBlankInlineKind::Raw(raw) => raw.value.as_ref(),
        _ => panic!("expected raw fold blank answer"),
    }
}

#[test]
fn reports_unclosed_fold_blank_as_error() {
    let result = parse_quizfold("! Japan's capital is ${Tokyo.\n");

    assert_diagnostic(&result, ParseError::UnclosedFoldBlank, "QF003", 21, 29);
}

#[test]
fn reports_missing_answer_separator_as_error() {
    let result = parse_quizfold("? Capital of Japan?\nTokyo\n");

    assert_diagnostic(&result, ParseError::MissingAnswerSeparator, "QF001", 20, 25);
}

#[test]
fn reports_qa_answer_that_is_memo_only_as_error() {
    let result = parse_quizfold("? Question\n---\n@memo\nhint\n@end\n");

    assert_diagnostic(&result, ParseError::QaSectionIsMemoOnly, "QF011", 15, 31);
}

#[test]
fn does_not_report_qa_question_that_is_memo_only_because_it_cannot_happen() {
    // マーカー行("? Question")は必ず実質のある段落としてquestion.blocksの
    // 先頭に入るため、questionがmemoだけになることは構造上あり得ない。
    // memoが続いても(Paragraph, Memo)の2ブロックになるだけで、all-memoにはならない。
    let result = parse_quizfold("? Question\n@memo\nhint\n@end\n---\nAnswer\n");

    assert!(result.diagnostics.is_empty());
}

#[test]
fn reports_fold_quiz_without_blank_as_error() {
    let result = parse_quizfold("! Japan's capital is Tokyo.\n");

    assert_diagnostic(&result, ParseError::FoldQuizWithoutBlank, "QF002", 0, 27);
}

#[test]
fn reports_unclosed_inline_math_as_error() {
    let result = parse_quizfold("Energy is $E = mc^2.\n");

    assert_diagnostic(&result, ParseError::UnclosedMathInline, "QF004", 10, 20);
}

#[test]
fn reports_unclosed_math_block_as_error() {
    let result = parse_quizfold("$$\nE = mc^2\n");

    assert_diagnostic(&result, ParseError::UnclosedBlock, "QF005", 0, 11);
}

#[test]
fn reports_unclosed_code_block_as_error() {
    let result = parse_quizfold("```rust\nfn main() {}\n");

    assert_diagnostic(&result, ParseError::UnclosedBlock, "QF005", 0, 20);
}

#[test]
fn parses_inline_math() {
    let result = parse_quizfold("Energy is $E = mc^2$.\n");

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    let BlockKind::Paragraph(paragraph) = &block.kind else {
        panic!("expected paragraph");
    };
    assert!(paragraph
        .inlines
        .iter()
        .any(|inline| matches!(inline.kind, InlineKind::MathInline(_))));
}

#[test]
fn parses_math_block() {
    let result = parse_quizfold("$$\nE = mc^2\n$$\n");

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    assert!(matches!(block.kind, BlockKind::MathBlock(_)));
}

#[test]
fn parses_mermaid_and_mmd_fences() {
    for language in ["mermaid", "mmd"] {
        let source = format!("```{language}\nflowchart LR\nA --> B\n```\n");
        let result = parse_quizfold(&source);

        assert!(result.diagnostics.is_empty());
        let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
            panic!("expected block");
        };
        assert!(matches!(block.kind, BlockKind::MermaidBlock(_)));
    }
}

#[test]
fn parses_code_fence_without_language() {
    let result = parse_quizfold("```\nlet value = 1;\n```\n");

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    let BlockKind::CodeBlock(code) = &block.kind else {
        panic!("expected code block");
    };
    assert!(code.language.is_none());
}

#[test]
fn parses_memo_with_multiple_block_kinds() {
    let source = "@memo\nRemember this equation.\n\n$$\nE = mc^2\n$$\n@end\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    let BlockKind::Memo(memo) = &block.kind else {
        panic!("expected memo block");
    };

    assert_eq!(memo.blocks.len(), 2);
    assert!(matches!(memo.blocks[0].kind, BlockKind::Paragraph(_)));
    assert!(matches!(memo.blocks[1].kind, BlockKind::MathBlock(_)));
}

#[test]
fn starts_top_level_memo_without_requiring_a_blank_line() {
    let source = "Remember the context.\n@memo\nHidden context.\n@end\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    assert_eq!(result.document.items.len(), 2);
    let DocumentItemKind::Block(block) = &result.document.items[1].kind else {
        panic!("expected block");
    };
    assert!(matches!(block.kind, BlockKind::Memo(_)));
}

#[test]
fn parses_memo_inside_qa_question() {
    let source =
        "? What is the energy equation?\n@memo\nUse mass and velocity.\n@end\n---\n$E = mc^2$\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Quiz(quiz) = &result.document.items[0].kind else {
        panic!("expected quiz item");
    };
    let QuizItemKind::Qa(qa) = &quiz.kind else {
        panic!("expected Q/A quiz");
    };

    assert_eq!(qa.question.blocks.len(), 2);
    assert!(matches!(
        qa.question.blocks[0].kind,
        BlockKind::Paragraph(_)
    ));
    assert!(matches!(qa.question.blocks[1].kind, BlockKind::Memo(_)));
}

#[test]
fn parses_memo_inside_qa_answer() {
    let source = "? What is the energy equation?\n---\n$E = mc^2$\n@memo\nEquivalent forms are acceptable.\n@end\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Quiz(quiz) = &result.document.items[0].kind else {
        panic!("expected quiz item");
    };
    let QuizItemKind::Qa(qa) = &quiz.kind else {
        panic!("expected Q/A quiz");
    };

    assert_eq!(qa.answer.blocks.len(), 2);
    assert!(matches!(qa.answer.blocks[1].kind, BlockKind::Memo(_)));
}

#[test]
fn treats_memo_markers_inside_code_fences_as_code() {
    let source = "@memo\n```text\n@memo\n@end\n```\n@end\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    let BlockKind::Memo(memo) = &block.kind else {
        panic!("expected memo block");
    };
    let BlockKind::CodeBlock(code) = &memo.blocks[0].kind else {
        panic!("expected code block");
    };
    assert_eq!(code.source.as_ref(), "@memo\n@end");
}

#[test]
fn treats_quiz_syntax_inside_memo_as_text() {
    let source = "@memo\n? This is not a question quiz.\n! This is not a fold quiz.\n---\n@end\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    assert_eq!(result.document.items.len(), 1);
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    let BlockKind::Memo(memo) = &block.kind else {
        panic!("expected memo block");
    };
    let BlockKind::Paragraph(paragraph) = &memo.blocks[0].kind else {
        panic!("expected paragraph");
    };
    let text = paragraph
        .inlines
        .iter()
        .filter_map(|inline| match &inline.kind {
            InlineKind::Raw(raw) => Some(raw.value.as_ref()),
            _ => None,
        })
        .collect::<String>();

    assert!(text.contains("? This is not a question quiz."));
    assert!(text.contains("! This is not a fold quiz."));
    assert!(text.contains("---"));
}

#[test]
fn treats_fold_blank_syntax_inside_memo_as_raw_text() {
    let source = "@memo\nRemember ${this} literally.\n@end\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    let BlockKind::Memo(memo) = &block.kind else {
        panic!("expected memo block");
    };
    let BlockKind::Paragraph(paragraph) = &memo.blocks[0].kind else {
        panic!("expected paragraph");
    };

    assert!(paragraph
        .inlines
        .iter()
        .all(|inline| !matches!(inline.kind, InlineKind::FoldBlank(_))));
    let text = paragraph
        .inlines
        .iter()
        .filter_map(|inline| match &inline.kind {
            InlineKind::Raw(raw) => Some(raw.value.as_ref()),
            _ => None,
        })
        .collect::<String>();
    assert_eq!(text, "Remember ${this} literally.");
}
#[test]
fn reports_unclosed_memo_as_error() {
    let source = "@memo\nRemember this.\n";
    let result = parse_quizfold(source);

    assert_diagnostic(&result, ParseError::UnclosedMemo, "QF008", 0, source.len());
}

#[test]
fn reports_unexpected_memo_end_as_error() {
    let source = "@end\n";
    let result = parse_quizfold(source);

    assert_diagnostic(&result, ParseError::UnexpectedMemoEnd, "QF009", 0, 4);
}

#[test]
fn reports_nested_memo_as_error() {
    let source = "@memo\nOuter.\n@memo\nInner.\n@end\n";
    let result = parse_quizfold(source);

    assert_diagnostic(&result, ParseError::NestedMemo, "QF010", 13, 18);
}

#[test]
fn reports_content_errors_inside_memo() {
    let source = "@memo\nEnergy is $E = mc^2.\n@end\n";
    let result = parse_quizfold(source);

    assert!(result
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.error == ParseError::UnclosedMathInline));
}

#[test]
fn treats_non_exact_memo_markers_as_text() {
    let source = "@memo note\n@end note\n";
    let result = parse_quizfold(source);

    assert!(result.diagnostics.is_empty());
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    assert!(matches!(block.kind, BlockKind::Paragraph(_)));
}

#[test]
fn parses_request_attachment_image() {
    let result = parse_quizfold("![Cell](qf-attachment:cell)\n");

    assert!(result.diagnostics.is_empty());
    let image = first_image(&result);
    assert_eq!(image.alt.value.as_ref(), "Cell");
    let ImageReference::RequestAttachment(key) = &image.reference else {
        panic!("expected request attachment");
    };
    assert_eq!(key.as_str(), "cell");
    assert_eq!(result.references.request_attachments[0].as_str(), "cell");
}

#[test]
fn parses_stored_image() {
    let result = parse_quizfold("![Cell](qf-stored:img_123)\n");

    assert!(result.diagnostics.is_empty());
    let image = first_image(&result);
    let ImageReference::StoredImage(id) = &image.reference else {
        panic!("expected stored image");
    };
    assert_eq!(id.as_str(), "img_123");
    assert_eq!(result.references.stored_images[0].as_str(), "img_123");
}

#[test]
fn reports_empty_image_alt_as_error() {
    let result = parse_quizfold("![](qf-attachment:cell)\n");

    assert_diagnostic(&result, ParseError::EmptyImageAlt, "QF006", 2, 2);
}

#[test]
fn rejects_external_image_reference() {
    let source = "![Cell](ftp://example.com/cell.png)\n";
    let result = parse_quizfold(source);
    let diagnostic = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.error == ParseError::InvalidImageReference)
        .expect("expected invalid image reference");

    assert_eq!(diagnostic.code(), "QF007");
    assert_eq!(
        &source[diagnostic.source_range.start..diagnostic.source_range.end],
        "ftp://example.com/cell.png"
    );
}

#[test]
fn parses_http_and_https_image_references() {
    for url in [
        "http://example.com/cell.png",
        "https://example.com/cell.png",
    ] {
        let source = format!("![Cell]({url})\n");
        let result = parse_quizfold(&source);

        assert!(result.diagnostics.is_empty());
        let image = first_image(&result);
        let ImageReference::ExternalUrl(external_url) = &image.reference else {
            panic!("expected external image URL");
        };
        assert_eq!(external_url.as_str(), url);
        assert_eq!(result.references.external_images[0].as_str(), url);
    }
}

#[test]
fn rejects_empty_image_references() {
    for source in [
        "![Cell](qf-attachment:)\n",
        "![Cell](qf-stored:)\n",
        "![Cell](https://)\n",
        "![Cell](http://)\n",
    ] {
        let result = parse_quizfold(source);
        assert!(result
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.error == ParseError::InvalidImageReference));
    }
}

fn first_image(result: &quizfold_parser::ParseResult) -> &quizfold_parser::ast::Image {
    let DocumentItemKind::Block(block) = &result.document.items[0].kind else {
        panic!("expected block");
    };
    let BlockKind::Paragraph(paragraph) = &block.kind else {
        panic!("expected paragraph");
    };
    paragraph
        .inlines
        .iter()
        .find_map(|inline| match &inline.kind {
            InlineKind::Image(image) => Some(image),
            _ => None,
        })
        .expect("expected image")
}

fn assert_diagnostic(
    result: &quizfold_parser::ParseResult,
    error: ParseError,
    code: &str,
    start: usize,
    end: usize,
) {
    let diagnostic = result
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.error == error)
        .unwrap_or_else(|| panic!("expected diagnostic {error:?}"));

    assert_eq!(diagnostic.code(), code);
    assert_eq!(diagnostic.severity(), Severity::Error);
    assert_eq!(diagnostic.source_range.start, start);
    assert_eq!(diagnostic.source_range.end, end);
    assert!(!diagnostic.message().is_empty());
}
