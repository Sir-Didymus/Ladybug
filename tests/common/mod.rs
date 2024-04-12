use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use ladybug::ladybug::{Ladybug, Message};
use ladybug::lookup::LOOKUP_TABLE;
use ladybug::lookup::lookup_table::LookupTable;
use ladybug::search::{Search, SearchCommand};

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
    thread::spawn(move || ladybug.run());

    (message_sender, output_receiver)
}