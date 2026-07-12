// Block printer for QuizFold AST nodes.
// It serializes quizzes, memo blocks, math/code fences, and paragraphs.
use super::Printer;
use crate::ast::{
    Block, BlockKind, DocumentItem, DocumentItemKind, FoldQuiz, QaQuiz, QuizContent, QuizItemKind,
};

impl Printer {
    pub(super) fn print_document_item(&mut self, item: &DocumentItem) {
        match &item.kind {
            DocumentItemKind::Quiz(quiz) => match &quiz.kind {
                QuizItemKind::Qa(qa) => self.print_qa(qa),
                QuizItemKind::Fold(fold) => self.print_fold(fold),
            },
            DocumentItemKind::Block(block) => self.print_block(block),
        }
    }

    fn print_qa(&mut self, quiz: &QaQuiz) {
        self.writer.push("? ");
        let question_rest = self.print_leading_paragraph(&quiz.question);
        self.writer.newline();
        if question_rest
            .first()
            .is_some_and(|block| matches!(block.kind, BlockKind::Paragraph(_)))
        {
            self.writer.blank_line();
        }
        self.print_blocks(question_rest, true);
        self.writer.push("---");
        self.writer.newline();
        self.print_blocks(&quiz.answer.blocks, false);
    }

    fn print_fold(&mut self, quiz: &FoldQuiz) {
        self.writer.push("! ");
        if let Some(Block {
            kind: BlockKind::Paragraph(paragraph),
            ..
        }) = quiz.content.blocks.first()
        {
            self.print_paragraph(paragraph);
        }
        self.writer.newline();
    }

    fn print_leading_paragraph<'a>(&mut self, content: &'a QuizContent) -> &'a [Block] {
        let Some((first, rest)) = content.blocks.split_first() else {
            return &[];
        };
        if let BlockKind::Paragraph(paragraph) = &first.kind {
            self.print_paragraph(paragraph);
            rest
        } else {
            &content.blocks
        }
    }

    pub(super) fn print_block(&mut self, block: &Block) {
        match &block.kind {
            BlockKind::Paragraph(paragraph) => {
                self.print_paragraph(paragraph);
                self.writer.newline();
            }
            BlockKind::Memo(memo) => {
                self.writer.push("@memo");
                self.writer.newline();
                self.print_blocks(&memo.blocks, true);
                self.writer.push("@end");
                self.writer.newline();
            }
            BlockKind::MathBlock(math) => {
                self.writer.push("$$");
                self.writer.newline();
                self.writer.push(&math.source);
                self.writer.newline();
                self.writer.push("$$");
                self.writer.newline();
            }
            BlockKind::CodeBlock(code) => {
                self.writer.push("```");
                if let Some(language) = &code.language {
                    self.writer.push(language);
                }
                self.writer.newline();
                self.writer.push(&code.source);
                self.writer.newline();
                self.writer.push("```");
                self.writer.newline();
            }
            BlockKind::MermaidBlock(mermaid) => {
                self.writer.push("```mermaid");
                self.writer.newline();
                self.writer.push(&mermaid.source);
                self.writer.newline();
                self.writer.push("```");
                self.writer.newline();
            }
        }
    }

    fn print_blocks(&mut self, blocks: &[Block], separated: bool) {
        for (index, block) in blocks.iter().enumerate() {
            if separated && index > 0 {
                self.writer.blank_line();
            }
            self.print_block(block);
        }
    }
}
