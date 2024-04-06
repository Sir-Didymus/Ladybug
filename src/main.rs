use chess_engine::board::Board;
use chess_engine::lookup::LOOKUP_TABLE;
use chess_engine::lookup::lookup_table::LookupTable;
use chess_engine::move_gen::perft::perft;

fn main() {
    let mut lookup = LookupTable::default();
    lookup.initialize_tables();
    let _ = LOOKUP_TABLE.set(lookup);

    let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
    perft(position, 6);
}
