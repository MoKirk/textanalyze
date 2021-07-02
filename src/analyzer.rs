use crate::linter::*;
use crate::text::Text;
/// This struct represents a text analyzer
pub struct TextAnalyzer {
    text: Text,
    linter: Vec<Box<dyn Linter>>,
}

impl TextAnalyzer {
    /// returns a new textanalyzer
    pub fn new(text: Text) -> Self {
        TextAnalyzer {
            text,
            linter: Vec::new(),
        }
    }

    /// this adds a linter to the array
    pub fn add_linter(&mut self, lin: Box<dyn Linter>) {
        self.linter.push(lin);
    }

    /// This returns the linter array
    pub fn linter(&self) -> &Vec<Box<dyn Linter>> {
        &self.linter
    }

    /// Runs all registered linters
    pub fn run(&self) -> Vec<LinterResult> {
        let mut results: Vec<LinterResult> = Vec::new();
        for linter in &self.linter {
            results.append(&mut linter.analyze_text(&self.text));
        }
        results
    }

    /// runs a provided linter
    pub fn run_linter(&self, lin: Box<dyn Linter>) -> Vec<LinterResult> {
        lin.analyze_text(&self.text)
    }
}
