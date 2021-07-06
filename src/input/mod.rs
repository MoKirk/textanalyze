use std::fs;
use crate::text::Text;
use crate::text::TextLanguage;

/// This is the input for the textanalyzer
pub struct InputFile {
    content: String
}

impl InputFile {
    /// Returns a file content for a data path
    pub fn for_data_path(file: &str) -> Option<Self> {
        let file = format!("src/data/{}", file);
        InputFile::for_path(&file)
    }

    /// returns a file content for a path
    pub fn for_path(file: &str) -> Option<Self> {
        let filecontent = fs::read_to_string(file).ok()?;
        return Some(InputFile {
            content: filecontent
        });
    }


    /// Returns the internal file string
    pub fn to_string(&self) -> String {
        self.content.clone()
    }

    /// returns a text object for the inputfile
    pub fn to_text(&self) -> Text {
        // TODO: fix language
        Text::from_string(&self.content.clone(), TextLanguage::German)
    }
}

#[cfg(test)]
mod tests {
    use super::InputFile;

    #[test]
    fn it_can_read_a_data_file() {
        let file = "test/example.txt"; // is in the data folder
        let filecontents = InputFile::for_data_path(file).expect("file not found");
        assert!(filecontents.to_string().contains("testing")); // testing is a word found in the file
    }

    #[test]
    fn it_can_read_a_file_and_returns_a_text() {
        let file = "src/data/test/example.txt"; 
        let filecontents = InputFile::for_path(file).expect("file not found");
        let text = filecontents.to_text();
        assert!(text.inner_text().contains("testing"));
    }


    #[test] 
    fn it_can_detect_if_a_file_is_not_there() {
        let file = "test/notexistent.txt"; // is in the data folder
        assert!(match InputFile::for_data_path(file) {
            None => true,
            _ => false
        });
    }
}
