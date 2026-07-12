// Generates the ErrorCode TypeScript union from
// quizfold_parser::errors::ParseError::ALL_CODES, so src/types.rs's
// wasm-bindgen typescript_custom_section can never drift from the actual
// error codes defined in crates/parser/src/errors.rs.
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=../parser/src/errors.rs");

    let mut error_code_union = String::from("export type ErrorCode =\n");
    for code in quizfold_parser::errors::ParseError::ALL_CODES {
        error_code_union.push_str(&format!("  | \"{code}\"\n"));
    }
    error_code_union.push(';');

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo"));
    fs::write(out_dir.join("error_code_union.ts"), error_code_union)
        .expect("write generated ErrorCode union");
}
