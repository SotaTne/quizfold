// Source-independent document model shared by editing, storage, and rendering.
// Conversion to and from the parser AST lives beside the types it protects.
mod convert;
mod error;
mod types;

pub use error::{ModelDiagnostic, ModelError};
pub use types::*;
