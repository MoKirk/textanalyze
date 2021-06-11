use crate::analyzer::TextAnalyzer;
use crate::linter::*;
use crate::text::Text;

#[test]
/// checks that a linter can be run
fn it_can_add_linters_to_the_analyzer() {
    let check = "foobar_a";
    let testtext = Text::from_string(check);
    let mut analyzer = TextAnalyzer::new(testtext);
    let testlinter = TestLinter::new();
    analyzer.add_linter(Box::new(testlinter));
    assert_eq!(1, analyzer.linter().len());
    let result = analyzer.run();
    let result_to_check = result.iter().next().unwrap();
    assert_eq!(result_to_check.message, check);
}


#[test]
/// checks that it can run a single linter
fn it_can_run_a_single_linter() {
    let check = "foobar_b";
    let testtext = Text::from_string(check);
    let mut analyzer = TextAnalyzer::new(testtext);
    let testlinter = TestLinter::new();
    let result = analyzer.run_linter(Box::new(testlinter));
    let result_to_check = result.iter().next().unwrap();
    assert_eq!(result_to_check.message, check);
}


#[test]
/// a linter can return multiple results
fn it_can_return_multiple_results() {
    let check = "foo\nbar"; // testlinter should split this one
    let testtext = Text::from_string(check);
    let mut analyzer = TextAnalyzer::new(testtext);
    let testlinter = TestLinter::new();
    analyzer.add_linter(Box::new(testlinter));
    assert_eq!(1, analyzer.linter().len());
    let result = analyzer.run();
    assert_eq!(2, result.len()); 
    let result_to_check = result.iter().next().unwrap();
    assert_eq!(result_to_check.message, "foo");
}


#[test]
/// Checks the result
fn a_result_has_a_type_and_a_location() {
    let result = Result::new("not okay", ResultType::Warning, ResultLocationType::Line, 1);
    assert_eq!(result.message, "not okay");
    assert!(match result.result_type {
        ResultType::Warning => true,
        _ => false
    });
}
