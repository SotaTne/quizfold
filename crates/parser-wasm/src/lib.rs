use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(typescript_custom_section)]
const TYPESCRIPT_TYPES: &'static str = r#"
export type Severity = "Fatal" | "Error" | "Warning";

export type ErrorCode =
  | "QF001"
  | "QF002"
  | "QF003"
  | "QF004"
  | "QF005"
  | "QF006"
  | "QF007"
  | "QF008"
  | "QF009"
  | "QF010";

export type ParseError =
  | "MissingAnswerSeparator"
  | "FoldQuizWithoutBlank"
  | "UnclosedFoldBlank"
  | "UnclosedMathInline"
  | "UnclosedBlock"
  | "EmptyImageAlt"
  | "InvalidImageReference"
  | "UnclosedMemo"
  | "UnexpectedMemoEnd"
  | "NestedMemo";

export interface SourceRange {
  start: number;
  end: number;
}

export interface Diagnostic {
  error: ParseError;
  severity: Severity;
  code: ErrorCode;
  message: string;
  source_range: SourceRange;
}

export interface ParseStats {
  byte_len: number;
}

export interface References {
  request_attachments: string[];
  stored_images: string[];
  external_images: string[];
}

export interface ParseResult {
  document: QuizFoldDocument;
  diagnostics: Diagnostic[];
  references: References;
  stats: ParseStats;
}

export interface QuizFoldDocument {
  items: DocumentItem[];
  source_range: SourceRange;
}

export interface DocumentItem {
  kind: DocumentItemKind;
  source_range: SourceRange;
}

export type DocumentItemKind =
  | { Quiz: QuizItem }
  | { Block: Block };

export interface QuizItem {
  kind: QuizItemKind;
  source_range: SourceRange;
}

export type QuizItemKind =
  | { Qa: QaQuiz }
  | { Fold: FoldQuiz };

export interface QaQuiz {
  question: QuizContent;
  answer: QuizContent;
  source_range: SourceRange;
}

export interface FoldQuiz {
  content: QuizContent;
  source_range: SourceRange;
}

export interface QuizContent {
  blocks: Block[];
  source_range: SourceRange;
}

export interface Block {
  kind: BlockKind;
  source_range: SourceRange;
}

export type BlockKind =
  | { Paragraph: Paragraph }
  | { Memo: MemoBlock }
  | { MathBlock: MathBlock }
  | { CodeBlock: CodeBlock }
  | { MermaidBlock: MermaidBlock };

export interface Paragraph {
  inlines: Inline[];
  source_range: SourceRange;
}

export interface MemoBlock {
  blocks: Block[];
  source_range: SourceRange;
}

export interface Inline {
  kind: InlineKind;
  source_range: SourceRange;
}

export type InlineKind =
  | { Raw: Raw }
  | { MathInline: MathInline }
  | { FoldBlank: FoldBlank }
  | { Image: Image }
  | "SoftBreak";

export interface FoldBlank {
  answer: FoldBlankContent;
  source_range: SourceRange;
}

export interface FoldBlankContent {
  inlines: FoldBlankInline[];
  source_range: SourceRange;
}

export interface FoldBlankInline {
  kind: FoldBlankInlineKind;
  source_range: SourceRange;
}

export type FoldBlankInlineKind =
  | { Raw: Raw }
  | { MathInline: MathInline };

export interface Raw {
  value: string;
}

export interface Image {
  alt: Raw;
  reference: ImageReference;
}

export type ImageReference =
  | { RequestAttachment: string }
  | { StoredImage: string }
  | { ExternalUrl: string };

export interface MathBlock {
  source: string;
}

export interface MathInline {
  source: string;
}

export interface CodeBlock {
  language?: string;
  source: string;
}

export interface MermaidBlock {
  source: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "ParseResult")]
    pub type JsParseResult;

    #[wasm_bindgen(typescript_type = "Diagnostic[]")]
    pub type JsDiagnostics;
}

#[wasm_bindgen(js_name = parseQuizFold)]
pub fn parse_quizfold(input: &str) -> Result<JsParseResult, JsValue> {
    serde_wasm_bindgen::to_value(&quizfold_parser::parse_quizfold(input))
        .map(JsValue::unchecked_into)
        .map_err(serialization_error)
}

#[wasm_bindgen(js_name = validateQuizFold)]
pub fn validate_quizfold(input: &str) -> Result<JsDiagnostics, JsValue> {
    serde_wasm_bindgen::to_value(&quizfold_parser::validate_quizfold(input))
        .map(JsValue::unchecked_into)
        .map_err(serialization_error)
}

fn serialization_error(error: serde_wasm_bindgen::Error) -> JsValue {
    JsValue::from_str(&error.to_string())
}
