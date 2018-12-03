use std::path::PathBuf;

// LEARNING: modules declared in main.rs using "mod" are accessible from other modules
use commands::Command;
use config::Config;

/// Poor man's command line arguments parsing.
/// Done deliberately - to have fun with Rust matching.
/// There are libraries to do this, e.g. <https://crates.io/crates/getopts>
///
pub fn parse_args(args: &[String]) -> Command {
    // LEARNING: Rust is move by default.
    //           To get a hold of a vector (without copying), we need to *borrow* it

    let (base_name, args) = (args[0].clone(), &args[1..]);

    let mut config = Config::new();
    let mut expect_dictionary_path = false;

    for arg in args.iter() {
        match arg as &str {
            "-h" | "--help" => {
                // LEARNING: can shorten `base_name: base_name` here
                //           (because the local var name is the same as field name)
                return Command::PrintUsage { base_name };
            },

            "-d" | "--dictionary" => {
                expect_dictionary_path = true;
            },

            "-c" | "--case-sensitive" => {
                config.case_sensitive = true;
            },

            // LEARNING: can use "if" guard for extra logic
            _ if expect_dictionary_path => {
                config.dictionary_path = PathBuf::from(arg);
                expect_dictionary_path = false;
            },

            _ => {
                config.input_words.push(arg.to_string());
            },
        }
    }

    if expect_dictionary_path {
        return Command::ErrorExit {
            reason: "Option `-d' requires an argument".into(),
        }
    }

    if config.input_words.is_empty() {
        return Command::ErrorExit {
            reason: "No words specified".into(),
        }
    }

    Command::PrintAnagrams(config)
}


pub fn print_usage(base_name: &str) {
    println!("Usage: {} [-d DICTIONARY_FILE] [-c] WORD...", base_name);
}
