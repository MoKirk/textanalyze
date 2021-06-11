mod testlinter;
pub use testlinter::TestLinter;
use crate::text::Text;

/// Possible result types
pub enum ResultType {
    Warning, Debug, Error,
}

/// This is the result type, where the result is found
pub enum ResultLocationType {
    Sentence, Line, None
}

/// This is the location, where the result is found
pub struct ResultLocation {
    location_type: ResultLocationType,
    position: usize
}


/// This is the linter result struct
pub struct Result {
    pub message: String,
    pub result_type: ResultType,
    pub result_location: ResultLocation
}


impl Result {
    /// returns a new result object
    pub fn new(message: &str, result_type: ResultType, location_type: ResultLocationType, position: usize) -> Result {
        Result {
            message: message.to_string(),
            result_type,
            result_location: ResultLocation { location_type, position }
        }
    }
}

/// this is the defintion of a linter
pub trait Linter {
    /// analyzes a text
    fn analyze_text(&self, text: &Text) -> Vec<Result>;
}
