// Source-independent QuizFold document model.
// It removes source ranges and replaces inline answers with stable local indices.
use crate::ast::{CodeBlock, Image, MathBlock, MermaidBlock};

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Document {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
#[serde(tag = "kind", content = "value")]
pub enum Item {
    Qa(Qa),
    QaFold(QaFold),
    Fold(Fold),
    Note(Note),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Qa {
    pub question: Content,
    pub answer: Content,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct QaFold {
    pub question: Content,
    pub content: Content,
    pub blanks: Vec<Blank>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Fold {
    pub content: Vec<Inline>,
    pub blanks: Vec<Blank>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Note {
    pub block: Block,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Content {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
#[serde(tag = "kind", content = "value")]
pub enum Block {
    Paragraph(Paragraph),
    Memo(Memo),
    MathBlock(#[cfg_attr(feature = "tsify", tsify(type = "MathBlock"))] MathBlock),
    CodeBlock(#[cfg_attr(feature = "tsify", tsify(type = "CodeBlock"))] CodeBlock),
    MermaidBlock(#[cfg_attr(feature = "tsify", tsify(type = "MermaidBlock"))] MermaidBlock),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Paragraph {
    pub inlines: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Memo {
    pub blocks: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
#[serde(tag = "kind", content = "value")]
pub enum Inline {
    Raw(Box<str>),
    MathInline(Box<str>),
    Image(#[cfg_attr(feature = "tsify", tsify(type = "Image"))] Image),
    SoftBreak,
    Blank(u32),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
pub struct Blank {
    pub answer: Vec<BlankInline>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(
    feature = "tsify",
    tsify(into_wasm_abi, from_wasm_abi, type_prefix = "Model")
)]
#[serde(tag = "kind", content = "value")]
pub enum BlankInline {
    Raw(Box<str>),
    MathInline(Box<str>),
}
