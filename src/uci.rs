/// Represents a UCI command.
pub enum UciCommand {
    Uci,
}

/// Tries to parse a string as a UCI command.
pub fn parse_uci(input: String) -> Result<UciCommand, String> {
    // split input string into parts
    let uci_parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    
    // if uci_parts is empty, return error
    if uci_parts.is_empty() {
        return Err(String::from("info string unknown command"))
    }
    
    // return matching uci command, otherwise error
    match uci_parts[0].as_str() {
        "uci" => Ok(UciCommand::Uci),
        _other => Err(String::from("info string unknown command")),
    }
}