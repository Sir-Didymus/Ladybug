use std::sync::mpsc::Receiver;
use crate::uci::{parse_uci, UciCommand};

/// The main character in this project!
/// The Ladybug struct acts as the UCI client and can receive and handle UCI commands.
pub struct Ladybug {
    name: String,
    author: String,
}

impl Default for Ladybug {
    /// Constructs Ladybug.
    fn default() -> Self {
        Self {
            name: String::from("Ladybug 0.1.0"),
            author: String::from("Felix O."),
        }
    }
}

impl Ladybug {
    /// Start running Ladybug.
    pub fn run(&self, receiver: Receiver<String>) {
        loop {
            let mut input = String::new();
            
            input = receiver.recv().unwrap();
            
            let uci_command = parse_uci(input).unwrap();
            
            match uci_command {
                UciCommand::Uci => self.handle_uci(),
            }
        }
    }
    
    fn handle_uci(&self) {
        println!("id name {}", self.name);
        println!("id author {}", self.author);
    }
}

