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
