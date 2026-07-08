// Parser root and document-level dispatch.
// It consumes lexer tokens, collects diagnostics, and builds ParseResult.
mod block;
mod cursor;
mod inline;

use self::cursor::LineCursor;
use crate::ast::QuizFoldDocument;
use crate::diagnostics::Diagnostic;
use crate::errors::ParseError;
use crate::lexer::{lex, Token, TokenKind};
use crate::source::SourceRange;
use crate::{ParseResult, ParseStats, References};

pub(crate) fn parse(input: &str) -> ParseResult {
    Parser::new(input).parse()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContentContext {
    Document,
    Question,
    Answer,
    Memo,
}

struct Parser<'a> {
    input: &'a str,
    tokens: Vec<Token>,
    cursor: LineCursor,
    diagnostics: Vec<Diagnostic>,
    references: References,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let tokens = lex(input);
        Self {
            input,
            cursor: LineCursor::new(input, &tokens),
            tokens,
            diagnostics: Vec::new(),
            references: References::default(),
        }
    }

    /// Document <- (BlankLine / QaQuiz / FoldQuiz / MemoBlock / Block)* EOF
    /// Document -> QuizFoldDocument
    fn parse(mut self) -> ParseResult {
        let mut items = Vec::new();

        while !self.cursor.eof() {
            self.skip_blank_lines();
            if self.cursor.eof() {
                break;
            }
            items.push(self.parse_document_item());
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

    fn parse_document_item(&mut self) -> crate::ast::DocumentItem {
        match self.current_kind() {
            Some(TokenKind::QuestionMarker) => self.parse_qa(),
            Some(TokenKind::FoldMarker) => self.parse_fold(),
            Some(TokenKind::MemoStart) => self.parse_memo_item(),
            Some(TokenKind::MemoEnd) => {
                let range = self.current_line().content_range;
                self.diagnostics
                    .push(Diagnostic::new(ParseError::UnexpectedMemoEnd, range));
                self.parse_block_item()
            }
            _ => self.parse_block_item(),
        }
    }

    fn current_line(&self) -> cursor::LexedLine {
        self.cursor
            .current()
            .expect("parser cursor must be in bounds")
    }

    fn line_at(&self, position: usize) -> Option<cursor::LexedLine> {
        self.cursor.get(position)
    }

    fn current_kind(&self) -> Option<TokenKind> {
        self.kind_at(self.cursor.position())
    }

    fn kind_at(&self, position: usize) -> Option<TokenKind> {
        let line = self.line_at(position)?;
        (line.token_start < line.token_end).then(|| self.tokens[line.token_start].kind)
    }

    fn current_is(&self, kind: TokenKind) -> bool {
        self.current_kind() == Some(kind)
    }

    fn is_blank_line(&self) -> bool {
        self.cursor
            .current()
            .is_some_and(|line| line.token_start == line.token_end)
    }

    fn skip_blank_lines(&mut self) {
        while !self.cursor.eof() && self.is_blank_line() {
            self.cursor.advance();
        }
    }

    fn source_content_end(&self) -> usize {
        self.tokens
            .iter()
            .rev()
            .find(|token| token.kind != TokenKind::Newline)
            .map_or(0, |token| token.source_range.end)
    }
}
