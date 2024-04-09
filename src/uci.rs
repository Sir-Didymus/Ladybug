/// Represents a UCI command.
#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::uci;
    use crate::uci::UciCommand;

    #[test]
    fn parse_uci_with_invalid_input_returns_error() {
        assert_eq!(Err(String::from("info string unknown command")), uci::parse_uci(String::from("Not Uci")));
        assert_eq!(Err(String::from("info string unknown command")), uci::parse_uci(String::from("       ")));
        assert_eq!(Err(String::from("info string unknown command")), uci::parse_uci(String::from("123456789")));
    }

    #[test]
    fn test_parse_uci_for_uci() {
        assert_eq!(UciCommand::Uci, uci::parse_uci(String::from("uci")).unwrap());
    }
}