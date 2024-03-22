use crate::bitboard::Bitboard;
use crate::color::Color;
use crate::square::Square;

/// A bitboard with all bits set to 1, except for those on the A file.
const NOT_A_FILE: Bitboard = Bitboard { value: 0xfefefefefefefefe };

/// A bitboard with all bits set to 1, except for those on the H file.
const NOT_H_FILE: Bitboard = Bitboard { value: 0x7f7f7f7f7f7f7f7f };

/// Generates the pawn attack table.
pub fn generate_pawn_attacks() -> [[Bitboard; 64]; 2] {
    let pawn_attacks = [[Bitboard::new(0); 64]; 2];
    pawn_attacks
}

/// Returns the attack bitboard for the pawn of the specified color on the specified square.
fn get_attack_bb(square: Square, color: Color) -> Bitboard {
    let mut attack_bb = Bitboard::new(0); // the result attack bitboard
    let pawn_bb = Bitboard::from_square(square); // bitboard with the square of the pawn set

    match color {
        Color::White => {
            // Shift bitboards by offsets to get the attack map for the square.
            // Filter out over the edge captures with NOT_A_FILE and NOT_H_FILE masks.
            if ((pawn_bb.value << 7) & NOT_H_FILE.value) > 0 {
                attack_bb.value |= pawn_bb.value << 7; // Left target square
            }
            if ((pawn_bb.value << 9) & NOT_A_FILE.value) > 0 {
                attack_bb.value |= pawn_bb.value << 9; // Right target square
            }
        }
        Color::Black => {
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

#[cfg(test)]
mod tests {
    use crate::bitboard::Bitboard;
    use crate::color::Color::{Black, White};
    use crate::file::{File, NUM_FILES};
    use crate::pawn_attacks::{get_attack_bb, NOT_A_FILE, NOT_H_FILE};
    use crate::rank::{NUM_RANKS, Rank};
    use crate::square::{NUM_SQUARES, Square};

    #[test]
    fn not_a_file_squares_on_a_file_not_set() {
        // verify that A file bits are not set
        for rank_index in Rank::First.to_index()..NUM_RANKS {
            assert!(!NOT_A_FILE.get_bit(Square::from_file_rank(File::A, Rank::from_index(rank_index))));
        }

        // verify that all other bits are set
        for file_index in File::B.to_index()..NUM_FILES {
            for rank_index in 0..NUM_RANKS {
                assert!(NOT_A_FILE.get_bit(Square::from_file_rank(File::from_index(file_index), Rank::from_index(rank_index))));
            }
        }
    }

    #[test]
    fn not_h_file_squares_on_h_file_not_set() {
        // verify that H file bits are not set
        for rank_index in Rank::First.to_index()..NUM_RANKS {
            assert!(!NOT_H_FILE.get_bit(Square::from_file_rank(File::H, Rank::from_index(rank_index))));
        }

        // verify that all other bits are set
        for file_index in File::A.to_index()..(NUM_FILES - 1) {
            for rank_index in 0..NUM_RANKS {
                assert!(NOT_H_FILE.get_bit(Square::from_file_rank(File::from_index(file_index), Rank::from_index(rank_index))));
            }
        }
    }

    #[test]
    fn get_attack_bb_for_white_returns_bitboard_with_attacked_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = get_attack_bb(Square::new(square_index), White);
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
    fn get_attack_bb_for_black_returns_bitboard_with_attacked_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = get_attack_bb(Square::new(square_index), Black);
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