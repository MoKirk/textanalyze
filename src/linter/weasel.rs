use crate::linter::Linter;
use crate::linter::LinterResult;
use crate::text::Text;

pub struct WeaselLinter;

impl WeaselLinter {
    /// returns a weasellinter instance
    pub fn new() -> Self {
        WeaselLinter {}
    }

    /// sets internal language (i think this needs to be set via config)
    pub fn set_language(&self, lang: &str) {
        println!("{}", lang);
    }
}

impl Linter for WeaselLinter {
    /// implements the analyze for the weasel words
    fn analyze_text(&self, _text: &Text) -> Vec<LinterResult> {
        Vec::new()
    }
}
