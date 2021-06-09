pub struct Text {
    inner_text: String
}


impl Text {
    /// Creates a text object from a string
    pub fn from_string(inner_text: &str) -> Self {
        Text { inner_text: inner_text.to_string() }
    }

    /// Returns the inner text from a text
    pub fn inner_text(&self) -> &String {
        &self.inner_text
    }

    /// returns the text as lines
    pub fn as_lines(&self) -> std::str::Lines {
        let lines = self.inner_text.lines();
        lines
    }

    /// returns the text as sentences
    pub fn as_sentences(&self) -> Vec<&str> {
        let sentences : Vec<&str> = self.inner_text.split_inclusive(&['.', '!', '?'][..]).collect();
        let mut trimmed_sentences = Vec::new();
        for sentence in sentences {
            trimmed_sentences.push(sentence.trim());
        }
        trimmed_sentences
    }
}
