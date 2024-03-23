use crate::board::bitboard::Bitboard;
use crate::lookup::pawn_attacks;

/// This is the lookup table for the move generator.
pub struct LookupTable {
    pawn_attacks: [[Bitboard; 64]; 2],
}

impl LookupTable {
    /// Initializes the lookup tables for all pieces.
    pub fn initialize_tables(&mut self) {
        self.pawn_attacks = pawn_attacks::generate_pawn_attacks();
    }
}
