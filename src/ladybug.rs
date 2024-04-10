use std::sync::mpsc::{Receiver, Sender};
use crate::board::Board;
use crate::move_gen::ply::Ply;
use crate::uci;
use crate::uci::{UciCommand};

/// The main character in this project!
/// The Ladybug struct acts as the UCI client and can receive and handle UCI commands.
pub struct Ladybug {
    name: String,
    author: String,
    board: Board,
}

impl Default for Ladybug {
    /// Constructs Ladybug.
    fn default() -> Self {
        Self {
            name: String::from("Ladybug 0.1.0"),
            author: String::from("Felix O."),
            board: Board::default(),
        }
    }
}

impl Ladybug {
    /// Starts running Ladybug.
    pub fn run(&mut self, output_sender: Sender<String>, input_sender: Receiver<String>) {
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
                UciCommand::IsReady => self.handle_is_ready(&output_sender),
                UciCommand::Position(args) => self.handle_position(&output_sender, args),
                UciCommand::Quit => {
                    self.handle_quit(&output_sender);
                    break;
                }
                UciCommand::Help => self.handle_help(&output_sender),
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

    /// Handles the "position" command.
    fn handle_position(&mut self, output_sender: &Sender<String>, args: Vec<String>) {
        if args.is_empty() {
            Self::send_output(output_sender, String::from("info string unknown command"));
            return;
        }

        let mut fen = String::from("");

        // build the fen string from the provided args
        match args[0].as_str() {
            "startpos" => {
                fen += "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
            }
            "fen" => {
                for (index, arg) in args.iter().enumerate() {
                    if index == 0 {
                        continue;
                    }
                    if arg == "moves" {
                        break;
                    }
                    fen += " ";
                    fen += arg.as_str();
                }
            }
            _other => {
                Self::send_output(output_sender, String::from("info string unknown command"));
                return;
            }
        };

        // try to parse the fen
        let board = Board::from_fen(fen.as_str());
        if board.is_err() {
            Self::send_output(output_sender, String::from("info string invalid fen"));
            return;
        }
        let mut board = board.unwrap();

        // split the args vector to only contain the moves
        let moves_index = args.iter().position(|r| r == "moves");
        if moves_index.is_none() {
            // command contains no moves - finish
            self.board = board;
            return;
        }
        let moves_index = moves_index.unwrap() + 1;
        if moves_index > args.len() {
            return;
        }
        let (_, moves) = args.split_at(moves_index);

        // loop over moves strings and try to make the moves on the board
        for move_string in moves {
            let ply = Ply::from_string(move_string, board.position);
            match ply {
                Some(ply) => board.position = board.position.make_move(ply),
                None => {
                    Self::send_output(output_sender, String::from("info string invalid moves"));
                    return;
                }
            }
        }

        self.board = board;
    }

    /// Handles the "quit" command.
    fn handle_quit(&self, output_sender: &Sender<String>) {
        Self::send_output(output_sender, String::from("quit"));
    }

    /// Handles the "help" command.
    fn handle_help(&self, output_sender: &Sender<String>) {
        Self::send_output(output_sender, String::from("Ladybug is a free and UCI compatible chess engine."));
        Self::send_output(output_sender, String::from("Currently, Ladybug only implements a subset of the UCI protocol:"));
        Self::send_output(output_sender, String::from("uci                              : Ask Ladybug if she supports UCI"));
        Self::send_output(output_sender, String::from("isready                          : Synchronize Ladybug with the GUI"));
        Self::send_output(output_sender, String::from("quit                             : Quit Ladybug"));
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use crate::ladybug::Ladybug;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;

    /// Creates a new Ladybug thread and returns the input_sender and output_receiver.
    fn setup() -> (Sender<String>, Receiver<String>) {
        initialize_lookup_table();

        // create input_sender and input_receiver so that the input thread can send input to the ladybug thread
        let (input_sender, input_receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

        // create output_sender and output_receiver so that the ladybug thread can send output to the output thread.
        let (output_sender, output_receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

        let mut ladybug = Ladybug::default();

        // spawn the Ladybug thread
        thread::spawn(move || ladybug.run(output_sender, input_receiver));

        (input_sender, output_receiver)
    }

    // helper function to initialize the lookup table
    fn initialize_lookup_table() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);
    }

    #[test]
    fn test_ladybug_with_invalid_uci_input_prints_error_message() {
        let (input_sender, output_receiver) = setup();

        let _ = input_sender.send(String::from("Not Uci"));
        assert_eq!("info string unknown command", output_receiver.recv().unwrap());

        let _ = input_sender.send(String::from("       "));
        assert_eq!("info string unknown command", output_receiver.recv().unwrap());

        let _ = input_sender.send(String::from("123456789"));
        assert_eq!("info string unknown command", output_receiver.recv().unwrap());

        let _ = input_sender.send(String::from("position test"));
        assert_eq!("info string unknown command", output_receiver.recv().unwrap());

        let _ = input_sender.send(String::from("position fen this is invalid fen"));
        assert_eq!("info string invalid fen", output_receiver.recv().unwrap());
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

    #[test]
    fn test_ladybug_for_position() {
        let (input_sender, output_receiver) = setup();
        
        todo!("Test handle_position")
    }

    #[test]
    fn test_ladybug_for_quit() {
        let (input_sender, output_receiver) = setup();

        let _ = input_sender.send(String::from("quit"));
        assert_eq!("quit", output_receiver.recv().unwrap());
    }

    #[test]
    fn test_ladybug_for_help() {
        let (input_sender, output_receiver) = setup();

        let _ = input_sender.send(String::from("help"));
        assert_eq!("Ladybug is a free and UCI compatible chess engine.", output_receiver.recv().unwrap());
        assert_eq!("Currently, Ladybug only implements a subset of the UCI protocol:", output_receiver.recv().unwrap());
        assert_eq!("uci                              : Ask Ladybug if she supports UCI", output_receiver.recv().unwrap());
        assert_eq!("isready                          : Synchronize Ladybug with the GUI", output_receiver.recv().unwrap());
        assert_eq!("quit                             : Quit Ladybug", output_receiver.recv().unwrap());
    }
}

