use crate::analyzer::TextAnalyzer;
use crate::linter::*;
use crate::text::Text;

/// Setup
fn setup_linter(input: &str) -> (WeaselLinter, TextAnalyzer) {
    let testtext = Text::from_string(input);
    (WeaselLinter::new(), TextAnalyzer::new(testtext))
}

#[test]
fn it_can_detect_weasel_words() {
    let (linter, analyzer) = setup_linter("sehr worte");
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 1 );
}

#[test]
fn it_can_detect_multiple_weasel_words() {
    let (linter, analyzer) = setup_linter("sehr viele worte");
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 1 );
}

#[test]
fn it_returns_false_if_no_words_are_present() {
    let (linter, analyzer) = setup_linter(":)");
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 0 );
}

#[test]
fn it_can_work_with_diffrent_languages() {
    // TODO
    assert!(false)
}
