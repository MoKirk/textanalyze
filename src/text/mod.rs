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

#[cfg(test)]
mod tests {
    use crate::text::*;

    #[test]
    fn it_can_be_created_from_a_string() {
        let txt = Text::from_string("abcd", TextLanguage::English);
        assert_eq!(txt.inner_text(), "abcd");
    }

    #[test]
    fn it_can_split_the_text_by_lines() {
        let txt = Text::from_string("abcd\nefgh", TextLanguage::English);
        let mut lines = txt.as_lines();
        assert_eq!("abcd", lines.next().unwrap());
        assert_eq!("efgh", lines.next().unwrap());
    }

    #[test]
    fn it_can_split_the_text_by_sentences() {
        let txt = Text::from_string("abcd. efgh? ijk! mnop. qrz ast", TextLanguage::English);
        let mut sentences = txt.as_sentences().into_iter();
        assert_eq!("abcd.", sentences.next().unwrap());
        assert_eq!("efgh?", sentences.next().unwrap());
        assert_eq!("ijk!", sentences.next().unwrap());
        assert_eq!("mnop.", sentences.next().unwrap());
        assert_eq!("qrz ast", sentences.next().unwrap());
    }

    #[test]
    fn it_can_have_a_language() {
        let txt = Text::from_string("abcd. efgh? ijk! mnop. qrz ast", TextLanguage::English);
        assert!(match txt.get_language() {
            TextLanguage::English => true,
            _ => false,
        });
    }

    #[test]
    fn it_can_return_the_language_shortcode() {
        let txt = Text::from_string("abcd. efgh? ijk! mnop. qrz ast", TextLanguage::English);
        assert_eq!("en", txt.get_language_shortcode());
    }
}
