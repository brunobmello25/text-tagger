mod tagger;

use std::fs;

use crate::tagger::TextTagger;

const INPUT_FILE: &str = "input.txt";
const OUTPUT_FILE: &str = "output.txt";

fn main() {
    let content = match get_content() {
        Ok(content) => content,
        Err(err) => exit_with_message(err, 1),
    };

    let mut tagger = TextTagger::new(content);
    tagger.tag();

    match write_result(tagger.get_output_text()) {
        Ok(_) => exit_with_message("Arquivo de saída gerado com sucesso!", 0),
        Err(err) => exit_with_message(err, 1),
    }
}

fn get_content() -> Result<String, String> {
    let content = fs::read(INPUT_FILE).map_err(|_| "Epa! Não consegui ler o arquivo de entrada. Tem certeza que você está rodando o programa no mesmo diretório que arquivo \"input.txt\"?")?;

    String::from_utf8(content).map_err(|e| format!(
        "Epa! Tive um problema na hora de ler o conteúdo do arquivo de entrada. Mostra isso aqui pro gostoso do seu namorado resolver: \n {}",e
    ))
}

fn exit_with_message(msg: impl Into<String>, status: i32) -> ! {
    println!("{}", msg.into());
    println!("Aperte qualquer tecla para encerrar...");
    let _ = std::io::stdin().read_line(&mut String::new());
    std::process::exit(status);
}

fn write_result(result: impl Into<String>) -> Result<(), String> {
    fs::write(OUTPUT_FILE, result.into()).map_err(|err| format!("Epa! Não consegui escrever o arquivo de saída! Mostra isso aqui pro gostoso do seu namorado: \n {}", err))
}
