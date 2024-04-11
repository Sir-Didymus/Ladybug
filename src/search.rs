use std::sync::mpsc::{Receiver, Sender};

/// Encodes the commands the search can receive from Ladybug.
pub enum SearchCommand {
    /// Perform a perft test up to the specified depth.
    Perft(u64),
    /// Stop the search immediately.
    Stop,
}

/// The search struct is responsible for performing all tasks involving calculation and search.
pub struct Search {
    /// Used to receive search commands from Ladybug.
    input_receiver: Receiver<SearchCommand>,
    /// Used to send search results to Ladybug.
    output_sender: Sender<String>,
}

impl Search {
    /// Constructs a new search instance.
    pub fn new(input_receiver: Receiver<SearchCommand>, output_sender: Sender<String>) -> Self {
        Self {
            input_receiver,
            output_sender,
        }
    }

    /// Start accepting search commands from Ladybug.
    pub fn run(&mut self) {
        loop {
            // blocks until the search receives a command from Ladybug
            let input = self.input_receiver.recv();

            // if the main thread closes the connection, the search thread must not continue running
            if input.is_err() {
                panic!("The main thread has unexpectedly closed the channel connection.")
            }

            // get the input string from the result
            let command = input.unwrap();
            
            //match command {
            //    Perft(depth) => 
           // }
        }
    }
    
    /// Handles the perft command.
    fn handle_perft() {
        
    }
}