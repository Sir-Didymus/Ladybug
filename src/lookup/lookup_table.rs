use crate::board::bitboard::Bitboard;
use crate::board::color::Color;
use crate::board::square::Square;

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
        self.pawn_attacks = Self::generate_pawn_attacks();
        self.knight_attacks = Self::generate_knight_attacks();
        self.king_attacks = Self::generate_king_attacks();
    }

    /// Returns the attack bitboard for a pawn of the specified color on the specified square.
    pub fn get_pawn_attacks(&self, square: Square, color: Color) -> Bitboard {
        self.pawn_attacks[color.to_index() as usize][square.index as usize]
    }

    /// Returns the attack bitboard for a knight on the specified square.
    pub fn get_knight_attacks(&self, square: Square) -> Bitboard {
        self.knight_attacks[square.index as usize]
    }

    /// Returns the attack bitboard for a bishop on the specified square and the specified blocker bitboard.
    pub fn get_bishop_attacks(&self, square: Square, blockers: Bitboard) -> Bitboard {
        Self::get_bishop_attack_bb(square, blockers)
    }

    /// Returns the attack bitboard for a rook on the specified square and the specified blocker bitboard.
    pub fn get_rook_attacks(&self, square: Square, blockers: Bitboard) -> Bitboard {
        Self::get_rook_attack_bb(square, blockers)
    }

    /// Returns the attack bitboard for a queen on the specified square and the specified blocker bitboard.
    pub fn get_queen_attacks(&self, square: Square, blockers: Bitboard) -> Bitboard {
        Bitboard::new(self.get_rook_attacks(square, blockers).value | self.get_bishop_attacks(square, blockers).value)
    }

    /// Returns the attack bitboard for a king of on the specified square.
    pub fn get_king_attacks(&self, square: Square) -> Bitboard {
        self.king_attacks[square.index as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::color::Color::{Black, White};
    use crate::board::square::{A5, B2, B5, B7, B8, C2, C3, C4, C7, D4, D8, E4, E5, F4, F7, F8, G2, G6, G7, H1, H5, H7, H8, NUM_SQUARES, Square};
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn default_returns_lookup_table_with_empty_bitboards() {
        let lookup_table = LookupTable::default();
        assert_eq!([[Bitboard::new(0); 64]; 2], lookup_table.pawn_attacks);
        assert_eq!([Bitboard::new(0); 64], lookup_table.knight_attacks);
        assert_eq!([Bitboard::new(0); 64], lookup_table.king_attacks);
    }

    #[test]
    fn get_pawn_attacks_returns_bitboard_with_attacked_bits_set() {
        let mut lookup_table = LookupTable::default();
        lookup_table.initialize_tables();

        // Testing the get_pawn_attacks method using fixed hex values for the result bitboard.
        assert_eq!(0xa0000, lookup_table.get_pawn_attacks(C2, White).value);
        assert_eq!(0x4000, lookup_table.get_pawn_attacks(H1, White).value);
        assert_eq!(0x50000000000, lookup_table.get_pawn_attacks(B5, White).value);
        assert_eq!(0x5000000000000000, lookup_table.get_pawn_attacks(F7, White).value);
        assert_eq!(0x400000000000, lookup_table.get_pawn_attacks(H5, White).value);
        assert_eq!(0xa, lookup_table.get_pawn_attacks(C2, Black).value);
        assert_eq!(0x0, lookup_table.get_pawn_attacks(H1, Black).value);
        assert_eq!(0x5000000, lookup_table.get_pawn_attacks(B5, Black).value);
        assert_eq!(0x500000000000, lookup_table.get_pawn_attacks(F7, Black).value);
        assert_eq!(0x40000000, lookup_table.get_pawn_attacks(H5, Black).value);
    }

    #[test]
    fn get_knight_attacks_returns_bitboard_with_attacked_bits_set() {
        let mut lookup_table = LookupTable::default();
        lookup_table.initialize_tables();

        // Testing the get_knight_attacks method using fixed hex values for the result bitboard.
        assert_eq!(0xa110011, lookup_table.get_knight_attacks(C2).value);
        assert_eq!(0x284400442800, lookup_table.get_knight_attacks(E4).value);
        assert_eq!(0x800080500000000, lookup_table.get_knight_attacks(B7).value);
        assert_eq!(0x402000, lookup_table.get_knight_attacks(H1).value);
        assert_eq!(0xa0100010a0000000, lookup_table.get_knight_attacks(G6).value);
        assert_eq!(0x22140000000000, lookup_table.get_knight_attacks(D8).value);
        assert_eq!(0x508800885000, lookup_table.get_knight_attacks(F4).value);
    }

    #[test]
    fn get_bishop_attacks_returns_bitboard_with_attacked_bits_set() {
        let mut lookup_table = LookupTable::default();
        lookup_table.initialize_tables();

        // Testing the get_bishop_attacks method using fixed hex values for the result and blocker bitboards.
        assert_eq!(0x8041221400142241, lookup_table.get_bishop_attacks(D4, Bitboard::new(0)).value);
        assert_eq!(0x40201008040201, lookup_table.get_bishop_attacks(H8, Bitboard::new(0)).value);
        assert_eq!(0xa000a01008040201, lookup_table.get_bishop_attacks(G7, Bitboard::new(0)).value);
        assert_eq!(0x810204000, lookup_table.get_bishop_attacks(H1, Bitboard::new(0x800000000)).value);
        assert_eq!(0x182442800284482, lookup_table.get_bishop_attacks(E4, Bitboard::new(0)).value);
        assert_eq!(0x182442800284482, lookup_table.get_bishop_attacks(E4, Bitboard::new(0x180000000000082)).value);
        assert_eq!(0x2800280000, lookup_table.get_bishop_attacks(E4, Bitboard::new(0x2800280000)).value);
        assert_eq!(0x102442800280400, lookup_table.get_bishop_attacks(E4, Bitboard::new(0x100400000200400)).value);
        assert_eq!(0x20100a000a1020, lookup_table.get_bishop_attacks(C4, Bitboard::new(0x20000200020020)).value);
        assert_eq!(0x14224180000000, lookup_table.get_bishop_attacks(D8, Bitboard::new(0)).value);
        assert_eq!(0x14220100000000, lookup_table.get_bishop_attacks(D8, Bitboard::new(0x200000000000)).value);
    }

    #[test]
    fn get_rook_attacks_returns_bitboard_with_attacked_bits_set() {
        let mut lookup_table = LookupTable::default();
        lookup_table.initialize_tables();

        // Testing the get_rook_attacks method using fixed hex values for the result and blocker bitboards.
        assert_eq!(0x101010ef10101010, lookup_table.get_rook_attacks(E5, Bitboard::new(0)).value);
        assert_eq!(0x101010ef10100000, lookup_table.get_rook_attacks(E5, Bitboard::new(0x1000008100100000)).value);
        assert_eq!(0x102810000000, lookup_table.get_rook_attacks(E5, Bitboard::new(0x102810000000)).value);
        assert_eq!(0x4040404fb040404, lookup_table.get_rook_attacks(C4, Bitboard::new(0)).value);
        assert_eq!(0x40404047b040404, lookup_table.get_rook_attacks(C4, Bitboard::new(0x400000041000004)).value);
        assert_eq!(0xdf20202020202020, lookup_table.get_rook_attacks(F8, Bitboard::new(0)).value);
        assert_eq!(0x5f20202020200000, lookup_table.get_rook_attacks(F8, Bitboard::new(0x4100000000200000)).value);
        assert_eq!(0x40404040bc40, lookup_table.get_rook_attacks(G2, Bitboard::new(0x400000008440)).value);
    }

    #[test]
    fn get_queen_attacks_returns_bitboard_with_attacked_bits_set() {
        let mut lookup_table = LookupTable::default();
        lookup_table.initialize_tables();

        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = lookup_table.get_queen_attacks(Square::new(square_index), Bitboard::new(0));
            // print attack_bb with empty blockers for debugging purposes
            println!("{attack_bb}");
        }

        // Testing the get_queen_attacks method using fixed hex values for the result and blocker bitboards.
        assert_eq!(0xefb0e1524448404, lookup_table.get_queen_attacks(C7, Bitboard::new(0)).value);
        assert_eq!(0xe0a0e0000000000, lookup_table.get_queen_attacks(C7, Bitboard::new(0xefb0e1524448404)).value);
        assert_eq!(0xefb0e1524040404, lookup_table.get_queen_attacks(C7, Bitboard::new(0x80000120000000)).value);
        assert_eq!(0x88492a1cf71c2a49, lookup_table.get_queen_attacks(D4, Bitboard::new(0)).value);
        assert_eq!(0x81c341c2848, lookup_table.get_queen_attacks(D4, Bitboard::new(0xefb0e1524448404)).value);
        assert_eq!(0x7fc0a09088848281, lookup_table.get_queen_attacks(H8, Bitboard::new(0)).value);
        assert_eq!(0x7fc0a09088808080, lookup_table.get_queen_attacks(H8, Bitboard::new(0x8000000)).value);
        assert_eq!(0x60c0a09088800000, lookup_table.get_queen_attacks(H8, Bitboard::new(0x2000000008800000)).value);
        assert_eq!(0x4142444850e0bfe0, lookup_table.get_queen_attacks(G2, Bitboard::new(0)).value);
        assert_eq!(0x4040404850e0bfe0, lookup_table.get_queen_attacks(G2, Bitboard::new(0x800000080)).value);
        assert_eq!(0x90503fe03050911, lookup_table.get_queen_attacks(A5, Bitboard::new(0)).value);
        assert_eq!(0x90503fe03050901, lookup_table.get_queen_attacks(A5, Bitboard::new(0x800)).value);
    }

    #[test]
    fn get_king_attacks_returns_bitboard_with_attacked_bits_set() {
        let mut lookup_table = LookupTable::default();
        lookup_table.initialize_tables();

        // Testing the get_king_attacks method using fixed hex values for the result bitboard.
        assert_eq!(0xc040c00000000000, lookup_table.get_king_attacks(H7).value);
        assert_eq!(0xe0a0e00, lookup_table.get_king_attacks(C3).value);
        assert_eq!(0x507000000000000, lookup_table.get_king_attacks(B8).value);
        assert_eq!(0x382838000000, lookup_table.get_king_attacks(E5).value);
        assert_eq!(0x70507, lookup_table.get_king_attacks(B2).value);
    }
}
