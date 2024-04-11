use std::sync::mpsc::{Receiver, Sender};
use crate::board::position::Position;
use crate::ladybug::Message;
use crate::move_gen::generates_moves;
use rand::Rng;

pub mod perft;

/// Encodes the commands the search can receive from Ladybug.
pub enum SearchCommand {
    /// Search the given position for the given amount of milliseconds.
    SearchTime(Position, u64),
    /// Perform a perft for the given position up to the specified depth.
    Perft(Position, u64),
    /// Stop the search immediately.
    Stop,
}

/// The search struct is responsible for performing all tasks involving calculation and search.
pub struct Search {
    /// Used to receive search commands from Ladybug.
    command_receiver: Receiver<SearchCommand>,
    /// Used to send search results to Ladybug.
    message_sender: Sender<Message>,
}

impl Search {
    /// Constructs a new search instance.
    pub fn new(input_receiver: Receiver<SearchCommand>, output_sender: Sender<Message>) -> Self {
        Self {
            command_receiver: input_receiver,
            message_sender: output_sender,
        }
    }

    /// Start accepting search commands from Ladybug.
    pub fn run(&mut self) {
        loop {
            // blocks until the search receives a command from Ladybug
            let input = self.command_receiver.recv();

            // if the main thread closes the connection, the search thread must not continue running
            if input.is_err() {
                return;
            }

            // get the input string from the result
            let command = input.unwrap();
            
            match command { 
                SearchCommand::Perft(position, depth) => self.handle_perft(position, depth),
                SearchCommand::SearchTime(position, time) => self.handle_search_time(position, time),
                _other => {},
            }
        }
    }

    /// Sends the given String to the main thread.
    fn send_output(&self, output: String) {
        let send_result = self.message_sender.send(Message::SearchMessage(output));

        // if the main thread closes the connection, the search thread must not continue running
        if send_result.is_err() {
            panic!("The main thread has unexpectedly closed the channel connection.")
        }
    }

    /// Handles the "SearchTime" command.
    fn handle_search_time(&self, position: Position, time: u64) {
        let moves = generates_moves(position);
        if moves.is_empty() {
            self.send_output(String::from("info string no legal moves"));
            return;
        }
        //thread::sleep(Duration::from_millis(time / 2));
        let move_index = rand::thread_rng().gen_range(0..moves.len());
        self.send_output(format!("bestmove {}", moves[move_index]))
    }
    
    /// Handles the "Perft" command.
    fn handle_perft(&self, position: Position, depth: u64) {
        self.perft(position, depth);
    }
}
