pub fn exit_with_message(msg: impl Into<String>, status: i32) -> ! {
    println!("{}", msg.into());
    println!("Aperte qualquer tecla para encerrar...");
    let _ = std::io::stdin().read_line(&mut String::new());
    std::process::exit(status);
}
