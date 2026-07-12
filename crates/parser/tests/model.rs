use quizfold_parser::model::{self, ModelError};
use quizfold_parser::{parse_quizfold, print_quizfold};

#[test]
fn converts_data_model_example_without_losing_memo_positions() {
    let source = concat!(
        "? 徳川家康がやったことは？\n",
        "@memo\n",
        "ここは、教科書ベース\n",
        "@end\n",
        "---\n",
        "@memo\n",
        "書き方はいくつもあるが、共通テストに合わせる\n",
        "@end\n",
        "${天下統一}を果たして、${江戸幕府}を作った\n",
        "\n",
        "@memo\n",
        "ここは家康関連のクイズおおめ\n",
        "@end\n",
    );

    let parsed = parse_quizfold(source);
    assert!(parsed.diagnostics.is_empty());
    let document = model::Document::try_from(&parsed.document).unwrap();

    assert_eq!(document.items.len(), 2);
    let model::Item::QaFold(qa) = &document.items[0] else {
        panic!("expected qafold item");
    };
    assert!(matches!(qa.question.blocks[0], model::Block::Paragraph(_)));
    assert!(matches!(qa.question.blocks[1], model::Block::Memo(_)));
    assert!(matches!(qa.content.blocks[0], model::Block::Memo(_)));
    let model::Block::Paragraph(answer) = &qa.content.blocks[1] else {
        panic!("expected answer paragraph");
    };
    assert!(matches!(answer.inlines[0], model::Inline::Blank(0)));
    assert!(matches!(answer.inlines[2], model::Inline::Blank(1)));
    assert_eq!(qa.blanks.len(), 2);
    assert_eq!(blank_text(&qa.blanks[0]), "天下統一");
    assert_eq!(blank_text(&qa.blanks[1]), "江戸幕府");

    let model::Item::Note(note) = &document.items[1] else {
        panic!("expected note item");
    };
    assert!(matches!(note.block, model::Block::Memo(_)));
}

#[test]
fn classifies_qa_by_whether_its_answer_contains_blanks() {
    let plain = parse_quizfold("? Capital of Japan?\n---\nTokyo\n");
    let folded = parse_quizfold("? Capital of Japan?\n---\n${Tokyo}\n");

    let plain = model::Document::try_from(&plain.document).unwrap();
    let folded = model::Document::try_from(&folded.document).unwrap();

    assert!(matches!(plain.items[0], model::Item::Qa(_)));
    assert!(matches!(folded.items[0], model::Item::QaFold(_)));
}

#[test]
fn fold_model_contains_inlines_instead_of_blocks() {
    let parsed = parse_quizfold("! The capital of ${France} is ${Paris}.\n");
    let document = model::Document::try_from(&parsed.document).unwrap();

    let model::Item::Fold(fold) = &document.items[0] else {
        panic!("expected fold item");
    };
    assert!(matches!(fold.content[1], model::Inline::Blank(0)));
    assert!(matches!(fold.content[3], model::Inline::Blank(1)));
    assert_eq!(fold.blanks.len(), 2);
}

#[test]
fn preserves_document_meaning_through_model_round_trip() {
    let scenarios = [
        "? Question\n@memo\nprivate note\n@end\n---\nAnswer\n",
        "? Complete this\n---\n@memo\nhint\n@end\n${first} and ${second}\n",
        "! Formula ${value $x$} and ![Graph](qf-stored:graph).\n",
        "@memo\nRemember ${this} literally.\n\n$$\nx^2\n$$\n@end\n",
        "```mmd\nflowchart LR\nA --> B\n```\n",
    ];

    for source in scenarios {
        assert_model_round_trip(source);
    }
}

#[test]
fn rejects_out_of_order_and_unused_blank_references() {
    let blank = model::Blank {
        answer: vec![model::BlankInline::Raw("answer".into())],
    };
    let out_of_order = model::Document {
        items: vec![model::Item::Fold(model::Fold {
            content: vec![model::Inline::Blank(1)],
            blanks: vec![blank.clone(), blank.clone()],
        })],
    };
    let unused = model::Document {
        items: vec![model::Item::Fold(model::Fold {
            content: vec![model::Inline::Blank(0)],
            blanks: vec![blank.clone(), blank],
        })],
    };

    let diagnostic = quizfold_parser::ast::QuizFoldDocument::try_from(&out_of_order).unwrap_err();
    assert_eq!(diagnostic.error, ModelError::BlankOutOfOrder);
    assert_eq!(diagnostic.code.as_ref(), "QFM004");
    assert!(diagnostic.source_range.is_none());

    let diagnostic = quizfold_parser::ast::QuizFoldDocument::try_from(&unused).unwrap_err();
    assert_eq!(diagnostic.error, ModelError::UnusedBlanks);
    assert_eq!(diagnostic.code.as_ref(), "QFM006");
    assert!(diagnostic.source_range.is_none());
}

#[test]
fn ast_to_model_error_keeps_the_source_range() {
    let parsed = parse_quizfold("Remember ${this}.\n");
    let diagnostic = model::Document::try_from(&parsed.document).unwrap_err();

    assert_eq!(diagnostic.error, ModelError::FoldBlankNotAllowed);
    assert_eq!(diagnostic.code.as_ref(), "QFM001");
    assert_eq!(diagnostic.source_range.unwrap().start, 9);
    assert_eq!(diagnostic.source_range.unwrap().end, 16);
}

fn blank_text(blank: &model::Blank) -> &str {
    let model::BlankInline::Raw(raw) = &blank.answer[0] else {
        panic!("expected raw blank answer");
    };
    raw
}

fn assert_model_round_trip(source: &str) {
    let first = parse_quizfold(source);
    assert!(
        first.diagnostics.is_empty(),
        "source diagnostics: {:?}",
        first.diagnostics
    );
    let first_model = model::Document::try_from(&first.document).unwrap();
    let restored = quizfold_parser::ast::QuizFoldDocument::try_from(&first_model).unwrap();
    let printed = print_quizfold(&restored);
    let second = parse_quizfold(&printed);
    assert!(
        second.diagnostics.is_empty(),
        "printed diagnostics: {:?}\n{printed}",
        second.diagnostics
    );
    let second_model = model::Document::try_from(&second.document).unwrap();

    assert_eq!(first_model, second_model);
    assert_eq!(printed, print_quizfold(&second.document));
}
