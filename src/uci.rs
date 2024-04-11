/// Represents a UCI command.
#[derive(PartialEq, Debug)]
pub enum UciCommand {
    Uci,
    IsReady,
    Position(Vec<String>),
    GoClockTime(Vec<String>),
    GoPerft(String),
    Quit,
    Help,
    Display,
}

/// Tries to parse a string as a UCI command.
pub fn parse_uci(input: String) -> Result<UciCommand, String> {
    // split input string into parts
    let mut uci_parts: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    
    // if uci_parts is empty, return error
    if uci_parts.is_empty() {
        return Err(String::from("info string unknown command"))
    }
    
    // return matching uci command, otherwise error
    match uci_parts[0].as_str() {
        "uci" => Ok(UciCommand::Uci),
        "isready" => Ok(UciCommand::IsReady),
        "position" => {
            match uci_parts.len() > 1 {
                false => Err(String::from("info string unknown command")),
                true => Ok(UciCommand::Position(uci_parts.split_off(1)))
            }
        }
        "go" => {
            if uci_parts.len() < 2 {
                Err(String::from("info string unknown command"))
            } else {
                match uci_parts[1].as_str() {
                    "perft" => {
                        if uci_parts.len() != 3 {
                            Err(String::from("info string unknown command"))
                        }
                        else {
                            Ok(UciCommand::GoPerft(uci_parts[2].clone()))
                        }
                    }
                    "wtime" => Ok(UciCommand::GoClockTime(uci_parts.split_off(1))),
                    _other => Err(String::from("info string unknown command"))
                }
            }
        }
        "quit" => Ok(UciCommand::Quit),
        "help" => Ok(UciCommand::Help),
        "display" => Ok(UciCommand::Display),
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

    #[test]
    fn test_parse_uci_for_isready() {
        assert_eq!(UciCommand::IsReady, uci::parse_uci(String::from("isready")).unwrap());
    }

    #[test]
    fn test_parse_uci_for_position() {
        assert_eq!(Err(String::from("info string unknown command")), uci::parse_uci(String::from("position")));
        
        assert_eq!(Ok(UciCommand::Position(vec!(String::from("startpos"), String::from("moves"), String::from("h3h4"), String::from("c6g2")))),
                   uci::parse_uci(String::from("position startpos moves h3h4 c6g2")));

        assert_eq!(Ok(UciCommand::Position(vec!(String::from("fen"), String::from("8/B6p/2b1k1p1/5p2/2PK4/6PP/6P1/8"), String::from("w"), String::from("-"),
                                                String::from("-"), String::from("1"), String::from("45"),
                                                 String::from("moves"), String::from("h3h4"), String::from("c6g2")))),
                   uci::parse_uci(String::from("position fen 8/B6p/2b1k1p1/5p2/2PK4/6PP/6P1/8 w - - 1 45 moves h3h4 c6g2")));
    }

    #[test]
    fn test_parse_uci_for_go_clock_time() {
        assert_eq!(UciCommand::GoClockTime(vec!["wtime".to_string(), "300000".to_string(), "btime".to_string(), "300000".to_string(), "winc".to_string(), "0".to_string(), "binc".to_string(), "0".to_string()]), 
                   uci::parse_uci(String::from("go wtime 300000 btime 300000 winc 0 binc 0")).unwrap());
    }

    #[test]
    fn test_parse_uci_for_go_perft() {
        assert_eq!(Err(String::from("info string unknown command")), uci::parse_uci(String::from("go perft")));
        assert_eq!(Err(String::from("info string unknown command")), uci::parse_uci(String::from("go perft one two")));
        
        assert_eq!(UciCommand::GoPerft(String::from("5")), uci::parse_uci(String::from("go perft 5")).unwrap());
        assert_eq!(UciCommand::GoPerft(String::from("0")), uci::parse_uci(String::from("go perft 0")).unwrap());
        assert_eq!(UciCommand::GoPerft(String::from("100")), uci::parse_uci(String::from("go perft 100")).unwrap());
    }

    #[test]
    fn test_parse_uci_for_quit() {
        assert_eq!(UciCommand::Quit, uci::parse_uci(String::from("quit")).unwrap());
    }

    #[test]
    fn test_parse_uci_for_help() {
        assert_eq!(UciCommand::Help, uci::parse_uci(String::from("help")).unwrap());
    }

    #[test]
    fn test_parse_uci_for_display() {
        assert_eq!(UciCommand::Display, uci::parse_uci(String::from("display")).unwrap());
    }
}