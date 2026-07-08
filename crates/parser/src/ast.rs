// Canonical QuizFold abstract syntax tree.
// These types define the persisted parser shape and generated TypeScript surface.
use crate::source::SourceRange;

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct QuizFoldDocument {
    pub items: Vec<DocumentItem>,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct DocumentItem {
    #[serde(flatten)]
    pub kind: DocumentItemKind,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "kind", content = "value")]
pub enum DocumentItemKind {
    Quiz(QuizItem),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct QuizItem {
    #[serde(flatten)]
    pub kind: QuizItemKind,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "kind", content = "value")]
pub enum QuizItemKind {
    /// ? question
    /// ---
    /// answer
    Qa(QaQuiz),

    /// ! This is ${answer}
    Fold(FoldQuiz),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct QaQuiz {
    pub question: QuizContent,
    pub answer: QuizContent,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct FoldQuiz {
    pub content: QuizContent,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct QuizContent {
    pub blocks: Vec<Block>,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Block {
    #[serde(flatten)]
    pub kind: BlockKind,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "kind", content = "value")]
pub enum BlockKind {
    Paragraph(Paragraph),
    Memo(MemoBlock),
    MathBlock(MathBlock),
    CodeBlock(CodeBlock),
    MermaidBlock(MermaidBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct MemoBlock {
    pub blocks: Vec<Block>,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Paragraph {
    pub inlines: Vec<Inline>,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Inline {
    #[serde(flatten)]
    pub kind: InlineKind,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "kind", content = "value")]
pub enum InlineKind {
    Raw(Raw),
    MathInline(MathInline),
    FoldBlank(FoldBlank),
    Image(Image),
    SoftBreak,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct FoldBlank {
    pub answer: FoldBlankContent,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct FoldBlankContent {
    pub inlines: Vec<FoldBlankInline>,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct FoldBlankInline {
    #[serde(flatten)]
    pub kind: FoldBlankInlineKind,
    pub source_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "kind", content = "value")]
pub enum FoldBlankInlineKind {
    Raw(Raw),
    MathInline(MathInline),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Raw {
    pub value: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Image {
    pub alt: Raw,
    pub reference: ImageReference,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "kind", content = "value")]
pub enum ImageReference {
    RequestAttachment(AttachmentKey),
    StoredImage(StoredImageId),
    ExternalUrl(ExternalImageUrl),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct AttachmentKey(Box<str>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct StoredImageId(Box<str>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct ExternalImageUrl(Box<str>);

impl AttachmentKey {
    pub(crate) fn from_source(value: &str) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl StoredImageId {
    pub(crate) fn from_source(value: &str) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ExternalImageUrl {
    pub(crate) fn from_source(value: &str) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct MathBlock {
    pub source: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct MathInline {
    pub source: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct CodeBlock {
    #[cfg_attr(feature = "tsify", tsify(optional))]
    pub language: Option<Box<str>>,
    pub source: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
pub struct MermaidBlock {
    pub source: Box<str>,
}

impl QuizFoldDocument {
    pub fn new(items: Vec<DocumentItem>, source_range: SourceRange) -> Self {
        Self {
            items,
            source_range,
        }
    }
}

impl DocumentItem {
    pub fn new(kind: DocumentItemKind, source_range: SourceRange) -> Self {
        Self { kind, source_range }
    }
}

impl QuizItem {
    pub fn new(kind: QuizItemKind, source_range: SourceRange) -> Self {
        Self { kind, source_range }
    }
}

impl QaQuiz {
    pub fn new(question: QuizContent, answer: QuizContent, source_range: SourceRange) -> Self {
        Self {
            question,
            answer,
            source_range,
        }
    }
}

impl FoldQuiz {
    pub fn new(content: QuizContent, source_range: SourceRange) -> Self {
        Self {
            content,
            source_range,
        }
    }
}

impl QuizContent {
    pub fn new(blocks: Vec<Block>, source_range: SourceRange) -> Self {
        Self {
            blocks,
            source_range,
        }
    }
}

impl Block {
    pub fn new(kind: BlockKind, source_range: SourceRange) -> Self {
        Self { kind, source_range }
    }
}

impl MemoBlock {
    pub fn new(blocks: Vec<Block>, source_range: SourceRange) -> Self {
        Self {
            blocks,
            source_range,
        }
    }
}

impl Paragraph {
    pub fn new(inlines: Vec<Inline>, source_range: SourceRange) -> Self {
        Self {
            inlines,
            source_range,
        }
    }
}

impl Inline {
    pub fn new(kind: InlineKind, source_range: SourceRange) -> Self {
        Self { kind, source_range }
    }
}

impl FoldBlank {
    pub fn new(answer: FoldBlankContent, source_range: SourceRange) -> Self {
        Self {
            answer,
            source_range,
        }
    }
}

impl FoldBlankContent {
    pub fn new(inlines: Vec<FoldBlankInline>, source_range: SourceRange) -> Self {
        Self {
            inlines,
            source_range,
        }
    }
}

impl FoldBlankInline {
    pub fn new(kind: FoldBlankInlineKind, source_range: SourceRange) -> Self {
        Self { kind, source_range }
    }
}
