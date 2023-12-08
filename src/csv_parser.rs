use std::{error::Error, fs::File};

use csv::{Reader, Writer};

#[derive(Debug)]
pub struct CSVParser {
    reader: Reader<File>,
    writer: Writer<File>,
}

#[derive(Debug, serde::Deserialize)]
struct CSVRow {
    #[serde(rename = "_IDSKU (Não alterável)")]
    id: String,
    #[serde(rename = "descrição gerada pela ia")]
    value: String,
}

impl CSVParser {
    pub fn new(
        input_file_path: impl Into<String>,
        output_file_path: impl Into<String>,
    ) -> Result<CSVParser, String> {
        let input_file_path: String = input_file_path.into();
        let output_file_path: String = output_file_path.into();

        let reader = match csv::Reader::from_path(&input_file_path) {
            Ok(rdr) => rdr,
            Err(err) => return Err(format!("Ops! não consegui abrir o arquivo {}. Tem certeza que você está rodando o programa no mesmo diretório deste arquivo? Mais detalhes:\n{}", input_file_path, err)) 
        };

        let writer = match csv::Writer::from_path(&output_file_path) {
            Ok(writer) => writer,
            Err(err) => return Err(format!("Ops! Estou tentando salvar as informações no arquivo \"{}\" mas estou tendo uns problemas =/ Mais detalhes:\n{}", output_file_path, err)) 
        };

        Ok(CSVParser { reader, writer })
    }

    pub fn parse(&mut self) -> Result<(), Box<dyn Error>> {
        for result in self.reader.deserialize() {
            let csv_row: CSVRow = result?;
            println!("{:?}", csv_row);
            break;
        }

        Ok(())
    }
}
