/// pawroman's rust-playground
///
/// Anagrams program.
///
/// Asks for string on the commandline, then tries to find anagrams in system dictionary
/// (default: /usr/share/dict/words).
///

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// static string (str) (lives for the whole lifetime of a program)
static DEFAULT_WORDS_LIST_PATH: &'static str = "/usr/share/dict/words";

// constants are inlined
const EXIT_CODE_ERROR: i32 = 1;

// Input words list
type InputWords = Vec<String>;

// Program config
struct Config {
    words_list_path : String,       // Filesystem path to the dictionary file
    input_words     : InputWords,   // Input words (words to finds anagrams for)
    case_sensitive  : bool,         // Be case sensitive (usually makes little sense)
}

// Possible commands to be executed
enum Command {
    Exit { reason: String, error: bool },
    PrintAnagrams(Config),
}

// Word dictionary - character-wise sorted words mapped to matches in the word file
// E.g. "abc" -> {"abc", "cba"}
type ValueSet = HashSet<String>;
type Dictionary = HashMap<String, ValueSet>;


// main has no return value - must use std::process::exit() to indicate non-0 exit code
fn main() {
    let args_vec = std::env::args().collect();

    match parse_args(&args_vec) {
        Command::Exit { reason, error } => {
            if error {
                print!("ERROR: ");
            }
            println!("{}", reason);

            if error {
                std::process::exit(EXIT_CODE_ERROR);
            }
        }

        Command::PrintAnagrams(config) => {
            match load_dictionary(&config.words_list_path, config.case_sensitive) {
                Ok(dict) => {
                    for word in config.input_words.iter() {
                        if let Some(anagrams) = find_anagrams(&word, &dict, config.case_sensitive) {
                            print_anagrams(word, anagrams);
                        }
                    }
                }
                Err(e) => {
                    println!("ERROR: couldn't load dictionary file: {}", e.to_string());
                    std::process::exit(EXIT_CODE_ERROR);
                }
            }
        }
    }
}

/// Poor man's command line arguments parsing.
/// Done deliberately - to have fun with Rust matching.
/// There are libraries to do this, e.g. https://crates.io/crates/getopts
///
fn parse_args(args_vec: &Vec<String>) -> Command {
    let (base_name, args) = (&args_vec[0], &args_vec[1..]);

    let mut config = Config {
        words_list_path: DEFAULT_WORDS_LIST_PATH.into(),
        input_words: Vec::new(),    // woah, the compiler will infer this based on first usage
        case_sensitive: false,
    };

    let mut expect_words_list_path = false;

    for arg in args.iter() {
        match arg as &str {
            "-h" | "--help" => {
                return Command::Exit {
                    reason: format!("Usage: {} [-w WORDS_LIST] [-c] WORD...", base_name),
                    error: false
                };
            },

            "-w" | "--words" | "--word-list" => {
                expect_words_list_path = true;
            },

            "-c" | "--case-sensitive" => {
                config.case_sensitive = true;
            },

            _ if expect_words_list_path => {
                config.words_list_path = arg.to_string();
                expect_words_list_path = false;
            },

            _ => {
                config.input_words.push(arg.to_string());
            }
        }
    }

    if expect_words_list_path {
        return Command::Exit {
            reason: "Option `-w' requires an argument".to_owned(),
            error: true,
        }
    }

    if config.input_words.is_empty() {
        return Command::Exit {
            reason: "No words specified".to_owned(),
            error: true,
        }
    }

    Command::PrintAnagrams(config)
}


/// Load the anagram dictionary from the specified path
///
fn load_dictionary(words_list_path: &str, case_sensitive: bool) -> Result<Dictionary, io::Error> {
    // Handle missing file case nicely
    if !Path::new(words_list_path).is_file() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound, format!("Can't find file: `{}'", words_list_path)
        ));
    }

    // ? returns the Ok Result, or causes the function to return the Error early
    // (equiv. to try!(..))
    let file = File::open(words_list_path)?;
    let reader = io::BufReader::new(file);
    let mut dict = Dictionary::new();

    // assume the word list is one word per line
    // Result.ok() -> Option (None if error)
    // filter_map -> skips None options
    for line in reader.lines().filter_map(|l| l.ok()) {
        if let Some(key) = anagram_key(&line, case_sensitive) {
            let mut value = dict.entry(key).or_insert_with(|| ValueSet::new());

            value.insert(line);
        }
    }

    Ok(dict)
}


fn anagram_key(line: &str, case_sensitive: bool) -> Option<String> {
    use std::iter::FromIterator;

    if line.ends_with("'s") {
        None
    }
    else {
        let base = if case_sensitive {
            line.to_string()
        } else {
            line.to_lowercase()
        };

        let mut chars_vec = base.chars().collect::<Vec<_>>();
        chars_vec.sort();

        Some(String::from_iter(chars_vec.into_iter()))
    }
}


/// Find anagrams.
///
fn find_anagrams(word: &str, dictionary: &Dictionary, case_sensitive: bool) -> Option<Vec<String>> {
    if let Some(key) = anagram_key(&word, case_sensitive) {
        if let Some(values) = dictionary.get(&key) {
            // cloned -> get values - String not &String
            let mut anagrams: Vec<_> = values
                .iter()
                .filter(|val| val.to_lowercase() != word.to_lowercase())
                .cloned()
                .collect();

            if anagrams.is_empty() {
                return None;
            }

            anagrams.sort();

            return Some(anagrams);
        }
    }

    None
}

fn print_anagrams(word: &str, anagrams: Vec<String>) {
    println!("{}: {:?}", word, anagrams)
}
