use crate::linter::*;
use crate::text::Text;

/// This Linter is used for testing purposes
pub struct TestLinter {
    pub found_text: String
}

impl TestLinter {
    /// returns a new linter object
    pub fn new() -> Self {
        TestLinter{ found_text: String::new() }
    }
}


impl Linter for TestLinter {
    /// implements the linter for the testlinter
    fn analyze_text(&self, text: &Text) -> Result {
        let message = text.inner_text().clone();
        return Result::new(&message, ResultType::Debug, ResultLocationType::None, 0);
    }
}
