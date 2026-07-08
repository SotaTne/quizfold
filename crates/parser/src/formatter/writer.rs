// Small output writer used by the formatter.
// It centralizes newline handling for canonical markdown generation.
#[derive(Default)]
pub(super) struct Writer {
    output: String,
}

impl Writer {
    pub(super) fn push(&mut self, value: &str) {
        self.output.push_str(value);
    }

    pub(super) fn newline(&mut self) {
        if !self.output.ends_with('\n') {
            self.output.push('\n');
        }
    }

    pub(super) fn blank_line(&mut self) {
        self.newline();
        if !self.output.ends_with("\n\n") {
            self.output.push('\n');
        }
    }

    pub(super) fn finish(mut self) -> String {
        while self.output.ends_with('\n') {
            self.output.pop();
        }
        if !self.output.is_empty() {
            self.output.push('\n');
        }
        self.output
    }
}
