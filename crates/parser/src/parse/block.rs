// Block-level QuizFold grammar.
// This file parses quizzes, memo blocks, math/code fences, and paragraphs.
use super::{ContentContext, Parser};
use crate::ast::{
    Block, BlockKind, CodeBlock, DocumentItem, DocumentItemKind, FoldQuiz, MathBlock, MemoBlock,
    MermaidBlock, Paragraph, QaQuiz, QuizContent, QuizItem, QuizItemKind,
};
use crate::diagnostics::Diagnostic;
use crate::errors::ParseError;
use crate::lexer::TokenKind;
use crate::source::SourceRange;

impl Parser<'_> {
    /// QaQuiz <- QUESTION_MARKER Inline* Block* ANSWER_SEPARATOR Block*
    /// QaQuiz -> DocumentItemKind::Quiz(QuizItemKind::Qa)
    pub(super) fn parse_qa(&mut self) -> DocumentItem {
        let question = self.parse_question_section();
        let Some(separator) = self.consume_answer_separator() else {
            return self.incomplete_qa(question);
        };
        let answer_start = self
            .cursor
            .current()
            .map_or(separator.full_end, |line| line.content_range.start);
        let answer = self.parse_answer_blocks();
        self.complete_qa(question, separator, answer_start, answer)
    }

    fn parse_question_section(&mut self) -> QuestionSection {
        let opening = self.current_line();
        let marker = self.tokens[opening.token_start];
        let missing_separator_range = self.missing_separator_range(marker.source_range.end);
        let mut blocks = self.parse_marker_line(opening, marker.source_range.end);
        self.cursor.advance();
        self.parse_question_blocks(&mut blocks);
        QuestionSection {
            quiz_start: opening.content_range.start,
            content_start: marker.source_range.end,
            missing_separator_range,
            blocks,
        }
    }

    fn consume_answer_separator(&mut self) -> Option<super::cursor::LexedLine> {
        if !self.current_is(TokenKind::AnswerSeparator) {
            return None;
        }
        let separator = self.current_line();
        self.cursor.advance();
        Some(separator)
    }

    fn missing_separator_range(&self, marker_end: usize) -> SourceRange {
        self.line_at(self.cursor.position() + 1)
            .map_or(SourceRange::new(marker_end, marker_end), |line| {
                line.content_range
            })
    }

    fn parse_question_blocks(&mut self, blocks: &mut Vec<Block>) {
        while !self.cursor.eof()
            && !self.current_is(TokenKind::AnswerSeparator)
            && !self.is_quiz_boundary()
        {
            if self.is_blank_line() {
                self.cursor.advance();
            } else if let Some(block) = self.parse_block(ContentContext::Question) {
                blocks.push(block);
            }
        }
    }

    fn incomplete_qa(&mut self, question: QuestionSection) -> DocumentItem {
        self.diagnostics.push(Diagnostic::new(
            ParseError::MissingAnswerSeparator,
            question.missing_separator_range,
        ));
        let end = self
            .cursor
            .current()
            .map_or(self.input.len(), |line| line.content_range.start);
        let content = QuizContent::new(
            question.blocks,
            SourceRange::new(question.content_start, end),
        );
        let answer = QuizContent::new(Vec::new(), SourceRange::new(end, end));
        qa_item(content, answer, question.quiz_start, end)
    }

    fn parse_answer_blocks(&mut self) -> Vec<Block> {
        let mut blocks = Vec::new();
        while !self.cursor.eof() && !self.is_quiz_boundary() && !self.is_blank_line() {
            if let Some(block) = self.parse_block(ContentContext::Answer) {
                blocks.push(block);
            }
        }
        blocks
    }

    fn complete_qa(
        &self,
        question: QuestionSection,
        separator: super::cursor::LexedLine,
        answer_start: usize,
        answer_blocks: Vec<Block>,
    ) -> DocumentItem {
        let end = answer_blocks
            .last()
            .map_or(answer_start, |block| block.source_range.end);
        let content = QuizContent::new(
            question.blocks,
            SourceRange::new(question.content_start, separator.content_range.start),
        );
        let answer = QuizContent::new(answer_blocks, SourceRange::new(answer_start, end));
        qa_item(content, answer, question.quiz_start, end)
    }

    /// FoldQuiz <- FOLD_MARKER Inline* FoldBlank Inline*
    /// FoldQuiz -> DocumentItemKind::Quiz(QuizItemKind::Fold)
    pub(super) fn parse_fold(&mut self) -> DocumentItem {
        let line = self.current_line();
        let marker = self.tokens[line.token_start];
        let range = SourceRange::new(line.content_range.start, line.full_end);
        let content_range = SourceRange::new(marker.source_range.end, line.content_range.end);
        let content =
            self.parse_content_tokens(line.token_start + 1, line.token_end, content_range);
        self.cursor.advance();

        if !contains_fold_blank(&content) {
            self.diagnostics.push(Diagnostic::new(
                ParseError::FoldQuizWithoutBlank,
                line.content_range,
            ));
        }

        quiz_item(QuizItemKind::Fold(FoldQuiz::new(content, range)), range)
    }

    /// MemoBlock <- MEMO_START MemoBody MEMO_END
    /// MemoBlock -> DocumentItemKind::Block(BlockKind::Memo)
    pub(super) fn parse_memo_item(&mut self) -> DocumentItem {
        let block = self.parse_memo_block();
        let range = block.source_range;
        DocumentItem::new(DocumentItemKind::Block(block), range)
    }

    fn parse_memo_block(&mut self) -> Block {
        let start = self.current_line().content_range.start;
        let blocks = match self.parse_memo() {
            Ok(blocks) => blocks,
            Err(diagnostic) => {
                self.diagnostics.push(diagnostic);
                Vec::new()
            }
        };
        let end = self.cursor.previous_end().unwrap_or(start);
        let range = SourceRange::new(start, end);
        Block::new(BlockKind::Memo(MemoBlock::new(blocks, range)), range)
    }

    /// MemoBody <- (BlankLine / MathBlock / FencedBlock / Paragraph)*
    /// MemoBody -> Vec<Block>
    pub(super) fn parse_memo(&mut self) -> Result<Vec<Block>, Diagnostic> {
        let opening = self.current_line();
        self.cursor.advance();
        let mut blocks = Vec::new();

        while !self.cursor.eof() {
            if self.current_is(TokenKind::MemoEnd) {
                self.cursor.advance();
                return Ok(blocks);
            }
            self.parse_memo_entry(&mut blocks)?;
        }

        Err(Diagnostic::new(
            ParseError::UnclosedMemo,
            SourceRange::new(opening.content_range.start, self.input.len()),
        ))
    }

    fn parse_memo_entry(&mut self, blocks: &mut Vec<Block>) -> Result<(), Diagnostic> {
        if self.current_is(TokenKind::MemoStart) {
            return Err(self.nested_memo_error());
        }
        if self.is_blank_line() {
            self.cursor.advance();
        } else if let Some(block) = self.parse_block(ContentContext::Memo) {
            blocks.push(block);
        }
        Ok(())
    }

    fn nested_memo_error(&mut self) -> Diagnostic {
        let range = self.current_line().content_range;
        self.recover_memo();
        Diagnostic::new(ParseError::NestedMemo, range)
    }

    fn recover_memo(&mut self) {
        while !self.cursor.eof() {
            let is_end = self.current_is(TokenKind::MemoEnd);
            self.cursor.advance();
            if is_end {
                break;
            }
        }
    }

    /// BlockItem <- Block
    /// BlockItem -> DocumentItemKind::Block
    pub(super) fn parse_block_item(&mut self) -> DocumentItem {
        let start = self.current_line().content_range.start;
        let block = self
            .parse_block(ContentContext::Document)
            .unwrap_or_else(|| empty_paragraph(start));
        let range = block.source_range;
        DocumentItem::new(DocumentItemKind::Block(block), range)
    }

    /// Block <- MemoBlock / MathBlock / FencedBlock / Paragraph
    /// Block -> BlockKind
    fn parse_block(&mut self, context: ContentContext) -> Option<Block> {
        match self.current_kind() {
            Some(TokenKind::MemoStart) if context != ContentContext::Memo => {
                Some(self.parse_memo_block())
            }
            Some(TokenKind::MathBlockDelimiter) => self.parse_math_block(),
            Some(TokenKind::CodeFenceStart { .. }) => self.parse_fenced_block(),
            _ => Some(self.parse_paragraph(context)),
        }
    }

    /// Paragraph <- Inline* (NEWLINE Inline*)*
    /// Paragraph -> BlockKind::Paragraph
    fn parse_paragraph(&mut self, context: ContentContext) -> Block {
        let first_line = self.cursor.position();
        let token_start = self.current_line().token_start;
        self.cursor.advance();

        while !self.cursor.eof() && !self.is_blank_line() && !self.is_block_boundary(context) {
            self.cursor.advance();
        }

        self.paragraph_from_lines(first_line, token_start)
    }

    fn parse_marker_line(
        &mut self,
        line: super::cursor::LexedLine,
        content_start: usize,
    ) -> Vec<Block> {
        if line.token_start + 1 >= line.token_end {
            return Vec::new();
        }
        let range = SourceRange::new(content_start, line.content_range.end);
        vec![self.paragraph_from_tokens(line.token_start + 1, line.token_end, range)]
    }

    fn paragraph_from_lines(&mut self, first_line: usize, token_start: usize) -> Block {
        let last_line = self.cursor.position().saturating_sub(1);
        let line = self
            .line_at(last_line)
            .expect("paragraph must contain a line");
        let start = self.tokens.get(token_start).map_or_else(
            || self.line_at(first_line).unwrap().content_range.start,
            |token| token.source_range.start,
        );
        self.paragraph_from_tokens(
            token_start,
            line.token_end,
            SourceRange::new(start, line.content_range.end),
        )
    }

    fn paragraph_from_tokens(
        &mut self,
        token_start: usize,
        token_end: usize,
        range: SourceRange,
    ) -> Block {
        let paragraph = Paragraph::new(self.parse_inlines_range(token_start, token_end), range);
        Block::new(BlockKind::Paragraph(paragraph), range)
    }

    /// MathBlock <- MATH_BLOCK_DELIMITER Raw* MATH_BLOCK_DELIMITER
    /// MathBlock -> BlockKind::MathBlock
    fn parse_math_block(&mut self) -> Option<Block> {
        let delimited = self.consume_delimited_block(
            TokenKind::MathBlockDelimiter,
            TokenKind::MathBlockDelimiter,
        )?;
        Some(Block::new(
            BlockKind::MathBlock(MathBlock {
                source: delimited.body.into(),
            }),
            delimited.range,
        ))
    }

    /// FencedBlock <- CODE_FENCE_START Raw* CODE_FENCE_END
    /// FencedBlock -> BlockKind::MermaidBlock / BlockKind::CodeBlock
    fn parse_fenced_block(&mut self) -> Option<Block> {
        let opening = self.tokens[self.current_line().token_start];
        let TokenKind::CodeFenceStart { info } = opening.kind else {
            return None;
        };
        let language: Option<Box<str>> =
            info.map(|range| self.input[range.start..range.end].into());
        let delimited = self.consume_delimited_block(opening.kind, TokenKind::CodeFenceEnd)?;
        let kind = match language.as_deref() {
            Some("mermaid" | "mmd") => BlockKind::MermaidBlock(MermaidBlock {
                source: delimited.body.into(),
            }),
            _ => BlockKind::CodeBlock(CodeBlock {
                language,
                source: delimited.body.into(),
            }),
        };
        Some(Block::new(kind, delimited.range))
    }

    fn consume_delimited_block(
        &mut self,
        opening_kind: TokenKind,
        closing_kind: TokenKind,
    ) -> Option<DelimitedBlock<'_>> {
        debug_assert_eq!(self.current_kind(), Some(opening_kind));
        let opening = self.current_line();
        self.cursor.advance();
        if !self.seek(closing_kind) {
            self.report_unclosed_block(opening.content_range.start);
            return None;
        }
        let closing = self.current_line();
        self.cursor.advance();
        Some(DelimitedBlock {
            body: self.input[opening.full_end..closing.content_range.start]
                .trim_end_matches(['\r', '\n']),
            range: SourceRange::new(opening.content_range.start, closing.full_end),
        })
    }

    fn seek(&mut self, kind: TokenKind) -> bool {
        while !self.cursor.eof() && !self.current_is(kind) {
            self.cursor.advance();
        }
        !self.cursor.eof()
    }

    fn report_unclosed_block(&mut self, start: usize) {
        self.diagnostics.push(Diagnostic::new(
            ParseError::UnclosedBlock,
            SourceRange::new(start, self.source_content_end()),
        ));
    }

    fn is_block_boundary(&self, context: ContentContext) -> bool {
        match self.current_kind() {
            Some(
                TokenKind::MathBlockDelimiter
                | TokenKind::CodeFenceStart { .. }
                | TokenKind::MemoStart
                | TokenKind::MemoEnd,
            ) => true,
            Some(TokenKind::AnswerSeparator) => context == ContentContext::Question,
            Some(TokenKind::QuestionMarker | TokenKind::FoldMarker) => {
                context != ContentContext::Memo
            }
            _ => false,
        }
    }

    fn is_quiz_boundary(&self) -> bool {
        matches!(
            self.current_kind(),
            Some(TokenKind::QuestionMarker | TokenKind::FoldMarker)
        )
    }
}

struct DelimitedBlock<'a> {
    body: &'a str,
    range: SourceRange,
}

struct QuestionSection {
    quiz_start: usize,
    content_start: usize,
    missing_separator_range: SourceRange,
    blocks: Vec<Block>,
}

fn qa_item(question: QuizContent, answer: QuizContent, start: usize, end: usize) -> DocumentItem {
    let range = SourceRange::new(start, end);
    quiz_item(
        QuizItemKind::Qa(QaQuiz::new(question, answer, range)),
        range,
    )
}

fn quiz_item(kind: QuizItemKind, range: SourceRange) -> DocumentItem {
    DocumentItem::new(DocumentItemKind::Quiz(QuizItem::new(kind, range)), range)
}

fn empty_paragraph(position: usize) -> Block {
    let range = SourceRange::new(position, position);
    Block::new(
        BlockKind::Paragraph(Paragraph::new(Vec::new(), range)),
        range,
    )
}

fn contains_fold_blank(content: &QuizContent) -> bool {
    content.blocks.iter().any(|block| match &block.kind {
        BlockKind::Paragraph(paragraph) => paragraph
            .inlines
            .iter()
            .any(|inline| matches!(inline.kind, crate::ast::InlineKind::FoldBlank(_))),
        _ => false,
    })
}
