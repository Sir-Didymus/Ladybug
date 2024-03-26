use crate::board::bitboard::Bitboard;
use crate::lookup::{king_attacks, knight_attacks};
use crate::lookup::pawn_attacks;

/// This is the lookup table for the move generator.
pub struct LookupTable {
    pawn_attacks: [[Bitboard; 64]; 2],
    knight_attacks: [Bitboard; 64],
    king_attacks: [Bitboard; 64],
}

impl Default for LookupTable {
    /// Default constructor for LookupTable.
    /// Make sure to call `initialize_tables` before using this instance.
    fn default() -> Self {
        LookupTable {
            pawn_attacks: [[Bitboard::new(0); 64]; 2],
            knight_attacks: [Bitboard::new(0); 64],
            king_attacks: [Bitboard::new(0); 64],
        }
    }
}

impl LookupTable {
    /// Initializes the lookup tables for all pieces.
    pub fn initialize_tables(&mut self) {
        self.pawn_attacks = pawn_attacks::generate_pawn_attacks();
        self.knight_attacks = knight_attacks::generate_knight_attacks();
        self.king_attacks = king_attacks::generate_king_attacks();
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn default_returns_lookup_table_with_empty_bitboards() {
        let lookup_table = LookupTable::default();
        assert_eq!([[Bitboard::new(0); 64]; 2], lookup_table.pawn_attacks);
        assert_eq!([Bitboard::new(0); 64], lookup_table.knight_attacks);
        assert_eq!([Bitboard::new(0); 64], lookup_table.king_attacks);
    }
}
