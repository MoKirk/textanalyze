use crate::linter::*;
use crate::text::Text;
use std::fs;

pub struct WeaselLinter;

impl WeaselLinter {
    /// returns a weasellinter instance
    pub fn new() -> Self {
        WeaselLinter {}
    }


    /// returns the warning, that a certain word is found
    fn add_warning(&self, word: String) -> LinterResult {
        LinterResult::new(
            &format!("found weaselword {}", word),
            ResultType::Warning,
            ResultLocationType::Sentence,
            0
        )
    }
}

impl Linter for WeaselLinter {
    /// implements the analyze for the weasel words
    fn analyze_text(&self, text: &Text) -> Vec<LinterResult> {
        let mut result : Vec<LinterResult> = Vec::new();
        let wordlist = WeaselWordlist::for_lang(text.get_language_shortcode());
        for sentence in text.as_sentences() {
            if let Some(word) = wordlist.is_found_in(sentence) {
                result.push(self.add_warning(word));
            }
        }
        result
    }
}



struct WeaselWordlist {
    words: Vec<String>
}

impl WeaselWordlist {
    /// Builds a wordlist for the weaselwords
    pub fn for_lang(lang: &str ) -> Self {
        let mut wl = WeaselWordlist {
            words: Vec::new()
        };
        wl.read(lang);
        return wl;
    }

    /// reads the input file and builds the word array
    pub fn read(&mut self, lang: &str) {
        // TODO: This loading can be made better, maybe wrap it into a data layer?
        let file = format!("src/data/{}/weasel_words.txt", lang);
        let filecontent = fs::read_to_string(file).unwrap();
        let words : Vec<&str> = filecontent.lines().collect();
        for word in words {
            &self.words.push(word.to_string());
        }
    }


    /// checks if a word from the wordlist is found in the option
    pub fn is_found_in(&self, sentence: &str) -> Option<String> {
        let words_of_the_sentence : Vec<&str> = sentence.split_whitespace().collect();
        for word in &self.words {
            if words_of_the_sentence.contains(&word.as_str()) {
                return Some(word.to_string());
            }
        }
        return None;
    }
}

