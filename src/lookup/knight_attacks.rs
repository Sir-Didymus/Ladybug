use crate::board::bitboard::Bitboard;
use crate::board::square::{NUM_SQUARES, Square};
use crate::lookup::{NOT_A_B_FILES, NOT_A_FILE, NOT_G_H_FILES, NOT_H_FILE};
use crate::lookup::lookup_table::LookupTable;

impl LookupTable {
    /// Generates the knight attack table.
    pub(super) fn generate_knight_attacks() -> [Bitboard; 64] {
        let mut knight_attacks = [Bitboard::new(0); 64];
        for square_index in 0..NUM_SQUARES {
            knight_attacks[square_index as usize] = Self::get_knight_attack_bb(Square::new(square_index));
        }
        knight_attacks
    }

    /// Returns the attack bitboard for a knight on a specified square.
    /// See [Knight Attacks](https://www.chessprogramming.org/Knight_Pattern#Knight_Attacks)
    fn get_knight_attack_bb(square: Square) -> Bitboard {
        let mut attack_bb = Bitboard::new(0); // the result attack bitboard
        let knight_bb = Bitboard::from_square(square); // bitboard with the square of the knight set

        // North North-East - Offset +17
        if ((knight_bb.value << 17) & NOT_A_FILE.value) > 0 {
            attack_bb.value |= knight_bb.value << 17;
        }
        // North North-West - Offset +15
        if ((knight_bb.value << 15) & NOT_H_FILE.value) > 0 {
            attack_bb.value |= knight_bb.value << 15;
        }
        // North East-East - Offset +10
        if ((knight_bb.value << 10) & NOT_A_B_FILES.value) > 0 {
            attack_bb.value |= knight_bb.value << 10;
        }
        // North West-East - Offset +6
        if ((knight_bb.value << 6) & NOT_G_H_FILES.value) > 0 {
            attack_bb.value |= knight_bb.value << 6;
        }
        // South East-East - Offset -6
        if ((knight_bb.value >> 6) & NOT_A_B_FILES.value) > 0 {
            attack_bb.value |= knight_bb.value >> 6;
        }
        // South West-West - Offset -10
        if ((knight_bb.value >> 10) & NOT_G_H_FILES.value) > 0 {
            attack_bb.value |= knight_bb.value >> 10;
        }
        // South South-East - Offset -15
        if ((knight_bb.value >> 15) & NOT_A_FILE.value) > 0 {
            attack_bb.value |= knight_bb.value >> 15;
        }
        // South South-West - Offset -17
        if ((knight_bb.value >> 17) & NOT_H_FILE.value) > 0 {
            attack_bb.value |= knight_bb.value >> 17;
        }

        attack_bb
    }
}

#[cfg(test)]
mod tests {
    use crate::board::square::{A1, A8, B3, C7, D8, E4, F6, G1, H1, H8, NUM_SQUARES, Square};
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn get_knight_attack_bb_returns_bitboard_with_attacked_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = LookupTable::get_knight_attack_bb(Square::new(square_index));
            // print attack_bb for debugging purposes
            println!("{attack_bb}");
        }

        // If I wanted to test for every square like I did for pawns, I'd basically have to rewrite the get_attack_bb function in here again, just for this test.
        // So I will only test for a few exemplary cases (using hard-coded hex values for the result bitboard). 
        // The correctness of the function can also be verified by checking the console output (see println! above).
        // You can see the knight's attack squares shifting up the bitboard, with over-the-edge captures accounted for by using the file masks.

        assert_eq!(0x284400442800, LookupTable::get_knight_attack_bb(E4).value);
        assert_eq!(0x20400, LookupTable::get_knight_attack_bb(A1).value);
        assert_eq!(0x402000, LookupTable::get_knight_attack_bb(H1).value);
        assert_eq!(0x4020000000000, LookupTable::get_knight_attack_bb(A8).value);
        assert_eq!(0x20400000000000, LookupTable::get_knight_attack_bb(H8).value);
        assert_eq!(0x22140000000000, LookupTable::get_knight_attack_bb(D8).value);
        assert_eq!(0xa01000, LookupTable::get_knight_attack_bb(G1).value);
        assert_eq!(0x508000805, LookupTable::get_knight_attack_bb(B3).value);
        assert_eq!(0x5088008850000000, LookupTable::get_knight_attack_bb(F6).value);
        assert_eq!(0x1100110a00000000, LookupTable::get_knight_attack_bb(C7).value);
    }
}