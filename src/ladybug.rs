use std::sync::mpsc::{Receiver, Sender};
use crate::uci;
use crate::uci::{UciCommand};

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
    /// Starts running Ladybug.
    pub fn run(&self, output_sender: Sender<String>, input_sender: Receiver<String>) {
        loop {
            // blocks until Ladybug receives input
            let input = input_sender.recv();
            
            // if the input thread closes the connection, Ladybug must not continue running
            if input.is_err() {
                panic!("The input thread has unexpectedly closed the channel connection.")
            }
            
            // get the input string from the result
            let input = input.unwrap();
            
            // try to parse the uci command
            let uci_command = uci::parse_uci(input);
            
            let uci_command = match uci_command {
                // if the uci command cannot be parsed, send the error message to the output thread
                Err(message) => {
                    Self::send_output(&output_sender, message);
                    continue;
                }
                Ok(command) => command
            };
            
            // delegate the handling of the uci command to the respective method
            match uci_command {
                UciCommand::Uci => self.handle_uci(&output_sender),
            }
        }
    }
    
    /// Sends the given String to the output thread.
    fn send_output(output_sender: &Sender<String>, output: String) {
        let send_result = output_sender.send(output);
        
        // if the output thread closes the connection, Ladybug must not continue running
        if send_result.is_err() {
            panic!("The output thread has unexpectedly closed the channel connection.")
        }
    }
    
    /// Handles the "uci" command
    fn handle_uci(&self, output_sender: &Sender<String>) {
        Self::send_output(output_sender, format!("id name {}", self.name));
        Self::send_output(output_sender, format!("id author {}", self.author));
        Self::send_output(output_sender, String::from("uciok"));
    }
}

