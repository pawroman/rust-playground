/// pawroman's rust-playground
///
/// Anagrams program.
///
/// Asks for string on the commandline, then tries to find anagrams in system dictionary
/// (default: /usr/share/dict/words).
///

use args_parser::parse_args;
use commands::CommandResult;

// LEARNING: need to explicitly declare program modules
mod anagram_dict;
mod args_parser;
mod commands;
mod config;


// LEARNING: constants are inlined
pub const EXIT_CODE_ERROR: i32 = 1;     // std::process::exit takes i32


// main has no return value - must use std::process::exit() to indicate non-0 exit code
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let command = parse_args(&args);
    let result = command.execute();

    // LEARNING: `if let` is like a one-pattern match
    if let CommandResult::Error(reason) = result {
        println!("{}", reason);
        std::process::exit(EXIT_CODE_ERROR);
    }
}
