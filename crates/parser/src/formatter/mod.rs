mod block;
mod inline;
mod writer;

use self::writer::Writer;
use crate::ast::QuizFoldDocument;

pub fn format_quizfold(document: &QuizFoldDocument) -> String {
    Formatter::new().format(document)
}

struct Formatter {
    writer: Writer,
}

impl Formatter {
    fn new() -> Self {
        Self {
            writer: Writer::default(),
        }
    }

    fn format(mut self, document: &QuizFoldDocument) -> String {
        for (index, item) in document.items.iter().enumerate() {
            if index > 0 {
                self.writer.blank_line();
            }
            self.format_document_item(item);
        }
        self.writer.finish()
    }
}
