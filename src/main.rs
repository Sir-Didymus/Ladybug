use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::{io, thread};
use ladybug::ladybug::Ladybug;
use ladybug::lookup::LOOKUP_TABLE;
use ladybug::lookup::lookup_table::LookupTable;

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

    // create input_sender and input_receiver so that the input thread can send input to the ladybug thread
    let (input_sender, input_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();

    // create output_sender and output_receiver so that the ladybug thread can send output to the output thread.
    let (output_sender, output_receiver) : (Sender<String>, Receiver<String>) = mpsc::channel();
    
    // spawn the input thread
    thread::spawn(move || read_input(input_sender));

    // spawn the output thread
    thread::spawn(move || write_output(output_receiver));

    // initialize Ladybug
    let mut ladybug = Ladybug::default();
    
    // start running Ladybug
    ladybug.run(output_sender, input_receiver);
}

/// Reads input from Stdin and sends it to Ladybug.
pub fn read_input(sender: Sender<String>) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // try to send the input to Ladybug
        let result = sender.send(input);
        
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
