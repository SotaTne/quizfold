// Generates ParseErrorCode, ModelErrorCode, and their shared ErrorCode union.
// The TypeScript public surface therefore cannot drift from Rust definitions.
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=../parser/src/errors.rs");
    println!("cargo:rerun-if-changed=../parser/src/model/error.rs");

    let mut declarations = type_union(
        "ParseErrorCode",
        quizfold_parser::errors::ParseError::ALL_CODES,
    );
    declarations.push_str("\n\n");
    declarations.push_str(&type_union(
        "ModelErrorCode",
        quizfold_parser::model::ModelError::ALL_CODES,
    ));
    declarations.push_str("\n\nexport type ErrorCode = ParseErrorCode | ModelErrorCode;");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo"));
    fs::write(out_dir.join("error_code_union.ts"), declarations)
        .expect("write generated ErrorCode union");
}

fn type_union(name: &str, codes: &[&str]) -> String {
    let mut declaration = format!("export type {name} =\n");
    for code in codes {
        declaration.push_str(&format!("  | \"{code}\"\n"));
    }
    declaration.push(';');
    declaration
}
