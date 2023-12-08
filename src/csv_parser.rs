use std::fs::File;

use csv::Reader;

use crate::util::exit_with_message;

pub struct CSVParser {
    reader: Reader<File>,
}

impl CSVParser {
    pub fn new(file_path: impl Into<String>) -> CSVParser {
        let file_path: String = file_path.into();

        let rdr = match csv::Reader::from_path(&file_path) {
            Ok(rdr) => rdr,
            Err(err) => exit_with_message(format!("Ops! não consegui abrir o arquivo {}. Tem certeza que você está rodando o programa no mesmo diretório deste arquivo? Mais detalhes:\n{}", file_path, err), 1),
        };

        CSVParser { reader: rdr }
    }
}
