// Parser-wide literal prefixes.
// Keep protocol-like strings centralized so lexer/parser/printer stay aligned.
pub(crate) const REQUEST_ATTACHMENT_PREFIX: &str = "qf-attachment:";
pub(crate) const STORED_IMAGE_PREFIX: &str = "qf-stored:";
pub(crate) const HTTP_PREFIX: &str = "http://";
pub(crate) const HTTPS_PREFIX: &str = "https://";
