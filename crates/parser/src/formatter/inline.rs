// Inline formatter for QuizFold AST nodes.
// It serializes inline text, math, fold blanks, and image references.
use super::Formatter;
use crate::ast::{
    FoldBlankContent, FoldBlankInlineKind, Image, ImageReference, InlineKind, Paragraph,
};
use crate::constants::{REQUEST_ATTACHMENT_PREFIX, STORED_IMAGE_PREFIX};

impl Formatter {
    pub(super) fn format_paragraph(&mut self, paragraph: &Paragraph) {
        for inline in &paragraph.inlines {
            self.format_inline(&inline.kind);
        }
    }

    fn format_inline(&mut self, inline: &InlineKind) {
        match inline {
            InlineKind::Raw(raw) => self.writer.push(&raw.value),
            InlineKind::MathInline(math) => {
                self.writer.push("$");
                self.writer.push(&math.source);
                self.writer.push("$");
            }
            InlineKind::FoldBlank(blank) => self.format_fold_blank(&blank.answer),
            InlineKind::Image(image) => self.format_image(image),
            InlineKind::SoftBreak => self.writer.newline(),
        }
    }

    fn format_fold_blank(&mut self, content: &FoldBlankContent) {
        self.writer.push("${");
        for inline in &content.inlines {
            match &inline.kind {
                FoldBlankInlineKind::Raw(raw) => self.writer.push(&raw.value),
                FoldBlankInlineKind::MathInline(math) => {
                    self.writer.push("$");
                    self.writer.push(&math.source);
                    self.writer.push("$");
                }
            }
        }
        self.writer.push("}");
    }

    fn format_image(&mut self, image: &Image) {
        self.writer.push("![");
        self.writer.push(&image.alt.value);
        self.writer.push("](");
        match &image.reference {
            ImageReference::RequestAttachment(key) => {
                self.writer.push(REQUEST_ATTACHMENT_PREFIX);
                self.writer.push(key.as_str());
            }
            ImageReference::StoredImage(id) => {
                self.writer.push(STORED_IMAGE_PREFIX);
                self.writer.push(id.as_str());
            }
            ImageReference::ExternalUrl(url) => self.writer.push(url.as_str()),
        }
        self.writer.push(")");
    }
}
