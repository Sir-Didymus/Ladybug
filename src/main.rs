use ladybug::ladybug::Ladybug;
use ladybug::lookup::LOOKUP_TABLE;
use ladybug::lookup::lookup_table::LookupTable;

fn main() {
    println!("\nLadybug 0.1.0\n");
    
    print!("Initializing tables... ");
    
    // initialize the lookup table
    let mut lookup = LookupTable::default();
    lookup.initialize_tables();
    let _ = LOOKUP_TABLE.set(lookup);

    println!("Done!");

    println!("Type \"help\" to see a list of all commands.\n");
    
    let ladybug = Ladybug::default();
}
