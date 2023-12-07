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

    fn tag(&mut self) {}

    fn get_output_text(&self) -> String {
        self.output_text.clone()
    }
}
