// Inline-level QuizFold grammar.
// This file parses raw text, inline math, fold blanks, and image references.
use super::{ContentContext, Parser};
use crate::ast::{
    AttachmentKey, Block, BlockKind, ExternalImageUrl, FoldBlank, FoldBlankContent,
    FoldBlankInline, FoldBlankInlineKind, Image, ImageReference, Inline, InlineKind, MathInline,
    Paragraph, QuizContent, Raw, StoredImageId,
};
use crate::constants::{HTTPS_PREFIX, HTTP_PREFIX, REQUEST_ATTACHMENT_PREFIX, STORED_IMAGE_PREFIX};
use crate::diagnostics::Diagnostic;
use crate::errors::ParseError;
use crate::lexer::{Token, TokenKind};
use crate::source::SourceRange;

impl Parser<'_> {
    pub(super) fn parse_content_tokens(
        &mut self,
        token_start: usize,
        token_end: usize,
        range: SourceRange,
    ) -> QuizContent {
        let paragraph = Paragraph::new(
            self.parse_inlines_range(token_start, token_end, ContentContext::Fold),
            range,
        );
        QuizContent::new(
            vec![Block::new(BlockKind::Paragraph(paragraph), range)],
            range,
        )
    }

    pub(super) fn parse_inlines_range(
        &mut self,
        start: usize,
        end: usize,
        context: ContentContext,
    ) -> Vec<Inline> {
        let tokens = self.tokens[start..end].to_vec();
        self.parse_inlines(&tokens, context)
    }

    /// Inline <- Raw / SoftBreak / FoldBlank / MathInline / Image
    /// Inline* -> Vec<Inline>
    fn parse_inlines(&mut self, tokens: &[Token], context: ContentContext) -> Vec<Inline> {
        let mut inlines = Vec::new();
        let mut index = 0;

        while let Some(token) = tokens.get(index).copied() {
            let (inline, next) = self.parse_inline(tokens, index, token, context);
            inlines.extend(inline);
            index = next;
        }

        inlines
    }

    fn parse_inline(
        &mut self,
        tokens: &[Token],
        index: usize,
        token: Token,
        context: ContentContext,
    ) -> (Vec<Inline>, usize) {
        match token.kind {
            TokenKind::Newline => (
                vec![Inline::new(InlineKind::SoftBreak, token.source_range)],
                index + 1,
            ),
            TokenKind::FoldBlankStart if context == ContentContext::Memo => {
                (vec![self.raw_inline(token)], index + 1)
            }
            TokenKind::FoldBlankStart => {
                let (inlines, next) = self.parse_fold_blank(tokens, index, token);
                if !context.allows_fold_blank()
                    && inlines
                        .iter()
                        .any(|inline| matches!(inline.kind, InlineKind::FoldBlank(_)))
                {
                    let range = inlines
                        .first()
                        .map_or(token.source_range, |inline| inline.source_range);
                    self.diagnostics
                        .push(Diagnostic::new(ParseError::FoldBlankOutsideAnswer, range));
                }
                (inlines, next)
            }
            TokenKind::MathInlineDelimiter => self.parse_math_inline(tokens, index, token),
            TokenKind::Image { alt, destination } => {
                (vec![self.parse_image(token, alt, destination)], index + 1)
            }
            _ => (vec![self.raw_inline(token)], index + 1),
        }
    }

    fn parse_fold_blank(
        &mut self,
        tokens: &[Token],
        index: usize,
        opening: Token,
    ) -> (Vec<Inline>, usize) {
        let (closing, end_index) = match self.close_inline(
            tokens,
            index,
            opening,
            TokenKind::FoldBlankEnd,
            ParseError::UnclosedFoldBlank,
        ) {
            Ok(closing) => closing,
            Err(recovered) => return recovered,
        };
        let answer_range = SourceRange::new(opening.source_range.end, closing.source_range.start);
        let answer = FoldBlankContent::new(
            fold_blank_inlines(self.input, &tokens[index + 1..end_index]),
            answer_range,
        );
        let range = SourceRange::new(opening.source_range.start, closing.source_range.end);
        (
            vec![Inline::new(
                InlineKind::FoldBlank(FoldBlank::new(answer, range)),
                range,
            )],
            end_index + 1,
        )
    }

    fn parse_math_inline(
        &mut self,
        tokens: &[Token],
        index: usize,
        opening: Token,
    ) -> (Vec<Inline>, usize) {
        let (closing, end_index) = match self.close_inline(
            tokens,
            index,
            opening,
            TokenKind::MathInlineDelimiter,
            ParseError::UnclosedMathInline,
        ) {
            Ok(closing) => closing,
            Err(recovered) => return recovered,
        };
        let range = SourceRange::new(opening.source_range.start, closing.source_range.end);
        let source = self.input[opening.source_range.end..closing.source_range.start].into();
        (
            vec![Inline::new(
                InlineKind::MathInline(MathInline { source }),
                range,
            )],
            end_index + 1,
        )
    }

    fn close_inline(
        &mut self,
        tokens: &[Token],
        index: usize,
        opening: Token,
        closing_kind: TokenKind,
        error: ParseError,
    ) -> Result<(Token, usize), (Vec<Inline>, usize)> {
        let Some(end_index) = find_token(tokens, index + 1, closing_kind) else {
            return Err(self.recover_unclosed_inline(tokens, index, opening, error));
        };
        Ok((tokens[end_index], end_index))
    }

    fn recover_unclosed_inline(
        &mut self,
        tokens: &[Token],
        index: usize,
        opening: Token,
        error: ParseError,
    ) -> (Vec<Inline>, usize) {
        let end = tokens
            .last()
            .map_or(opening.source_range.end, |token| token.source_range.end);
        self.diagnostics.push(Diagnostic::new(
            error,
            SourceRange::new(opening.source_range.start, end),
        ));
        (
            tokens[index..]
                .iter()
                .map(|token| self.raw_inline(*token))
                .collect(),
            tokens.len(),
        )
    }

    /// Image <- IMAGE
    /// Image -> InlineKind::Image
    fn parse_image(&mut self, token: Token, alt: SourceRange, destination: SourceRange) -> Inline {
        self.validate_image_alt(alt);
        let alt_text = &self.input[alt.start..alt.end];
        let destination_text = &self.input[destination.start..destination.end];
        let Some(reference) = self.parse_image_reference(destination_text) else {
            self.diagnostics.push(Diagnostic::new(
                ParseError::InvalidImageReference,
                destination,
            ));
            return self.raw_inline(token);
        };

        image_inline(alt_text, reference, token.source_range)
    }

    fn validate_image_alt(&mut self, range: SourceRange) {
        if range.start == range.end {
            self.diagnostics
                .push(Diagnostic::new(ParseError::EmptyImageAlt, range));
        }
    }

    fn parse_image_reference(&mut self, value: &str) -> Option<ImageReference> {
        if let Some(key) = value.strip_prefix(REQUEST_ATTACHMENT_PREFIX) {
            return self.request_attachment(key);
        }
        if let Some(id) = value.strip_prefix(STORED_IMAGE_PREFIX) {
            return self.stored_image(id);
        }
        self.external_image(value)
    }

    fn request_attachment(&mut self, value: &str) -> Option<ImageReference> {
        if value.is_empty() {
            return None;
        }
        let key = AttachmentKey::from_source(value);
        self.references.request_attachments.push(key.clone());
        Some(ImageReference::RequestAttachment(key))
    }

    fn stored_image(&mut self, value: &str) -> Option<ImageReference> {
        if value.is_empty() {
            return None;
        }
        let id = StoredImageId::from_source(value);
        self.references.stored_images.push(id.clone());
        Some(ImageReference::StoredImage(id))
    }

    fn external_image(&mut self, value: &str) -> Option<ImageReference> {
        if !is_http_image_url(value) {
            return None;
        }
        let url = ExternalImageUrl::from_source(value);
        self.references.external_images.push(url.clone());
        Some(ImageReference::ExternalUrl(url))
    }

    fn raw_inline(&self, token: Token) -> Inline {
        Inline::new(
            InlineKind::Raw(Raw {
                value: self.input[token.source_range.start..token.source_range.end].into(),
            }),
            token.source_range,
        )
    }
}

/// FoldBlankContent <- (Raw / MathInline)*
/// FoldBlankContent -> Vec<FoldBlankInline>
fn fold_blank_inlines(input: &str, tokens: &[Token]) -> Vec<FoldBlankInline> {
    let mut inlines = Vec::new();
    let mut index = 0;

    while let Some(token) = tokens.get(index).copied() {
        if let Some((inline, next)) = fold_blank_math(input, tokens, index, token) {
            inlines.push(inline);
            index = next;
        } else {
            inlines.push(fold_blank_raw(input, token));
            index += 1;
        }
    }

    inlines
}

fn fold_blank_math(
    input: &str,
    tokens: &[Token],
    index: usize,
    opening: Token,
) -> Option<(FoldBlankInline, usize)> {
    if opening.kind != TokenKind::MathInlineDelimiter {
        return None;
    }
    let end_index = find_token(tokens, index + 1, TokenKind::MathInlineDelimiter)?;
    let closing = tokens[end_index];
    let range = SourceRange::new(opening.source_range.start, closing.source_range.end);
    let source = input[opening.source_range.end..closing.source_range.start].into();
    Some((
        FoldBlankInline::new(
            FoldBlankInlineKind::MathInline(MathInline { source }),
            range,
        ),
        end_index + 1,
    ))
}

fn fold_blank_raw(input: &str, token: Token) -> FoldBlankInline {
    FoldBlankInline::new(
        FoldBlankInlineKind::Raw(Raw {
            value: input[token.source_range.start..token.source_range.end].into(),
        }),
        token.source_range,
    )
}

fn image_inline(alt: &str, reference: ImageReference, range: SourceRange) -> Inline {
    Inline::new(
        InlineKind::Image(Image {
            alt: Raw { value: alt.into() },
            reference,
        }),
        range,
    )
}

fn find_token(tokens: &[Token], start: usize, kind: TokenKind) -> Option<usize> {
    tokens[start..]
        .iter()
        .position(|token| token.kind == kind)
        .map(|relative| start + relative)
}

fn is_http_image_url(value: &str) -> bool {
    let remainder = value
        .strip_prefix(HTTPS_PREFIX)
        .or_else(|| value.strip_prefix(HTTP_PREFIX));

    remainder.is_some_and(|remainder| {
        !remainder.is_empty()
            && !remainder.starts_with('/')
            && !remainder.chars().any(char::is_whitespace)
    })
}
