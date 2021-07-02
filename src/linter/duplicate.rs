use crate::linter::*;
use crate::text::Text;

pub struct DuplicateLinter {}

impl DuplicateLinter {
    /// returns a weasellinter instance
    pub fn new() -> Self {
        DuplicateLinter {}
    }

    /// finds a duplicate in a string
    fn has_duplicate(&self, sentence: &str) -> Option<LinterResult> {
        let words = sentence.split(' ');
        let mut pos = 0;
        for word in words {
            let inner_words = sentence.split(' ');
            let mut inner_pos = 0;
            for inner in inner_words {
                if inner_pos > pos && word == inner {
                    return Some(LinterResult {
                        message: format!("found duplicate {}", word),
                        result_type: ResultType::Warning,
                        result_location: ResultLocation {
                            _location_type: ResultLocationType::Sentence,
                            _position: inner_pos
                        }

                    });
                }
                inner_pos += 1;
            }
            pos += 1;
        }
        None
    }
}

impl Linter for DuplicateLinter {
    /// implements the analyze for the weasel words
    fn analyze_text(&self, text: &Text) -> Vec<LinterResult> {
        let sentences = text.as_sentences();
        let mut result  : Vec<LinterResult>= Vec::new();
        for sentence in sentences {
            if let Some(lint) = self.has_duplicate(sentence) {
                result.push(lint);
            }
        }
        result
    }
}
