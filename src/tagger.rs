use regex::Regex;

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
        let html_regex = Regex::new(r"^<\w+( .*)*>.*<\/\w+>$").unwrap();

        self.input_text.split("\n").for_each(|line| {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                return;
            }

            if html_regex.is_match(trimmed) {
                self.output_text.push_str(trimmed);
                self.output_text.push_str("\n\n");
            } else {
                self.output_text.push_str("<p>");
                self.output_text.push_str(trimmed);
                self.output_text.push_str("</p>");
                self.output_text.push_str("\n\n");
            }
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
        assert_eq!(tagger.get_output_text(), "<p>banana</p>\n\n");
    }

    #[test]
    fn test_tag_multiple_lines() {
        let mut tagger = TextTagger::new("\n\nuva\n\nbanana\n\n");
        tagger.tag();
        assert_eq!("<p>uva</p>\n\n<p>banana</p>\n\n", tagger.get_output_text());
    }

    #[test]
    fn test_ignore_html_tags() {
        let mut tagger = TextTagger::new("<p>banana</p>\nuva\nmaçã\n<h1>laranja</h1>\npera");
        tagger.tag();
        assert_eq!(
            "<p>banana</p>\n\n<p>uva</p>\n\n<p>maçã</p>\n\n<h1>laranja</h1>\n\n<p>pera</p>\n\n",
            tagger.get_output_text()
        );
    }

    #[test]
    fn test_output() {
        let mut tagger = TextTagger::new("banana");
        tagger.tag();
        assert_eq!(tagger.get_output_text(), "<p>banana</p>\n\n");
    }
}
