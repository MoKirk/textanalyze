mod testlinter;
pub use testlinter::TestLinter;

mod weasel;
pub use weasel::WeaselLinter;

mod duplicate;
pub use duplicate::DuplicateLinter;

use crate::text::Text;

/// Possible result types
pub enum ResultType {
    Warning,
    Debug,
    Error,
}

/// This is the result type, where the result is found
pub enum ResultLocationType {
    Sentence,
    Line,
    None,
}

/// This is the location, where the result is found
pub struct ResultLocation {
    _location_type: ResultLocationType,
    _position: usize,
}

/// This is the linter result struct
pub struct LinterResult {
    pub message: String,
    pub result_type: ResultType,
    pub result_location: ResultLocation,
}

impl LinterResult {
    /// returns a new result object
    pub fn new(
        message: &str,
        result_type: ResultType,
        location_type: ResultLocationType,
        position: usize,
    ) -> LinterResult {
        LinterResult {
            message: message.to_string(),
            result_type,
            result_location: ResultLocation {
                _location_type: location_type,
                _position: position,
            },
        }
    }
}

/// this is the defintion of a linter
pub trait Linter {
    /// analyzes a text
    fn analyze_text(&self, text: &Text) -> Vec<LinterResult>;
}
