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
                    return Some(LinterResult::new(
                        &format!("found duplicate {}", word),
                        ResultType::Warning,
                        ResultLocationType::Sentence,
                        inner_pos,
                    ));
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
        let mut result: Vec<LinterResult> = Vec::new();
        for sentence in sentences {
            if let Some(lint) = self.has_duplicate(sentence) {
                result.push(lint);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::analyzer::TextAnalyzer;
    use crate::linter::*;
    use crate::text::Text;
    use crate::text::TextLanguage;
    /// Setup
    fn setup_linter(input: &str) -> (DuplicateLinter, TextAnalyzer) {
        let testtext = Text::from_string(input, TextLanguage::English);
        (DuplicateLinter::new(), TextAnalyzer::new(testtext))
    }

    #[test]
    fn it_can_detect_duplicates_in_a_sentence() {
        let (linter, analyzer) = setup_linter("yes yes duplicates here");
        let result = analyzer.run_linter(Box::new(linter));
        assert_eq!(1, result.iter().count());
    }

    #[test]
    fn it_cant_detect_duplicates_in_a_text_without_them() {
        let (linter, analyzer) = setup_linter("no duplicates here");
        let result = analyzer.run_linter(Box::new(linter));
        assert_eq!(0, result.iter().count());
    }
}
