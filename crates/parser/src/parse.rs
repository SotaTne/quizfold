use crate::ast::{
    AttachmentKey, Block, BlockKind, CodeBlock, DocumentItem, DocumentItemKind, ExternalImageUrl,
    FoldBlank, FoldBlankContent, FoldBlankInline, FoldBlankInlineKind, FoldQuiz, Image,
    ImageReference, Inline, InlineKind, MathBlock, MathInline, MermaidBlock, Paragraph, QaQuiz,
    QuizContent, QuizFoldDocument, QuizItem, QuizItemKind, Raw, StoredImageId,
};
use crate::diagnostics::Diagnostic;
use crate::errors::ParseError;
use crate::lexer::{lex_with_offset, Token, TokenKind};
use crate::source::SourceRange;
use crate::{ParseResult, ParseStats, References};

pub(crate) fn parse(input: &str) -> ParseResult {
    Parser::new(input).parse()
}

struct Parser<'a> {
    input: &'a str,
    cursor: usize,
    diagnostics: Vec<Diagnostic>,
    references: References,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            cursor: 0,
            diagnostics: Vec::new(),
            references: References::default(),
        }
    }

    fn parse(mut self) -> ParseResult {
        let mut items = Vec::new();

        while self.cursor < self.input.len() {
            self.skip_blank_lines();
            if self.cursor >= self.input.len() {
                break;
            }

            let line = self.current_line();
            let item = if line.text.starts_with("? ") {
                self.parse_qa()
            } else if line.text.starts_with("! ") {
                self.parse_fold()
            } else {
                self.parse_block_item()
            };
            items.push(item);
        }

        ParseResult {
            document: QuizFoldDocument::new(items, SourceRange::new(0, self.input.len())),
            diagnostics: self.diagnostics,
            references: self.references,
            stats: ParseStats {
                byte_len: self.input.len(),
            },
        }
    }

    fn parse_qa(&mut self) -> DocumentItem {
        let start = self.cursor;
        let question_line = self.take_line();
        let question_start = question_line.start + 2;
        let question = self.parse_content(
            &self.input[question_start..question_line.content_end],
            question_start,
        );

        let separator = self.current_line();
        if separator.text != "---" {
            self.diagnostics.push(Diagnostic::new(
                ParseError::MissingAnswerSeparator,
                SourceRange::new(separator.start, separator.content_end),
            ));
            let end = question_line.end;
            let answer = QuizContent::new(Vec::new(), SourceRange::new(end, end));
            let quiz = QaQuiz::new(question, answer, SourceRange::new(start, end));
            return quiz_item(QuizItemKind::Qa(quiz), start, end);
        }

        self.take_line();
        let answer_start = self.cursor;
        let answer_end = self.take_until_item_boundary();
        let answer = self.parse_content(&self.input[answer_start..answer_end], answer_start);
        let quiz = QaQuiz::new(question, answer, SourceRange::new(start, answer_end));
        quiz_item(QuizItemKind::Qa(quiz), start, answer_end)
    }

    fn parse_fold(&mut self) -> DocumentItem {
        let line = self.take_line();
        let content_start = line.start + 2;
        let content =
            self.parse_content(&self.input[content_start..line.content_end], content_start);

        if !contains_fold_blank(&content) {
            self.diagnostics.push(Diagnostic::new(
                ParseError::FoldQuizWithoutBlank,
                SourceRange::new(line.start, line.content_end),
            ));
        }

        let quiz = FoldQuiz::new(content, SourceRange::new(line.start, line.end));
        quiz_item(QuizItemKind::Fold(quiz), line.start, line.end)
    }

    fn parse_block_item(&mut self) -> DocumentItem {
        let start = self.cursor;
        let end = self.take_until_item_boundary();
        let content = self.parse_content(&self.input[start..end], start);
        let block = content.blocks.into_iter().next().unwrap_or_else(|| {
            let range = SourceRange::new(start, end);
            Block::new(
                BlockKind::Paragraph(Paragraph::new(Vec::new(), range)),
                range,
            )
        });
        DocumentItem::new(DocumentItemKind::Block(block), SourceRange::new(start, end))
    }

    fn parse_content(&mut self, source: &str, offset: usize) -> QuizContent {
        let range = SourceRange::new(offset, offset + source.len());
        let source = source.trim_end_matches(['\r', '\n']);

        if source.starts_with("$$") {
            if let Some(block) = self.parse_math_block(source, offset) {
                return QuizContent::new(vec![block], range);
            }
        } else if source.starts_with("```") {
            if let Some(block) = self.parse_fenced_block(source, offset) {
                return QuizContent::new(vec![block], range);
            }
        }

        let paragraph = Paragraph::new(self.parse_inlines(source, offset), range);
        QuizContent::new(
            vec![Block::new(BlockKind::Paragraph(paragraph), range)],
            range,
        )
    }

    fn parse_math_block(&mut self, source: &str, offset: usize) -> Option<Block> {
        let (opening, body, closed) = delimited_body(source, "$$")?;
        if opening != "$$" {
            return None;
        }
        if !closed {
            self.diagnostics.push(Diagnostic::new(
                ParseError::UnclosedBlock,
                SourceRange::new(offset, offset + source.len()),
            ));
            return None;
        }
        let range = SourceRange::new(offset, offset + source.len());
        Some(Block::new(
            BlockKind::MathBlock(MathBlock {
                source: body.into(),
            }),
            range,
        ))
    }

    fn parse_fenced_block(&mut self, source: &str, offset: usize) -> Option<Block> {
        let (opening, body, closed) = delimited_body(source, "```")?;
        let language = opening.strip_prefix("```")?.trim();
        if !closed {
            self.diagnostics.push(Diagnostic::new(
                ParseError::UnclosedBlock,
                SourceRange::new(offset, offset + source.len()),
            ));
            return None;
        }

        let range = SourceRange::new(offset, offset + source.len());
        let kind = match language {
            "mermaid" | "mmd" => BlockKind::MermaidBlock(MermaidBlock {
                source: body.into(),
            }),
            other => BlockKind::CodeBlock(CodeBlock {
                language: (!other.is_empty()).then(|| other.into()),
                source: body.into(),
            }),
        };
        Some(Block::new(kind, range))
    }

    fn parse_inlines(&mut self, source: &str, offset: usize) -> Vec<Inline> {
        let tokens = lex_with_offset(source, offset);
        let mut inlines = Vec::new();
        let mut index = 0;

        while index < tokens.len() {
            let token = tokens[index];
            match token.kind {
                TokenKind::Raw => {
                    inlines.push(Inline::new(
                        InlineKind::Raw(Raw {
                            value: token_text(source, offset, token).into(),
                        }),
                        token.source_range,
                    ));
                    index += 1;
                }
                TokenKind::Newline => {
                    inlines.push(Inline::new(InlineKind::SoftBreak, token.source_range));
                    index += 1;
                }
                TokenKind::FoldBlankStart => {
                    let Some(end_index) = find_token(&tokens, index + 1, TokenKind::FoldBlankEnd)
                    else {
                        let end = SourceRange::new(token.source_range.start, offset + source.len());
                        self.diagnostics
                            .push(Diagnostic::new(ParseError::UnclosedFoldBlank, end));
                        push_raw(
                            &mut inlines,
                            source,
                            token.source_range.start - offset,
                            source.len(),
                            offset,
                        );
                        return inlines;
                    };
                    let closing = tokens[end_index];
                    let answer_range =
                        SourceRange::new(token.source_range.end, closing.source_range.start);
                    let answer = FoldBlankContent::new(
                        fold_blank_inlines(source, offset, &tokens[index + 1..end_index]),
                        answer_range,
                    );
                    let range =
                        SourceRange::new(token.source_range.start, closing.source_range.end);
                    inlines.push(Inline::new(
                        InlineKind::FoldBlank(FoldBlank::new(answer, range)),
                        range,
                    ));
                    index = end_index + 1;
                }
                TokenKind::MathInlineDelimiter => {
                    let Some(end_index) =
                        find_token(&tokens, index + 1, TokenKind::MathInlineDelimiter)
                    else {
                        let end = SourceRange::new(token.source_range.start, offset + source.len());
                        self.diagnostics
                            .push(Diagnostic::new(ParseError::UnclosedMathInline, end));
                        push_raw(
                            &mut inlines,
                            source,
                            token.source_range.start - offset,
                            source.len(),
                            offset,
                        );
                        return inlines;
                    };
                    let closing = tokens[end_index];
                    let math_range =
                        SourceRange::new(token.source_range.start, closing.source_range.end);
                    let source_range =
                        SourceRange::new(token.source_range.end, closing.source_range.start);
                    inlines.push(Inline::new(
                        InlineKind::MathInline(MathInline {
                            source: range_text(source, offset, source_range).into(),
                        }),
                        math_range,
                    ));
                    index = end_index + 1;
                }
                TokenKind::Image { alt, destination } => {
                    let alt_text = range_text(source, offset, alt);
                    if alt_text.is_empty() {
                        self.diagnostics
                            .push(Diagnostic::new(ParseError::EmptyImageAlt, alt));
                    }

                    let destination_text = range_text(source, offset, destination);
                    let reference =
                        if let Some(key) = destination_text.strip_prefix("qf-attachment:") {
                            if key.is_empty() {
                                None
                            } else {
                                let key = AttachmentKey::from_source(key);
                                self.references.request_attachments.push(key.clone());
                                Some(ImageReference::RequestAttachment(key))
                            }
                        } else if let Some(id) = destination_text.strip_prefix("qf-stored:") {
                            if id.is_empty() {
                                None
                            } else {
                                let id = StoredImageId::from_source(id);
                                self.references.stored_images.push(id.clone());
                                Some(ImageReference::StoredImage(id))
                            }
                        } else if is_http_image_url(destination_text) {
                            let url = ExternalImageUrl::from_source(destination_text);
                            self.references.external_images.push(url.clone());
                            Some(ImageReference::ExternalUrl(url))
                        } else {
                            None
                        };

                    let Some(reference) = reference else {
                        self.diagnostics.push(Diagnostic::new(
                            ParseError::InvalidImageReference,
                            destination,
                        ));
                        inlines.push(Inline::new(
                            InlineKind::Raw(Raw {
                                value: token_text(source, offset, token).into(),
                            }),
                            token.source_range,
                        ));
                        index += 1;
                        continue;
                    };

                    inlines.push(Inline::new(
                        InlineKind::Image(Image {
                            alt: Raw {
                                value: alt_text.into(),
                            },
                            reference,
                        }),
                        token.source_range,
                    ));
                    index += 1;
                }
                _ => {
                    inlines.push(Inline::new(
                        InlineKind::Raw(Raw {
                            value: token_text(source, offset, token).into(),
                        }),
                        token.source_range,
                    ));
                    index += 1;
                }
            }
        }

        inlines
    }

    fn take_until_item_boundary(&mut self) -> usize {
        let mut end = self.cursor;
        let mut first_line = true;
        let mut delimiter: Option<&str> = None;

        while self.cursor < self.input.len() {
            let line = self.current_line();
            if delimiter.is_none() && line.text.is_empty() {
                break;
            }
            if delimiter.is_none()
                && !first_line
                && (line.text.starts_with("? ") || line.text.starts_with("! "))
            {
                break;
            }

            match delimiter {
                Some(expected) if line.text == expected => delimiter = None,
                None if line.text.starts_with("```") => delimiter = Some("```"),
                None if line.text == "$$" => delimiter = Some("$$"),
                _ => {}
            }

            end = line.end;
            self.cursor = line.end;
            first_line = false;
        }
        end
    }

    fn skip_blank_lines(&mut self) {
        while self.cursor < self.input.len() && self.current_line().text.is_empty() {
            self.cursor = self.current_line().end;
        }
    }

    fn take_line(&mut self) -> Line<'a> {
        let line = self.current_line();
        self.cursor = line.end;
        line
    }

    fn current_line(&self) -> Line<'a> {
        line_at(self.input, self.cursor)
    }
}

#[derive(Clone, Copy)]
struct Line<'a> {
    text: &'a str,
    start: usize,
    content_end: usize,
    end: usize,
}

fn line_at(input: &str, start: usize) -> Line<'_> {
    let remaining = &input[start..];
    let relative_end = remaining.find('\n').unwrap_or(remaining.len());
    let content_end = start + relative_end;
    let end = if content_end < input.len() {
        content_end + 1
    } else {
        content_end
    };
    let text = input[start..content_end].trim_end_matches('\r');
    Line {
        text,
        start,
        content_end: start + text.len(),
        end,
    }
}

fn quiz_item(kind: QuizItemKind, start: usize, end: usize) -> DocumentItem {
    let range = SourceRange::new(start, end);
    DocumentItem::new(DocumentItemKind::Quiz(QuizItem::new(kind, range)), range)
}

fn push_raw(inlines: &mut Vec<Inline>, source: &str, start: usize, end: usize, offset: usize) {
    if start >= end {
        return;
    }
    let range = SourceRange::new(offset + start, offset + end);
    inlines.push(Inline::new(
        InlineKind::Raw(Raw {
            value: source[start..end].into(),
        }),
        range,
    ));
}

fn fold_blank_inlines(source: &str, offset: usize, tokens: &[Token]) -> Vec<FoldBlankInline> {
    let mut inlines = Vec::new();
    let mut index = 0;

    while index < tokens.len() {
        let token = tokens[index];
        if token.kind == TokenKind::MathInlineDelimiter {
            if let Some(end_index) = find_token(tokens, index + 1, TokenKind::MathInlineDelimiter) {
                let closing = tokens[end_index];
                let range = SourceRange::new(token.source_range.start, closing.source_range.end);
                let math_source =
                    SourceRange::new(token.source_range.end, closing.source_range.start);
                inlines.push(FoldBlankInline::new(
                    FoldBlankInlineKind::MathInline(MathInline {
                        source: range_text(source, offset, math_source).into(),
                    }),
                    range,
                ));
                index = end_index + 1;
                continue;
            }
        }

        inlines.push(FoldBlankInline::new(
            FoldBlankInlineKind::Raw(Raw {
                value: token_text(source, offset, token).into(),
            }),
            token.source_range,
        ));
        index += 1;
    }

    inlines
}

fn find_token(tokens: &[Token], start: usize, kind: TokenKind) -> Option<usize> {
    tokens[start..]
        .iter()
        .position(|token| token.kind == kind)
        .map(|relative| start + relative)
}

fn token_text(source: &str, offset: usize, token: Token) -> &str {
    range_text(source, offset, token.source_range)
}

fn range_text(source: &str, offset: usize, range: SourceRange) -> &str {
    &source[range.start - offset..range.end - offset]
}

fn is_http_image_url(value: &str) -> bool {
    let remainder = value
        .strip_prefix("https://")
        .or_else(|| value.strip_prefix("http://"));

    remainder.is_some_and(|remainder| {
        !remainder.is_empty()
            && !remainder.starts_with('/')
            && !remainder.chars().any(char::is_whitespace)
    })
}

fn delimited_body<'a>(source: &'a str, delimiter: &str) -> Option<(&'a str, &'a str, bool)> {
    let opening_end = source.find('\n')?;
    let opening = source[..opening_end].trim_end_matches('\r');
    let Some(last_line_start) = source.rfind('\n') else {
        return Some((opening, "", false));
    };
    let closing = source[last_line_start + 1..].trim_end_matches('\r');
    let closed = closing == delimiter;
    let body = if closed {
        &source[opening_end + 1..last_line_start]
    } else {
        &source[opening_end + 1..]
    };
    Some((opening, body, closed))
}

fn contains_fold_blank(content: &QuizContent) -> bool {
    content.blocks.iter().any(|block| match &block.kind {
        BlockKind::Paragraph(paragraph) => paragraph
            .inlines
            .iter()
            .any(|inline| matches!(inline.kind, InlineKind::FoldBlank(_))),
        _ => false,
    })
}
