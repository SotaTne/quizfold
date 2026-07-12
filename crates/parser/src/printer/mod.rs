// Markdown printer entry point for QuizFold AST.
// It converts canonical AST back into stable QuizFold source text.
mod block;
mod inline;
mod writer;

use self::writer::Writer;
use crate::ast::QuizFoldDocument;

pub fn print_quizfold(document: &QuizFoldDocument) -> String {
    Printer::new().print(document)
}

struct Printer {
    writer: Writer,
}

impl Printer {
    fn new() -> Self {
        Self {
            writer: Writer::default(),
        }
    }

    fn print(mut self, document: &QuizFoldDocument) -> String {
        for (index, item) in document.items.iter().enumerate() {
            if index > 0 {
                self.writer.blank_line();
            }
            self.print_document_item(item);
        }
        self.writer.finish()
    }
}
