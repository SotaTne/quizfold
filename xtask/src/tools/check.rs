use std::process::Command;

pub struct ToolCheck<'a> {
    pub command: &'a str,
    pub args: &'a [&'a str],
    pub install_hint: &'a str,
    pub display_name: &'a str,
}

impl<'a> ToolCheck<'a> {
    pub fn verify(&self) -> bool {
        match Command::new(self.command).args(self.args).output() {
            Ok(output) if output.status.success() => {
                println!("{} {}", self.display_name, green("ok"));
                true
            }
            Ok(_) | Err(_) => {
                println!("{} {}", self.display_name, red("fail"));
                println!("  hint: {}", self.install_hint);
                false
            }
        }
    }
}

fn green(text: &str) -> String {
    format!("\u{1b}[32m{text}\u{1b}[0m")
}

fn red(text: &str) -> String {
    format!("\u{1b}[31m{text}\u{1b}[0m")
}
