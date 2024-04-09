pub enum UciCommand {
    Uci,
}

pub fn parse_uci(input: String) -> Result<UciCommand, String> {
    match input.as_str() {
        "uci\r\n" => Ok(UciCommand::Uci),
        _other => {
            println!("\"{}\"", _other);
            Err(String::from("Invalid Input"))
        }
    }
}