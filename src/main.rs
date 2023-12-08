mod csv_parser;
mod tagger;
mod util;

use std::{error::Error, fs};

use util::exit_with_message;

const INPUT_FILE: &str = "input.csv";
const OUTPUT_FILE: &str = "output.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let mut parser = match csv_parser::CSVParser::new(INPUT_FILE, OUTPUT_FILE) {
        Ok(parser) => parser,
        Err(msg) => exit_with_message(msg, 1),
    };

    parser.parse()?;

    Ok(())

    // let content = match get_content() {
    //     Ok(content) => content,
    //     Err(err) => exit_with_message(err, 1),
    // };
    //
    // let mut tagger = TextTagger::new(content);
    // tagger.tag();
    //
    // match write_result(tagger.get_output_text()) {
    //     Ok(_) => exit_with_message("Arquivo de saída gerado com sucesso!", 0),
    //     Err(err) => exit_with_message(err, 1),
    // }
}
