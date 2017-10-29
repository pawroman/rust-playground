/// Possible commands to be executed

use anagram_dict::{AnagramDictionary, AnagramDictTrait};
use args_parser::print_usage;
use config::Config;


pub enum Command {
    // LEARNING: named vs unnamed member fields
    PrintUsage { base_name: String },
    PrintAnagrams(Config),
    ErrorExit { reason: String },
}


pub enum CommandResult {
    Success,
    Error(String),
}


impl Command {
    pub fn execute(&self) -> CommandResult {
        // bring enum members into this scope
        use self::Command::*;

        match *self {
            // LEARNING: because self is borrowed, need to access the members via a reference (ref)
            PrintUsage { ref base_name } => {
                print_usage(&base_name);
            },

            PrintAnagrams(ref config) => {
                match AnagramDictionary::load_from_path(&config.dictionary_path, config.case_sensitive) {
                    Ok(dict) => {
                        lookup_and_print_anagrams(&dict, &config);
                    },
                    Err(e) => {
                        return CommandResult::Error(
                            format!("Couldn't load dictionary file: {}", e.to_string())
                        );
                    },
                }
            },

            // LEARNING: de-structuring enum with named members
            ErrorExit { ref reason } => {
                // LEARNING: moving out reason out of self is not possible here.
                //           Need to clone (make a copy of) the String.
                return CommandResult::Error(reason.clone());
            },
        }

        CommandResult::Success
    }
}


fn lookup_and_print_anagrams(dict: &AnagramDictionary, config: &Config) {
    for word in config.input_words.iter() {
        if let Some(anagrams) = dict.lookup(&word, config.case_sensitive) {
            print_anagrams(word, anagrams);
        }
    }
}


fn print_anagrams(word: &str, anagrams: Vec<&str>) {
    let formatted_anagrams = anagrams.join(", ");

    println!("{}: {}", word, formatted_anagrams)
}
