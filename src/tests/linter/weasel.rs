use crate::analyzer::TextAnalyzer;
use crate::linter::*;
use crate::text::Text;
use crate::text::TextLanguage;

/// Setup
fn setup_linter(input: &str) -> (WeaselLinter, TextAnalyzer) {
    let testtext = Text::from_string(input, TextLanguage::English);
    (WeaselLinter::new(), TextAnalyzer::new(testtext))
}

#[test]
fn it_can_detect_weasel_words() {
    let (linter, analyzer) = setup_linter("many words");
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 1);
}

#[test]
fn it_returns_false_if_no_words_are_present() {
    let (linter, analyzer) = setup_linter(":)");
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 0);
}

#[test]
fn it_can_work_with_diffrent_languages() {
    let testtext = Text::from_string("sehr viele worte", TextLanguage::German);
    let linter = WeaselLinter::new();
    let analyzer = TextAnalyzer::new(testtext);
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 1);
}
