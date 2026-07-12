use quizfold_parser::ast::{
    Block, BlockKind, DocumentItem, DocumentItemKind, FoldBlankContent, FoldBlankInline,
    FoldBlankInlineKind, ImageReference, Inline, InlineKind, QuizContent, QuizFoldDocument,
    QuizItemKind,
};
use quizfold_parser::{parse_quizfold, print_quizfold};

#[test]
fn prints_quizfold_as_canonical_markdown() {
    let source = concat!(
        "? Energy equation?\n",
        "@memo\n",
        "Use mass and velocity.\n",
        "@end\n",
        "---\n",
        "$E = mc^2$ and ![Graph](qf-attachment:graph)\n",
        "\n",
        "! Japan's capital is ${Tokyo}.\n",
        "\n",
        "@memo\n",
        "![Stored](qf-stored:img_123)\n",
        "\n",
        "```mmd\n",
        "flowchart LR\n",
        "A --> B\n",
        "```\n",
        "\n",
        "$$\n",
        "E = mc^2\n",
        "$$\n",
        "@end\n",
    );

    let parsed = parse_quizfold(source);
    assert!(parsed.diagnostics.is_empty());

    let printed = print_quizfold(&parsed.document);

    assert_eq!(
        printed,
        concat!(
            "? Energy equation?\n",
            "@memo\n",
            "Use mass and velocity.\n",
            "@end\n",
            "---\n",
            "$E = mc^2$ and ![Graph](qf-attachment:graph)\n",
            "\n",
            "! Japan's capital is ${Tokyo}.\n",
            "\n",
            "@memo\n",
            "![Stored](qf-stored:img_123)\n",
            "\n",
            "```mermaid\n",
            "flowchart LR\n",
            "A --> B\n",
            "```\n",
            "\n",
            "$$\n",
            "E = mc^2\n",
            "$$\n",
            "@end\n",
        )
    );
}

#[test]
fn preserves_semantic_ast_through_round_trip() {
    let scenarios = [
        (
            "memo in question",
            "? Question\n@memo\n? memo text\n! memo text\n---\n@end\n---\nAnswer\n",
        ),
        (
            "fold blank, math, and external image",
            "! Formula: ${value $x$} and ![External](https://example.com/a.png).\n",
        ),
        (
            "multiple fold blanks in one item",
            "! The capital of ${France} is ${Paris}, not ${Berlin}.\n",
        ),
        (
            "memo with code and display math",
            "@memo\n```rust\nfn main() {}\n```\n\n$$\nx^2\n$$\n@end\n",
        ),
        ("multiline paragraph", "Plain\nmultiline\nparagraph\n"),
        (
            "multiple question paragraphs",
            "? First paragraph\n\nSecond paragraph\n---\nAnswer\n",
        ),
        (
            "memo in answer",
            "? Question\n---\nAnswer\n@memo\nPrivate answer context.\n@end\n",
        ),
        (
            "all image references",
            concat!(
                "![Attachment](qf-attachment:cell) ",
                "![Stored](qf-stored:img_123) ",
                "![External](http://example.com/image.png)\n",
            ),
        ),
        ("code without language", "```\nplain code\n```\n"),
        ("mermaid alias", "```mmd\nflowchart LR\nA --> B\n```\n"),
    ];

    for (name, source) in scenarios {
        assert_round_trip(name, source);
    }
}

fn assert_round_trip(name: &str, source: &str) {
    let first = parse_quizfold(source);
    assert!(
        first.diagnostics.is_empty(),
        "{name}: source diagnostics: {:?}",
        first.diagnostics
    );

    let printed = print_quizfold(&first.document);
    let second = parse_quizfold(&printed);
    assert!(
        second.diagnostics.is_empty(),
        "{name}: printed diagnostics: {:?}\n{printed}",
        second.diagnostics
    );

    assert_document_eq(&first.document, &second.document);
    assert_eq!(
        printed,
        print_quizfold(&second.document),
        "{name}: printer is not idempotent"
    );
}

fn assert_document_eq(left: &QuizFoldDocument, right: &QuizFoldDocument) {
    assert_eq!(left.items.len(), right.items.len());
    for (left, right) in left.items.iter().zip(&right.items) {
        assert_item_eq(left, right);
    }
}

fn assert_item_eq(left: &DocumentItem, right: &DocumentItem) {
    match (&left.kind, &right.kind) {
        (DocumentItemKind::Quiz(left), DocumentItemKind::Quiz(right)) => {
            match (&left.kind, &right.kind) {
                (QuizItemKind::Qa(left), QuizItemKind::Qa(right)) => {
                    assert_content_eq(&left.question, &right.question);
                    assert_content_eq(&left.answer, &right.answer);
                }
                (QuizItemKind::Fold(left), QuizItemKind::Fold(right)) => {
                    assert_content_eq(&left.content, &right.content);
                }
                _ => panic!("quiz kinds differ"),
            }
        }
        (DocumentItemKind::Block(left), DocumentItemKind::Block(right)) => {
            assert_block_eq(left, right);
        }
        _ => panic!("document item kinds differ"),
    }
}

fn assert_content_eq(left: &QuizContent, right: &QuizContent) {
    assert_eq!(left.blocks.len(), right.blocks.len());
    for (left, right) in left.blocks.iter().zip(&right.blocks) {
        assert_block_eq(left, right);
    }
}

fn assert_block_eq(left: &Block, right: &Block) {
    match (&left.kind, &right.kind) {
        (BlockKind::Paragraph(left), BlockKind::Paragraph(right)) => {
            assert_inlines_eq(&left.inlines, &right.inlines);
        }
        (BlockKind::Memo(left), BlockKind::Memo(right)) => {
            assert_eq!(left.blocks.len(), right.blocks.len());
            for (left, right) in left.blocks.iter().zip(&right.blocks) {
                assert_block_eq(left, right);
            }
        }
        (BlockKind::MathBlock(left), BlockKind::MathBlock(right)) => {
            assert_eq!(left.source, right.source);
        }
        (BlockKind::CodeBlock(left), BlockKind::CodeBlock(right)) => {
            assert_eq!(left.language, right.language);
            assert_eq!(left.source, right.source);
        }
        (BlockKind::MermaidBlock(left), BlockKind::MermaidBlock(right)) => {
            assert_eq!(left.source, right.source);
        }
        _ => panic!("block kinds differ"),
    }
}

fn assert_inlines_eq(left: &[Inline], right: &[Inline]) {
    assert_eq!(left.len(), right.len());
    for (left, right) in left.iter().zip(right) {
        match (&left.kind, &right.kind) {
            (InlineKind::Raw(left), InlineKind::Raw(right)) => {
                assert_eq!(left.value, right.value);
            }
            (InlineKind::MathInline(left), InlineKind::MathInline(right)) => {
                assert_eq!(left.source, right.source);
            }
            (InlineKind::FoldBlank(left), InlineKind::FoldBlank(right)) => {
                assert_fold_content_eq(&left.answer, &right.answer);
            }
            (InlineKind::Image(left), InlineKind::Image(right)) => {
                assert_eq!(left.alt, right.alt);
                assert_image_reference_eq(&left.reference, &right.reference);
            }
            (InlineKind::SoftBreak, InlineKind::SoftBreak) => {}
            _ => panic!("inline kinds differ"),
        }
    }
}

fn assert_fold_content_eq(left: &FoldBlankContent, right: &FoldBlankContent) {
    assert_eq!(left.inlines.len(), right.inlines.len());
    for (left, right) in left.inlines.iter().zip(&right.inlines) {
        assert_fold_inline_eq(left, right);
    }
}

fn assert_fold_inline_eq(left: &FoldBlankInline, right: &FoldBlankInline) {
    match (&left.kind, &right.kind) {
        (FoldBlankInlineKind::Raw(left), FoldBlankInlineKind::Raw(right)) => {
            assert_eq!(left.value, right.value);
        }
        (FoldBlankInlineKind::MathInline(left), FoldBlankInlineKind::MathInline(right)) => {
            assert_eq!(left.source, right.source);
        }
        _ => panic!("fold inline kinds differ"),
    }
}

fn assert_image_reference_eq(left: &ImageReference, right: &ImageReference) {
    match (left, right) {
        (ImageReference::RequestAttachment(left), ImageReference::RequestAttachment(right)) => {
            assert_eq!(left, right);
        }
        (ImageReference::StoredImage(left), ImageReference::StoredImage(right)) => {
            assert_eq!(left, right);
        }
        (ImageReference::ExternalUrl(left), ImageReference::ExternalUrl(right)) => {
            assert_eq!(left, right);
        }
        _ => panic!("image reference kinds differ"),
    }
}
