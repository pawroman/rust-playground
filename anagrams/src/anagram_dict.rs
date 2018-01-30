use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


// Word dictionary - character-wise sorted words mapped to matches in the word file
// E.g. "abc" -> {"abc", "cba"}
type WordSet = HashSet<String>;

pub type AnagramDictionary = HashMap<String, WordSet>;


// LEARNING: can't add methods for type aliases - must use traits.
//           Alternatively we'd wrap AnagramDictionary to be a struct containing the HashMap.
pub trait AnagramDictTrait: Sized {
    fn load_from_path(words_list_path: &Path, case_sensitive: bool) -> Result<Self, io::Error>;
    fn lookup(self: &Self, word: &str, case_sensitive: bool) -> Option<Vec<&str>>;
}

// Skip all dictionary entries with this suffix
static SKIP_SUFFIX: &'static str = "'s";


impl AnagramDictTrait for AnagramDictionary {
    /// Load the anagram dictionary from the specified path
    fn load_from_path(words_list_path: &Path, case_sensitive: bool) -> Result<AnagramDictionary, io::Error> {
        // Handle missing file case nicely
        if !(&words_list_path.is_file()) {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                // LEARNING: if to_str() fails (returns None), expect will cause runtime panic
                format!("Can't find file: {}", words_list_path.to_str().expect("Invalid path"))
            ));
        }

        // ? returns the Ok Result, or causes the function to return the Error early
        // (equiv. to try!(..))
        let file = File::open(words_list_path)?;
        let reader = io::BufReader::new(file);
        let mut dict = AnagramDictionary::new();

        // assume the word list is one word per line
        // Result.ok() -> Option (None if error)
        // filter_map -> skips None options
        for line in reader.lines().filter_map(|l| l.ok()) {
            if let Some(key) = anagram_key(&line, case_sensitive) {
                let value = dict.entry(key).or_insert_with(WordSet::new);

                value.insert(line);
            }
        }

        Ok(dict)
    }

    /// Find anagrams.
    fn lookup<'a>(self: &'a AnagramDictionary, word: &str, case_sensitive: bool) -> Option<Vec<&'a str>> {
        if let Some(key) = anagram_key(word, case_sensitive) {
            if let Some(values) = self.get(&key) {
                let mut anagrams: Vec<_> = values
                    .iter()
                    .filter(|val| val.to_lowercase() != word.to_lowercase())
                    .map(|s| s.as_ref())
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
}


fn anagram_key(word: &str, case_sensitive: bool) -> Option<String> {
    // LEARNING: need to use String::from_iter
    use std::iter::FromIterator;

    if word.ends_with(SKIP_SUFFIX) {
        None
    }
    else {
        let base: String = if case_sensitive {
            word.to_string()
        } else {
            word.to_lowercase()
        };

        let mut chars_vec = base.chars().collect::<Vec<_>>();
        chars_vec.sort();

        Some(String::from_iter(chars_vec.into_iter()))
    }
}
