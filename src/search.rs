use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};
use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::square;
use crate::ladybug::Message;
use crate::move_gen::generates_moves;
use crate::move_gen::ply::Ply;

pub mod perft;
pub mod negamax;

/// Encodes the commands the search can receive from Ladybug.
pub enum SearchCommand {
    /// Search the given position for the given amount of milliseconds.
    SearchTime(Position, u64),
    /// Search the given position until the given depth is reached.
    SearchDepth(Position, u64),
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
    /// The number of nodes traversed during the search.
    node_count: u64,
    /// The current best move found during search.
    best_move: Option<Ply>,
    /// Used to measure the expired time during search.
    instant: Option<Instant>,
    /// Stores the lengths of the principe variations.
    pv_length: [u8; 64],
    /// Stores the principle variations.
    pv_table: [[Ply; 64]; 64],
}

impl Search {
    /// Constructs a new search instance.
    pub fn new(input_receiver: Receiver<SearchCommand>, output_sender: Sender<Message>) -> Self {
        Self {
            command_receiver: input_receiver,
            message_sender: output_sender,
            node_count: 0,
            best_move: None,
            instant: None,
            pv_length: [0; 64],
            // initialize the pv table with null moves (a1 to a1)
            pv_table: [[Ply {
                source: square::A1,
                target: square::A1,
                piece: Piece::Pawn,
                captured_piece: None,
                promotion_piece: None,
            }; 64];64]
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
                SearchCommand::SearchTime(position, time) => self.handle_search(position, None, Some(time)),
                SearchCommand::SearchDepth(position, depth) => self.handle_search(position, Some(depth), None),
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

    /// Handles the various "Search" commands.
    fn handle_search(&mut self, position: Position, depth_limit: Option<u64>, time_limit: Option<u64>) {
        let moves = generates_moves(position);
        if moves.is_empty() {
            self.send_output(String::from("info string no legal moves"));
            return;
        }

        // check if a depth value was provided, if not, use a default depth limit of 100
        let depth_limit = depth_limit.unwrap_or(100);

        // check if a time limit was provided
        let time_limit = match time_limit {
            // if no time limit ws provided, use a default limit of 72 hours
            None => Duration::from_secs(72 * 60 * 60),
            Some(time) => Duration::from_millis(time),
        };

        self.iterative_search(position, depth_limit, time_limit);
    }
    
    /// Handles the "Perft" command.
    fn handle_perft(&self, position: Position, depth: u64) {
        self.perft(position, depth);
    }
}
