pub struct TextTagger {
    input_text: String,
    output_text: String,
}

impl TextTagger {
    pub fn new(input_text: impl Into<String>) -> TextTagger {
        TextTagger {
            input_text: input_text.into(),
            output_text: String::new(),
        }
    }

    pub fn tag(&mut self) {
        self.input_text.split("\n").for_each(|line| {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                return;
            }

            self.output_text.push_str("<p>");
            self.output_text.push_str(trimmed);
            self.output_text.push_str("</p>");
            self.output_text.push_str("\n");
        })
    }

    pub fn get_output_text(&self) -> String {
        self.output_text.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag() {
        let mut tagger = TextTagger::new("banana");
        tagger.tag();
        assert_eq!(tagger.get_output_text(), "<p>banana</p>\n");
    }

    #[test]
    fn test_tag_multiple_lines() {
        let mut tagger = TextTagger::new("\n\nuva\n\nbanana\n\n");
        tagger.tag();
        assert_eq!("<p>uva</p>\n<p>banana</p>\n", tagger.get_output_text());
    }

    #[test]
    fn test_output() {
        let mut tagger = TextTagger::new("banana");
        tagger.tag();
        assert_eq!(tagger.get_output_text(), "<p>banana</p>\n");
    }
}
