use chess_engine::lookup::LOOKUP_TABLE;
use chess_engine::lookup::lookup_table::LookupTable;

fn main() {
    // Initialize the lookup table
    let mut lookup_table = LookupTable::default();
    lookup_table.initialize_tables();
    LOOKUP_TABLE.set(lookup_table).expect("Could not write to OnceLock");
}
