use std::path::PathBuf;


// Input words list
pub type InputWords = Vec<String>;


// Program config
pub struct Config {
    pub case_sensitive  : bool,         // Be case sensitive in dictionary lookups (usually makes
                                        // little sense)
    pub dictionary_path : PathBuf,      // Filesystem path to the dictionary file
    pub input_words     : InputWords,   // Input words (words to finds anagrams for)
}

// LEARNING: static string (str) (lives for the whole lifetime of a program)
static DEFAULT_WORDS_LIST_PATH: &'static str = "/usr/share/dict/words";


impl Config {
    pub fn new() -> Config {
        Config {
            case_sensitive: false,
            dictionary_path: PathBuf::from(DEFAULT_WORDS_LIST_PATH),
            input_words: InputWords::new(),
        }
    }
}
