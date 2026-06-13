use crate::lexer::{Token, TokenKind};
use crate::source::SourceRange;

#[derive(Debug, Clone, Copy)]
pub(super) struct LexedLine {
    pub(super) token_start: usize,
    pub(super) token_end: usize,
    pub(super) content_range: SourceRange,
    pub(super) full_end: usize,
}

pub(super) struct LineCursor {
    lines: Vec<LexedLine>,
    position: usize,
}

impl LineCursor {
    pub(super) fn new(input: &str, tokens: &[Token]) -> Self {
        Self {
            lines: collect_lines(input, tokens),
            position: 0,
        }
    }

    pub(super) fn eof(&self) -> bool {
        self.position >= self.lines.len()
    }

    pub(super) fn position(&self) -> usize {
        self.position
    }

    pub(super) fn current(&self) -> Option<LexedLine> {
        self.get(self.position)
    }

    pub(super) fn get(&self, position: usize) -> Option<LexedLine> {
        self.lines.get(position).copied()
    }

    pub(super) fn advance(&mut self) {
        self.position += usize::from(!self.eof());
    }

    pub(super) fn previous_end(&self) -> Option<usize> {
        self.position
            .checked_sub(1)
            .and_then(|position| self.get(position))
            .map(|line| line.full_end)
    }
}

fn collect_lines(input: &str, tokens: &[Token]) -> Vec<LexedLine> {
    if input.is_empty() {
        return Vec::new();
    }

    let mut lines = Vec::new();
    let mut token_start = 0;
    let mut source_start = 0;

    for (index, token) in tokens.iter().enumerate() {
        if token.kind != TokenKind::Newline {
            continue;
        }
        lines.push(LexedLine {
            token_start,
            token_end: index,
            content_range: SourceRange::new(source_start, token.source_range.start),
            full_end: token.source_range.end,
        });
        token_start = index + 1;
        source_start = token.source_range.end;
    }

    if source_start < input.len() {
        lines.push(LexedLine {
            token_start,
            token_end: tokens.len(),
            content_range: SourceRange::new(source_start, input.len()),
            full_end: input.len(),
        });
    }

    lines
}
