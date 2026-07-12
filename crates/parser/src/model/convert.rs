// Bidirectional conversion between source-oriented AST and source-free model.
// Blank extraction and restoration are centralized here to keep indices coherent.
use crate::ast;
use crate::model::{
    Blank, BlankInline, Block, Content, Document, Fold, Inline, Item, Memo, ModelDiagnostic,
    ModelError, Note, Paragraph, Qa, QaFold,
};
use crate::source::SourceRange;

const EMPTY_RANGE: SourceRange = SourceRange { start: 0, end: 0 };

#[derive(Clone, Copy)]
enum BlankPolicy {
    Allow,
    Reject(&'static str),
}

impl TryFrom<&ast::QuizFoldDocument> for Document {
    type Error = ModelDiagnostic;

    fn try_from(document: &ast::QuizFoldDocument) -> Result<Self, Self::Error> {
        document
            .items
            .iter()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map(|items| Self { items })
    }
}

impl TryFrom<ast::QuizFoldDocument> for Document {
    type Error = ModelDiagnostic;

    fn try_from(document: ast::QuizFoldDocument) -> Result<Self, Self::Error> {
        Self::try_from(&document)
    }
}

impl TryFrom<&ast::DocumentItem> for Item {
    type Error = ModelDiagnostic;

    fn try_from(item: &ast::DocumentItem) -> Result<Self, Self::Error> {
        match &item.kind {
            ast::DocumentItemKind::Quiz(quiz) => match &quiz.kind {
                ast::QuizItemKind::Qa(qa) => qa_item(qa),
                ast::QuizItemKind::Fold(fold) => fold_item(fold),
            },
            ast::DocumentItemKind::Block(block) => Ok(Self::Note(Note {
                block: block_from_ast(block, BlankPolicy::Reject("a note"), &mut Vec::new())?,
            })),
        }
    }
}

fn qa_item(qa: &ast::QaQuiz) -> Result<Item, ModelDiagnostic> {
    let question = content_from_ast(
        &qa.question,
        BlankPolicy::Reject("a Q/A question"),
        &mut Vec::new(),
    )?;
    let mut blanks = Vec::new();
    let answer = content_from_ast(&qa.answer, BlankPolicy::Allow, &mut blanks)?;

    if blanks.is_empty() {
        Ok(Item::Qa(Qa { question, answer }))
    } else {
        Ok(Item::QaFold(QaFold {
            question,
            content: answer,
            blanks,
        }))
    }
}

fn fold_item(fold: &ast::FoldQuiz) -> Result<Item, ModelDiagnostic> {
    let [block] = fold.content.blocks.as_slice() else {
        return Err(ModelDiagnostic::new(
            ModelError::InvalidFoldShape,
            ModelError::InvalidFoldShape.message(),
            Some(fold.source_range),
        ));
    };
    let ast::BlockKind::Paragraph(paragraph) = &block.kind else {
        return Err(ModelDiagnostic::new(
            ModelError::InvalidFoldShape,
            ModelError::InvalidFoldShape.message(),
            Some(block.source_range),
        ));
    };
    let mut blanks = Vec::new();
    let content = inlines_from_ast(&paragraph.inlines, BlankPolicy::Allow, &mut blanks)?;
    if blanks.is_empty() {
        return Err(ModelDiagnostic::new(
            ModelError::MissingBlank,
            "Fold quiz must contain at least one blank.",
            Some(fold.source_range),
        ));
    }
    Ok(Item::Fold(Fold { content, blanks }))
}

fn content_from_ast(
    content: &ast::QuizContent,
    policy: BlankPolicy,
    blanks: &mut Vec<Blank>,
) -> Result<Content, ModelDiagnostic> {
    content
        .blocks
        .iter()
        .map(|block| block_from_ast(block, policy, blanks))
        .collect::<Result<Vec<_>, _>>()
        .map(|blocks| Content { blocks })
}

fn block_from_ast(
    block: &ast::Block,
    policy: BlankPolicy,
    blanks: &mut Vec<Blank>,
) -> Result<Block, ModelDiagnostic> {
    match &block.kind {
        ast::BlockKind::Paragraph(paragraph) => Ok(Block::Paragraph(Paragraph {
            inlines: inlines_from_ast(&paragraph.inlines, policy, blanks)?,
        })),
        ast::BlockKind::Memo(memo) => memo
            .blocks
            .iter()
            .map(|block| block_from_ast(block, BlankPolicy::Reject("a memo"), &mut Vec::new()))
            .collect::<Result<Vec<_>, _>>()
            .map(|blocks| Block::Memo(Memo { blocks })),
        ast::BlockKind::MathBlock(value) => Ok(Block::MathBlock(value.clone())),
        ast::BlockKind::CodeBlock(value) => Ok(Block::CodeBlock(value.clone())),
        ast::BlockKind::MermaidBlock(value) => Ok(Block::MermaidBlock(value.clone())),
    }
}

fn inlines_from_ast(
    inlines: &[ast::Inline],
    policy: BlankPolicy,
    blanks: &mut Vec<Blank>,
) -> Result<Vec<Inline>, ModelDiagnostic> {
    inlines
        .iter()
        .map(|inline| match &inline.kind {
            ast::InlineKind::Raw(value) => Ok(Inline::Raw(value.value.clone())),
            ast::InlineKind::MathInline(value) => Ok(Inline::MathInline(value.source.clone())),
            ast::InlineKind::Image(value) => Ok(Inline::Image(value.clone())),
            ast::InlineKind::SoftBreak => Ok(Inline::SoftBreak),
            ast::InlineKind::FoldBlank(value) => match policy {
                BlankPolicy::Reject(location) => Err(ModelDiagnostic::new(
                    ModelError::FoldBlankNotAllowed,
                    format!("Fold blank is not allowed in {location}."),
                    Some(inline.source_range),
                )),
                BlankPolicy::Allow => {
                    let index = u32::try_from(blanks.len()).map_err(|_| {
                        ModelDiagnostic::new(
                            ModelError::TooManyBlanks,
                            ModelError::TooManyBlanks.message(),
                            Some(inline.source_range),
                        )
                    })?;
                    blanks.push(Blank {
                        answer: value
                            .answer
                            .inlines
                            .iter()
                            .map(|inline| match &inline.kind {
                                ast::FoldBlankInlineKind::Raw(value) => {
                                    BlankInline::Raw(value.value.clone())
                                }
                                ast::FoldBlankInlineKind::MathInline(value) => {
                                    BlankInline::MathInline(value.source.clone())
                                }
                            })
                            .collect(),
                    });
                    Ok(Inline::Blank(index))
                }
            },
        })
        .collect()
}

impl TryFrom<&Document> for ast::QuizFoldDocument {
    type Error = ModelDiagnostic;

    fn try_from(document: &Document) -> Result<Self, Self::Error> {
        let items = document
            .items
            .iter()
            .map(ast::DocumentItem::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self::new(items, EMPTY_RANGE))
    }
}

impl TryFrom<Document> for ast::QuizFoldDocument {
    type Error = ModelDiagnostic;

    fn try_from(document: Document) -> Result<Self, Self::Error> {
        Self::try_from(&document)
    }
}

impl TryFrom<&Item> for ast::DocumentItem {
    type Error = ModelDiagnostic;

    fn try_from(item: &Item) -> Result<Self, Self::Error> {
        let kind = match item {
            Item::Qa(qa) => ast::DocumentItemKind::Quiz(ast::QuizItem::new(
                ast::QuizItemKind::Qa(ast::QaQuiz::new(
                    content_to_ast(&qa.question, None, "a Q/A question")?,
                    content_to_ast(&qa.answer, None, "a Q/A answer")?,
                    EMPTY_RANGE,
                )),
                EMPTY_RANGE,
            )),
            Item::QaFold(qa) => {
                if qa.blanks.is_empty() {
                    return Err(ModelDiagnostic::new(
                        ModelError::MissingBlank,
                        "Q/A fold quiz must contain at least one blank.",
                        None,
                    ));
                }
                ast::DocumentItemKind::Quiz(ast::QuizItem::new(
                    ast::QuizItemKind::Qa(ast::QaQuiz::new(
                        content_to_ast(&qa.question, None, "a Q/A question")?,
                        content_to_ast(&qa.content, Some(&qa.blanks), "a Q/A fold answer")?,
                        EMPTY_RANGE,
                    )),
                    EMPTY_RANGE,
                ))
            }
            Item::Fold(fold) => {
                if fold.blanks.is_empty() {
                    return Err(ModelDiagnostic::new(
                        ModelError::MissingBlank,
                        "Fold quiz must contain at least one blank.",
                        None,
                    ));
                }
                let inlines = restore_inlines(&fold.content, Some(&fold.blanks), "a fold quiz")?;
                let paragraph = ast::Paragraph::new(inlines, EMPTY_RANGE);
                let content = ast::QuizContent::new(
                    vec![ast::Block::new(
                        ast::BlockKind::Paragraph(paragraph),
                        EMPTY_RANGE,
                    )],
                    EMPTY_RANGE,
                );
                ast::DocumentItemKind::Quiz(ast::QuizItem::new(
                    ast::QuizItemKind::Fold(ast::FoldQuiz::new(content, EMPTY_RANGE)),
                    EMPTY_RANGE,
                ))
            }
            Item::Note(note) => {
                ast::DocumentItemKind::Block(block_to_ast(&note.block, None, "a note")?)
            }
        };
        Ok(Self::new(kind, EMPTY_RANGE))
    }
}

fn content_to_ast(
    content: &Content,
    blanks: Option<&[Blank]>,
    location: &'static str,
) -> Result<ast::QuizContent, ModelDiagnostic> {
    let mut next_blank = 0;
    let blocks = content
        .blocks
        .iter()
        .map(|block| block_to_ast_with_cursor(block, blanks, location, &mut next_blank))
        .collect::<Result<Vec<_>, _>>()?;
    validate_blank_count(blanks, next_blank)?;
    Ok(ast::QuizContent::new(blocks, EMPTY_RANGE))
}

fn block_to_ast(
    block: &Block,
    blanks: Option<&[Blank]>,
    location: &'static str,
) -> Result<ast::Block, ModelDiagnostic> {
    let mut next_blank = 0;
    let block = block_to_ast_with_cursor(block, blanks, location, &mut next_blank)?;
    validate_blank_count(blanks, next_blank)?;
    Ok(block)
}

fn block_to_ast_with_cursor(
    block: &Block,
    blanks: Option<&[Blank]>,
    location: &'static str,
    next_blank: &mut u32,
) -> Result<ast::Block, ModelDiagnostic> {
    let kind = match block {
        Block::Paragraph(paragraph) => ast::BlockKind::Paragraph(ast::Paragraph::new(
            restore_inlines_with_cursor(&paragraph.inlines, blanks, location, next_blank)?,
            EMPTY_RANGE,
        )),
        Block::Memo(memo) => {
            let blocks = memo
                .blocks
                .iter()
                .map(|block| block_to_ast(block, None, "a memo"))
                .collect::<Result<Vec<_>, _>>()?;
            ast::BlockKind::Memo(ast::MemoBlock::new(blocks, EMPTY_RANGE))
        }
        Block::MathBlock(value) => ast::BlockKind::MathBlock(value.clone()),
        Block::CodeBlock(value) => ast::BlockKind::CodeBlock(value.clone()),
        Block::MermaidBlock(value) => ast::BlockKind::MermaidBlock(value.clone()),
    };
    Ok(ast::Block::new(kind, EMPTY_RANGE))
}

fn restore_inlines(
    inlines: &[Inline],
    blanks: Option<&[Blank]>,
    location: &'static str,
) -> Result<Vec<ast::Inline>, ModelDiagnostic> {
    let mut next_blank = 0;
    let inlines = restore_inlines_with_cursor(inlines, blanks, location, &mut next_blank)?;
    validate_blank_count(blanks, next_blank)?;
    Ok(inlines)
}

fn restore_inlines_with_cursor(
    inlines: &[Inline],
    blanks: Option<&[Blank]>,
    location: &'static str,
    next_blank: &mut u32,
) -> Result<Vec<ast::Inline>, ModelDiagnostic> {
    inlines
        .iter()
        .map(|inline| {
            let kind = match inline {
                Inline::Raw(value) => ast::InlineKind::Raw(ast::Raw {
                    value: value.clone(),
                }),
                Inline::MathInline(value) => ast::InlineKind::MathInline(ast::MathInline {
                    source: value.clone(),
                }),
                Inline::Image(value) => ast::InlineKind::Image(value.clone()),
                Inline::SoftBreak => ast::InlineKind::SoftBreak,
                Inline::Blank(index) => {
                    let Some(blanks) = blanks else {
                        return Err(ModelDiagnostic::new(
                            ModelError::FoldBlankNotAllowed,
                            format!("Fold blank is not allowed in {location}."),
                            None,
                        ));
                    };
                    if *index != *next_blank {
                        return Err(ModelDiagnostic::new(
                            ModelError::BlankOutOfOrder,
                            format!(
                                "Blank reference {index} is out of order; expected {next_blank}."
                            ),
                            None,
                        ));
                    }
                    let blank = blanks.get(*index as usize).ok_or_else(|| {
                        ModelDiagnostic::new(
                            ModelError::MissingBlankReference,
                            format!("Blank reference {index} does not exist."),
                            None,
                        )
                    })?;
                    *next_blank += 1;
                    ast::InlineKind::FoldBlank(ast::FoldBlank::new(
                        ast::FoldBlankContent::new(
                            blank
                                .answer
                                .iter()
                                .map(|inline| match inline {
                                    BlankInline::Raw(value) => ast::FoldBlankInline::new(
                                        ast::FoldBlankInlineKind::Raw(ast::Raw {
                                            value: value.clone(),
                                        }),
                                        EMPTY_RANGE,
                                    ),
                                    BlankInline::MathInline(value) => ast::FoldBlankInline::new(
                                        ast::FoldBlankInlineKind::MathInline(ast::MathInline {
                                            source: value.clone(),
                                        }),
                                        EMPTY_RANGE,
                                    ),
                                })
                                .collect(),
                            EMPTY_RANGE,
                        ),
                        EMPTY_RANGE,
                    ))
                }
            };
            Ok(ast::Inline::new(kind, EMPTY_RANGE))
        })
        .collect()
}

fn validate_blank_count(blanks: Option<&[Blank]>, used: u32) -> Result<(), ModelDiagnostic> {
    let Some(blanks) = blanks else {
        return Ok(());
    };
    let used = used as usize;
    if used < blanks.len() {
        let unused = blanks.len() - used;
        return Err(ModelDiagnostic::new(
            ModelError::UnusedBlanks,
            format!("{unused} blank answers are not referenced."),
            None,
        ));
    }
    Ok(())
}
