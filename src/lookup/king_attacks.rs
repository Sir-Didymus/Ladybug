use crate::board::bitboard::Bitboard;
use crate::board::square::{NUM_SQUARES, Square};
use crate::lookup::{NOT_A_FILE, NOT_H_FILE};
use crate::lookup::lookup_table::LookupTable;

impl LookupTable {
    /// Generates the king attack table.
    pub(super) fn generate_king_attacks() -> [Bitboard; 64] {
        let mut king_attacks = [Bitboard::new(0); 64];
        for square_index in 0..NUM_SQUARES {
            king_attacks[square_index as usize] = Self::get_king_attack_bb(Square::new(square_index));
        }
        king_attacks
    }

    /// Returns the attack bitboard for a king on a specified square.
    fn get_king_attack_bb(square: Square) -> Bitboard {
        let mut attack_bb = Bitboard::new(0); // the result attack bitboard
        let king_bb = Bitboard::from_square(square); // bitboard with the square of the king set

        // North-East - Offset +9
        if ((king_bb.value << 9) & NOT_A_FILE.value) > 0 {
            attack_bb.value |= king_bb.value << 9;
        }
        // North - Offset +8
        if (king_bb.value << 8) > 0 {
            attack_bb.value |= king_bb.value << 8;
        }
        // North-West - Offset +7
        if ((king_bb.value << 7) & NOT_H_FILE.value) > 0 {
            attack_bb.value |= king_bb.value << 7;
        }
        // East - Offset +1
        if ((king_bb.value << 1) & NOT_A_FILE.value) > 0 {
            attack_bb.value |= king_bb.value << 1;
        }
        // West - Offset -1
        if ((king_bb.value >> 1) & NOT_H_FILE.value) > 0 {
            attack_bb.value |= king_bb.value >> 1;
        }
        // South-East - Offset -7
        if ((king_bb.value >> 7) & NOT_A_FILE.value) > 0 {
            attack_bb.value |= king_bb.value >> 7;
        }
        // South - Offset -8
        if (king_bb.value >> 8) > 0 {
            attack_bb.value |= king_bb.value >> 8;
        }
        // South-West - Offset -9
        if ((king_bb.value >> 9) & NOT_H_FILE.value) > 0 {
            attack_bb.value |= king_bb.value >> 9;
        }

        attack_bb
    }
}

#[cfg(test)]
mod tests {
    use crate::board::square::{A1, A8, B3, C7, D8, E4, F6, G1, H1, H8, NUM_SQUARES, Square};
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn get_king_attack_bb_returns_bitboard_with_attacked_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = LookupTable::get_king_attack_bb(Square::new(square_index));
            // print attack_bb for debugging purposes
            println!("{attack_bb}");
        }

        // If I wanted to test for every square like I did for pawns, I'd basically have to rewrite the get_attack_bb function in here again, just for this test.
        // So I will only test for a few exemplary cases (using hard-coded hex values for the result bitboard). 
        // The correctness of the function can also be verified by checking the console output (see println! above).
        // You can see the king's attack squares shifting up the bitboard, with over-the-edge captures accounted for by using the file masks.

        assert_eq!(0x3828380000, LookupTable::get_king_attack_bb(E4).value);
        assert_eq!(0x302, LookupTable::get_king_attack_bb(A1).value);
        assert_eq!(0xc040, LookupTable::get_king_attack_bb(H1).value);
        assert_eq!(0x203000000000000, LookupTable::get_king_attack_bb(A8).value);
        assert_eq!(0x40c0000000000000, LookupTable::get_king_attack_bb(H8).value);
        assert_eq!(0x141c000000000000, LookupTable::get_king_attack_bb(D8).value);
        assert_eq!(0xe0a0, LookupTable::get_king_attack_bb(G1).value);
        assert_eq!(0x7050700, LookupTable::get_king_attack_bb(B3).value);
        assert_eq!(0x70507000000000, LookupTable::get_king_attack_bb(F6).value);
        assert_eq!(0xe0a0e0000000000, LookupTable::get_king_attack_bb(C7).value);
    }
}