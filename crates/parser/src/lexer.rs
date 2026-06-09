use crate::source::SourceRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Raw,
    Newline,
    QuestionMarker,
    FoldMarker,
    AnswerSeparator,
    MathBlockDelimiter,
    CodeFenceStart {
        info: Option<SourceRange>,
    },
    CodeFenceEnd,
    FoldBlankStart,
    FoldBlankEnd,
    MathInlineDelimiter,
    Image {
        alt: SourceRange,
        destination: SourceRange,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlockMode {
    Code,
    Math,
}

pub fn lex(input: &str) -> Vec<Token> {
    lex_with_offset(input, 0)
}

pub(crate) fn lex_with_offset(input: &str, offset: usize) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut cursor = 0;
    let mut block_mode = None;

    while cursor < input.len() {
        let line_end = input[cursor..]
            .find('\n')
            .map_or(input.len(), |relative| cursor + relative);
        let content_end =
            line_end - usize::from(line_end > cursor && input.as_bytes()[line_end - 1] == b'\r');
        let line = &input[cursor..content_end];

        match block_mode {
            Some(BlockMode::Code) if line == "```" => {
                push(
                    &mut tokens,
                    TokenKind::CodeFenceEnd,
                    cursor,
                    content_end,
                    offset,
                );
                block_mode = None;
            }
            Some(BlockMode::Math) if line == "$$" => {
                push(
                    &mut tokens,
                    TokenKind::MathBlockDelimiter,
                    cursor,
                    content_end,
                    offset,
                );
                block_mode = None;
            }
            Some(_) => push(&mut tokens, TokenKind::Raw, cursor, content_end, offset),
            None if line == "$$" => {
                push(
                    &mut tokens,
                    TokenKind::MathBlockDelimiter,
                    cursor,
                    content_end,
                    offset,
                );
                block_mode = Some(BlockMode::Math);
            }
            None if line.starts_with("```") => {
                let info_start = cursor + 3;
                let trimmed_start = info_start
                    + input[info_start..content_end]
                        .len()
                        .saturating_sub(input[info_start..content_end].trim_start().len());
                let info = (trimmed_start < content_end)
                    .then(|| SourceRange::new(offset + trimmed_start, offset + content_end));
                push(
                    &mut tokens,
                    TokenKind::CodeFenceStart { info },
                    cursor,
                    content_end,
                    offset,
                );
                block_mode = Some(BlockMode::Code);
            }
            None if line == "---" => push(
                &mut tokens,
                TokenKind::AnswerSeparator,
                cursor,
                content_end,
                offset,
            ),
            None => lex_normal_line(input, cursor, content_end, offset, &mut tokens),
        }

        if line_end < input.len() {
            push(
                &mut tokens,
                TokenKind::Newline,
                line_end,
                line_end + 1,
                offset,
            );
            cursor = line_end + 1;
        } else {
            cursor = line_end;
        }
    }

    tokens
}

fn lex_normal_line(
    input: &str,
    line_start: usize,
    line_end: usize,
    offset: usize,
    tokens: &mut Vec<Token>,
) {
    let mut cursor = line_start;
    if input[cursor..line_end].starts_with("? ") {
        push(
            tokens,
            TokenKind::QuestionMarker,
            cursor,
            cursor + 2,
            offset,
        );
        cursor += 2;
    } else if input[cursor..line_end].starts_with("! ") {
        push(tokens, TokenKind::FoldMarker, cursor, cursor + 2, offset);
        cursor += 2;
    }

    lex_inline_range(input, cursor, line_end, offset, tokens);
}

fn lex_inline_range(input: &str, start: usize, end: usize, offset: usize, tokens: &mut Vec<Token>) {
    let mut cursor = start;
    let mut raw_start = start;
    let mut in_fold_blank = false;
    let mut in_math = false;

    while cursor < end {
        if input[cursor..end].starts_with("![") && !in_math && !in_fold_blank {
            if let Some((image_end, alt, destination)) = image_at(input, cursor, end, offset) {
                flush_raw(tokens, raw_start, cursor, offset);
                push(
                    tokens,
                    TokenKind::Image { alt, destination },
                    cursor,
                    image_end,
                    offset,
                );
                cursor = image_end;
                raw_start = cursor;
                continue;
            }
        }

        if input[cursor..end].starts_with("${") && !in_math {
            flush_raw(tokens, raw_start, cursor, offset);
            push(
                tokens,
                TokenKind::FoldBlankStart,
                cursor,
                cursor + 2,
                offset,
            );
            cursor += 2;
            raw_start = cursor;
            in_fold_blank = true;
            continue;
        }

        let byte = input.as_bytes()[cursor];
        if byte == b'}' && in_fold_blank && !in_math {
            flush_raw(tokens, raw_start, cursor, offset);
            push(tokens, TokenKind::FoldBlankEnd, cursor, cursor + 1, offset);
            cursor += 1;
            raw_start = cursor;
            in_fold_blank = false;
            continue;
        }

        if byte == b'$' && !is_escaped(input, cursor, start) {
            let valid = if in_math {
                previous_char(input, cursor).is_some_and(|character| !character.is_whitespace())
            } else {
                next_char(input, cursor + 1, end)
                    .is_some_and(|character| !character.is_whitespace() && character != '$')
            };
            if valid {
                flush_raw(tokens, raw_start, cursor, offset);
                push(
                    tokens,
                    TokenKind::MathInlineDelimiter,
                    cursor,
                    cursor + 1,
                    offset,
                );
                cursor += 1;
                raw_start = cursor;
                in_math = !in_math;
                continue;
            }
        }

        cursor += input[cursor..end].chars().next().unwrap().len_utf8();
    }

    flush_raw(tokens, raw_start, end, offset);
}

fn flush_raw(tokens: &mut Vec<Token>, start: usize, end: usize, offset: usize) {
    if start < end {
        push(tokens, TokenKind::Raw, start, end, offset);
    }
}

fn push(tokens: &mut Vec<Token>, kind: TokenKind, start: usize, end: usize, offset: usize) {
    tokens.push(Token {
        kind,
        source_range: SourceRange::new(offset + start, offset + end),
    });
}

fn is_escaped(input: &str, index: usize, lower_bound: usize) -> bool {
    index > lower_bound && input.as_bytes()[index - 1] == b'\\'
}

fn previous_char(input: &str, index: usize) -> Option<char> {
    input[..index].chars().next_back()
}

fn next_char(input: &str, index: usize, end: usize) -> Option<char> {
    input[index..end].chars().next()
}

fn image_at(
    input: &str,
    start: usize,
    end: usize,
    offset: usize,
) -> Option<(usize, SourceRange, SourceRange)> {
    let alt_start = start + 2;
    let alt_end = alt_start + input[alt_start..end].find(']')?;
    let destination_open = alt_end + 1;
    if input.as_bytes().get(destination_open) != Some(&b'(') {
        return None;
    }

    let destination_start = destination_open + 1;
    let destination_end = destination_start + input[destination_start..end].find(')')?;
    let image_end = destination_end + 1;

    Some((
        image_end,
        SourceRange::new(offset + alt_start, offset + alt_end),
        SourceRange::new(offset + destination_start, offset + destination_end),
    ))
}
