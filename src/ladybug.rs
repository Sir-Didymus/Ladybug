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
                UciCommand::IsReady => self.handle_is_ready(&output_sender)
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
    
    /// Handles the "uci" command.
    fn handle_uci(&self, output_sender: &Sender<String>) {
        Self::send_output(output_sender, format!("id name {}", self.name));
        Self::send_output(output_sender, format!("id author {}", self.author));
        Self::send_output(output_sender, String::from("uciok"));
    }

    /// Handles the "isready" command.
    fn handle_is_ready(&self, output_sender: &Sender<String>) {
        Self::send_output(output_sender, String::from("readyok"));
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use crate::ladybug::Ladybug;

    /// Creates a new Ladybug thread and returns the input_sender and output_receiver.
    fn setup() -> (Sender<String>, Receiver<String>) {
        // create input_sender and input_receiver so that the input thread can send input to the ladybug thread
        let (input_sender, input_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

        // create output_sender and output_receiver so that the ladybug thread can send output to the output thread.
        let (output_sender, output_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

        let ladybug = Ladybug::default();

        // spawn the Ladybug thread
        thread::spawn(move || ladybug.run(output_sender, input_receiver));

        (input_sender, output_receiver)
    }

    #[test]
    fn test_ladybug_invalid_uci_input_prints_error_message() {
        let (input_sender, output_receiver) = setup();
        
        let _ = input_sender.send(String::from("Not Uci"));
        assert_eq!("info string unknown command", output_receiver.recv().unwrap());

        let _ = input_sender.send(String::from("       "));
        assert_eq!("info string unknown command", output_receiver.recv().unwrap());

        let _ = input_sender.send(String::from("123456789"));
        assert_eq!("info string unknown command", output_receiver.recv().unwrap());
    }

    #[test]
    fn test_ladybug_for_uci() {
        let (input_sender, output_receiver) = setup();
        
        let _ = input_sender.send(String::from("uci"));
        assert_eq!("id name Ladybug 0.1.0", output_receiver.recv().unwrap());
        assert_eq!("id author Felix O.", output_receiver.recv().unwrap());
        assert_eq!("uciok", output_receiver.recv().unwrap());
    }

    #[test]
    fn test_ladybug_for_isready() {
        let (input_sender, output_receiver) = setup();
        
        let _ = input_sender.send(String::from("isready"));
        assert_eq!("readyok", output_receiver.recv().unwrap());
    }
}

