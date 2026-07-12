// Shared error-definition macro and parser error taxonomy.
// Add new variants carefully because these codes are part of the external API.
//
// Variants/codes/messages are declared once via `define_errors!`. The same
// macro is reused by model errors, and each enum exposes ALL_CODES so the WASM
// build can generate TypeScript unions without a hand-maintained second list.
use crate::diagnostics::Severity;

macro_rules! define_errors {
    (
        $(#[$meta:meta])*
        $visibility:vis enum $name:ident {
            $($variant:ident => $code:literal, $message:expr;)+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
        #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
        $visibility enum $name {
            $($variant),+
        }

        impl $name {
            pub const fn code(self) -> &'static str {
                match self {
                    $(Self::$variant => $code),+
                }
            }

            pub const fn message(self) -> &'static str {
                match self {
                    $(Self::$variant => $message),+
                }
            }

            /// Every error code, in declaration order.
            pub const ALL_CODES: &'static [&'static str] = &[$($code),+];

            pub const fn severity(self) -> Severity {
                Severity::Error
            }
        }
    };
}

pub(crate) use define_errors;

define_errors! {
pub enum ParseError {
    MissingAnswerSeparator => "QF001", "Q/A block requires an answer separator (`---`).";
    FoldQuizWithoutBlank => "QF002", "Fold quiz requires at least one `${...}` answer.";
    UnclosedFoldBlank => "QF003", "Fold answer is not closed with `}`.";
    UnclosedMathInline => "QF004", "Inline math is not closed with `$`.";
    UnclosedBlock => "QF005", "Block is not closed.";
    EmptyImageAlt => "QF006", "Image alt text must not be empty.";
    InvalidImageReference => "QF007", "Image reference must use `qf-attachment:`, `qf-stored:`, `http://`, or `https://`.";
    UnclosedMemo => "QF008", "Memo block is not closed with `@end`.";
    UnexpectedMemoEnd => "QF009", "`@end` does not have a matching `@memo`.";
    NestedMemo => "QF010", "Memo blocks cannot be nested.";
    QaSectionIsMemoOnly => "QF011", "Q/A question or answer must contain content other than a memo block.";
    FoldBlankOutsideAnswer => "QF012", "`${...}` can only appear in a fold line or a Q/A answer.";
}
}
