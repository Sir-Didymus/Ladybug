use crate::board::bitboard::Bitboard;
use crate::board::color::Color;
use crate::board::color::Color::{Black, White};
use crate::board::square::{NUM_SQUARES, Square};
use crate::lookup::{NOT_A_FILE, NOT_H_FILE};
use crate::lookup::lookup_table::LookupTable;

impl LookupTable {
    /// Generates the pawn attack table.
    pub(super) fn generate_pawn_attacks() -> [[Bitboard; 64]; 2] {
        let mut pawn_attacks = [[Bitboard::new(0); 64]; 2];
        for square_index in 0..NUM_SQUARES {
            pawn_attacks[0][square_index as usize] = Self::get_pawn_attack_bb(Square::new(square_index), White);
            pawn_attacks[1][square_index as usize] = Self::get_pawn_attack_bb(Square::new(square_index), Black);
        }
        pawn_attacks
    }

    /// Returns the attack bitboard for a pawn of a specified color on a specified square.
    fn get_pawn_attack_bb(square: Square, color: Color) -> Bitboard {
        let mut attack_bb = Bitboard::new(0); // the result attack bitboard
        let pawn_bb = Bitboard::from_square(square); // bitboard with the square of the pawn set

        match color {
            White => {
                // Shift bitboards by offsets to get the attack map for the square.
                // Filter out over the edge captures with NOT_A_FILE and NOT_H_FILE masks.
                if ((pawn_bb.value << 7) & NOT_H_FILE.value) > 0 {
                    attack_bb.value |= pawn_bb.value << 7; // Left target square
                }
                if ((pawn_bb.value << 9) & NOT_A_FILE.value) > 0 {
                    attack_bb.value |= pawn_bb.value << 9; // Right target square
                }
            }
            Black => {
                // Reversed offsets and NOT_FILE constants for black
                if ((pawn_bb.value >> 9) & NOT_H_FILE.value) > 0 {
                    attack_bb.value |= pawn_bb.value >> 9; // Left target square
                }
                if ((pawn_bb.value >> 7) & NOT_A_FILE.value) > 0 {
                    attack_bb.value |= pawn_bb.value >> 7; // Right target square
                }
            }
        }

        attack_bb
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::color::Color::{Black, White};
    use crate::board::file::{File};
    use crate::board::rank::{Rank};
    use crate::board::square::{NUM_SQUARES, Square};
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn generate_pawn_attacks_returns_array_with_correct_sizes() {
        assert_eq!(2, LookupTable::generate_pawn_attacks().len());
        assert_eq!(64, LookupTable::generate_pawn_attacks()[0].len());
        assert_eq!(64, LookupTable::generate_pawn_attacks()[1].len());
    }

    #[test]
    fn get_pawn_attack_bb_for_white_returns_bitboard_with_attacked_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = LookupTable::get_pawn_attack_bb(Square::new(square_index), White);
            // print attack_bb for debugging purposes
            println!("{attack_bb}");
            // No white pawns on rank 8 allowed (promotion)
            if Square::new(square_index).get_rank() == Rank::Eighth {
                assert_eq!(0, attack_bb.value);
            }
            // if on file A, result map must contain only one pawn on file B
            else if Square::new(square_index).get_file() == File::A {
                let rank = Square::new(square_index).get_rank().up(); // rank of the pawn target square
                let file = Square::new(square_index).get_file().right(); // file of the pawn target square
                assert_eq!(Bitboard::from_square(Square::from_file_rank(file, rank)).value, attack_bb.value)
            }
            // if on file H, result map must contain only one pawn on file G
            else if Square::new(square_index).get_file() == File::H {
                let rank = Square::new(square_index).get_rank().up(); // rank of the pawn target square
                let file = Square::new(square_index).get_file().left(); // file of the pawn target square
                assert_eq!(Bitboard::from_square(Square::from_file_rank(file, rank)).value, attack_bb.value)
            }
            // the rest are the non-special cases
            else {
                let rank = Square::new(square_index).get_rank().up(); // rank of the pawn target square
                let file_left = Square::new(square_index).get_file().left(); // file of the pawn target square
                let file_right = Square::new(square_index).get_file().right(); // file of the pawn target square
                let expected_bb_value = Bitboard::from_square(Square::from_file_rank(file_left, rank)).value | Bitboard::from_square(Square::from_file_rank(file_right, rank)).value;
                assert_eq!(expected_bb_value, attack_bb.value)
            }
        }
    }

    #[test]
    fn get_pawn_attack_bb_for_black_returns_bitboard_with_attacked_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = LookupTable::get_pawn_attack_bb(Square::new(square_index), Black);
            // print attack_bb for debugging purposes
            println!("{attack_bb}");
            // No black pawns on rank 1 allowed (promotion)
            if Square::new(square_index).get_rank() == Rank::First {
                assert_eq!(0, attack_bb.value);
            }
            // if on file A, result map must contain only one pawn on file B
            else if Square::new(square_index).get_file() == File::A {
                let rank = Square::new(square_index).get_rank().down(); // rank of the pawn target square
                let file = Square::new(square_index).get_file().right(); // file of the pawn target square
                assert_eq!(Bitboard::from_square(Square::from_file_rank(file, rank)).value, attack_bb.value)
            }
            // if on file H, result map must contain only one pawn on file G
            else if Square::new(square_index).get_file() == File::H {
                let rank = Square::new(square_index).get_rank().down(); // rank of the pawn target square
                let file = Square::new(square_index).get_file().left(); // file of the pawn target square
                assert_eq!(Bitboard::from_square(Square::from_file_rank(file, rank)).value, attack_bb.value)
            }
            // the rest are the non-special cases
            else {
                let rank = Square::new(square_index).get_rank().down(); // rank of the pawn target square
                let file_left = Square::new(square_index).get_file().left(); // file of the pawn target square
                let file_right = Square::new(square_index).get_file().right(); // file of the pawn target square
                let expected_bb_value = Bitboard::from_square(Square::from_file_rank(file_left, rank)).value | Bitboard::from_square(Square::from_file_rank(file_right, rank)).value;
                assert_eq!(expected_bb_value, attack_bb.value)
            }
        }
    }
}