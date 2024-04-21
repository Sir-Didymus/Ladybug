use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use ladybug::ladybug::{Ladybug, Message};
use ladybug::lookup::LOOKUP_TABLE;
use ladybug::lookup::lookup_table::LookupTable;
use ladybug::search::{Search, SearchCommand};

/// Helper function to assert that the engine returns the expected output after reaching the given depth.
/// 
/// This function makes the tests cleaner and easier to read, since we want to discard all the "info depth..." messages and
/// are only interested in the final result.
pub fn assert_result(receiver: &Receiver<String>, depth: u8 , expected: &str) {
    loop {
        let output = receiver.recv().unwrap();
        if output.contains(format!("info depth {depth}").as_str()) {
            assert!(receiver.recv().unwrap().contains(expected));
            break;
        }
    }
}

/// Helper function to send the given fen string to Ladybug and tell her to search to the given depth.
pub fn go_position(sender: &Sender<Message>, fen: &str, depth: u8) {
    let go_command = format!("position fen {fen}");
    let _ = sender.send(Message::ConsoleMessage(go_command));
    let _ = sender.send(Message::ConsoleMessage(format!("go depth {depth}")));
}

/// Helper function to initialize and spawn the main and search threads, just like in the main function, but return the sender and receiver 
/// to the test function instead of creating dedicated input and output threads. The test thread will act as both input and output thread,
/// and is thus able to properly test Ladybug's output for various input.
pub fn setup() -> (Sender<Message>, Receiver<String>) {
    // initialize the lookup table
    let mut lookup = LookupTable::default();
    lookup.initialize_tables();
    let _ = LOOKUP_TABLE.set(lookup);

    // create search_command_sender and search_command_receiver so that the ladybug thread can send commands to the search thread
    let (search_command_sender, search_command_receiver): (Sender<SearchCommand>, Receiver<SearchCommand>) = mpsc::channel();

    // create message_sender and message_receiver so that the test and search threads can send messages to the ladybug thread
    let (message_sender, message_receiver) : (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // create output_sender and output_receiver so that the ladybug thread can send output to the test thread.
    let (output_sender, output_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

    // initialize the search
    let mut search = Search::new(search_command_receiver, message_sender.clone());

    // spawn the search thread
    let _ = thread::Builder::new().name("search".to_string()).spawn(move || search.run());

    // initialize Ladybug
    let mut ladybug = Ladybug::new(search_command_sender, output_sender.clone(), message_receiver);

    // spawn the Ladybug thread
    let _ = thread::Builder::new().name("ladybug".to_string()).spawn(move || ladybug.run());

    (message_sender, output_receiver)
}