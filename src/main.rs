use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{io, thread};
use ladybug::ladybug::{Ladybug, Message};
use ladybug::lookup::LOOKUP_TABLE;
use ladybug::lookup::lookup_table::LookupTable;
use ladybug::search::{Search, SearchCommand};

/// Initializes the lookup table, spawns the input and output threads, and starts running Ladybug.
fn main() {
    println!("\nLadybug 0.1.0\n");

    print!("Initializing tables... ");

    // initialize the lookup table
    let mut lookup = LookupTable::default();
    lookup.initialize_tables();
    let _ = LOOKUP_TABLE.set(lookup);

    println!("Done!");

    println!("Type \"help\" to see a list of all commands.\n");

    // create search_command_sender and search_command_receiver so that the ladybug thread can send commands to the search thread
    let (search_command_sender, search_command_receiver): (Sender<SearchCommand>, Receiver<SearchCommand>) = mpsc::channel();

    // create message_sender and message_receiver so that the input and search threads can send messages to the ladybug thread
    let (message_sender, message_receiver) : (Sender<Message>, Receiver<Message>) = mpsc::channel();

    // create output_sender and output_receiver so that the ladybug thread can send output to the output thread.
    let (output_sender, output_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();
    
    // make a copy of the message_sender for the input thread
    let message_sender_copy = message_sender.clone();
    
    // spawn the input thread
    let _ = thread::Builder::new().name("console_in".to_string()).spawn(move || read_input(message_sender_copy));

    // spawn the output thread
    let _ = thread::Builder::new().name("console_out".to_string()).spawn(move || write_output(output_receiver));
    
    // initialize the search
    let mut search = Search::new(search_command_receiver, message_sender);
    
    // spawn the search thread
    let _ = thread::Builder::new().name("search".to_string()).spawn(move || search.run());

    // initialize Ladybug
    let mut ladybug = Ladybug::new(search_command_sender, output_sender, message_receiver);
    
    // start running Ladybug
    ladybug.run();
}

/// Reads input from Stdin and sends it to Ladybug.
pub fn read_input(sender: Sender<Message>) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // try to send the input to Ladybug
        let result = sender.send(Message::ConsoleMessage(input));
        
        if result.is_err() {
            // the Ladybug thread was terminated, terminate the input thread
            return;
        }
    }
}

/// Receives output from Ladybug and writes it to Stdout.
pub fn write_output(receiver: Receiver<String>) {
    loop {
        let output = receiver.recv().unwrap();
        
        // if the output thread receives "quit", terminate it
        if output == "quit" {
            return;
        }

        // print Ladybug's output
        println!("{}", output);
    }
}
