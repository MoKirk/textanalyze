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
fn it_can_detect_multiple_weasel_words() {
    let (linter, analyzer) = setup_linter("very many words");
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 2);
    // TODO: CHECK POSITION AND WARNING
}

#[test]
fn it_returns_false_if_no_words_are_present() {
    let (linter, analyzer) = setup_linter(":)");
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 0);
}

#[test]
fn it_can_work_with_diffrent_languages() {
    let (linter, analyzer) = setup_linter("viele worte");
    linter.set_language("de"); // TODO: How do I do this? This is bad
    let result = analyzer.run_linter(Box::new(linter));
    assert!(result.len() == 1);
    assert!(false)
}
