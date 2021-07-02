// Those are the supported languages right now
pub enum TextLanguage {
    English,
    German,
}

/**
 * A text represents the input text. it has an inner text, as well as a language enum
 */
pub struct Text {
    inner_text: String,
    language: TextLanguage,
}

impl Text {
    /// Creates a text object from a string
    pub fn from_string(inner_text: &str, lang: TextLanguage) -> Self {
        Text {
            inner_text: inner_text.to_string(),
            language: lang,
        }
    }

    /// Returns the inner text from a text
    pub fn inner_text(&self) -> &String {
        &self.inner_text
    }

    /// returns the text as lines
    pub fn as_lines(&self) -> std::str::Lines {
        self.inner_text.lines()
    }

    /// returns the text as sentences
    pub fn as_sentences(&self) -> Vec<&str> {
        let sentences: Vec<&str> = self
            .inner_text
            .split_inclusive(&['.', '!', '?'][..])
            .collect();
        let mut trimmed_sentences = Vec::new();
        for sentence in sentences {
            trimmed_sentences.push(sentence.trim());
        }
        trimmed_sentences
    }

    /// returns internal lang
    pub fn get_language(&self) -> &TextLanguage {
        &self.language
    }


    pub fn get_language_shortcode(&self) -> &str {
     match self.language {
      TextLanguage::English => "en",
      TextLanguage::German => "de",
     }
    }
}
