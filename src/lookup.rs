//! The lookup module is responsible for generating and storing the lookup tables used by the move generator.
//! The submodule `lookup_table` contains a struct to store these tables, while the generation logic
//! is provided by functions in submodules such as `pawn_attacks` or `knight_attacks`.

use std::sync::OnceLock;
use crate::board::bitboard::Bitboard;
use crate::lookup::lookup_table::LookupTable;

pub mod lookup_table;
pub mod pawn_attacks;
pub mod knight_attacks;
pub mod king_attacks;
pub mod bishop_occupancies;
pub mod rook_occupancies;
pub mod bishop_attacks;
pub mod rook_attacks;

/// This static `OnceLock` variable will contain the `LookupTable` instance.
/// Because calculating the lookup table is so expensive, it should only be done once.
/// 
/// The `OnceLock` type allows us to use its `set()` method to set the lookup table exactly once, and after that,
/// the lookup table can be obtained by calling the `get()` method.
pub static LOOKUP_TABLE: OnceLock<LookupTable> = OnceLock::new();

// ---------------------------------------------------------------
// Constants used to mask the attack bitboards for various pieces
// ---------------------------------------------------------------

/// A bitboard with all bits set to 1, except for those on the A file.
const NOT_A_FILE: Bitboard = Bitboard { value: 0xfefefefefefefefe };

/// A bitboard with all bits set to 1, except for those on the H file.
const NOT_H_FILE: Bitboard = Bitboard { value: 0x7f7f7f7f7f7f7f7f };

/// A bitboard with all bits set to 1, except for those on the A and B files.
const NOT_A_B_FILES: Bitboard = Bitboard { value: 0xfcfcfcfcfcfcfcfc };

/// A bitboard with all bits set to 1, except for those on the G and H files.
const NOT_G_H_FILES: Bitboard = Bitboard { value: 0x3f3f3f3f3f3f3f3f };

#[cfg(test)]
mod tests {
    use crate::board::file::{File, NUM_FILES};
    use crate::board::rank::{NUM_RANKS, Rank};
    use crate::board::square::Square;
    use crate::lookup::{NOT_A_B_FILES, NOT_A_FILE, NOT_G_H_FILES, NOT_H_FILE};

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
    fn not_a_b_files_squares_on_a_b_files_not_set() {
        // verify that A file bits are not set
        for rank_index in Rank::First.to_index()..NUM_RANKS {
            assert!(!NOT_A_B_FILES.get_bit(Square::from_file_rank(File::A, Rank::from_index(rank_index))));
        }
        // verify that B file bits are not set
        for rank_index in Rank::First.to_index()..NUM_RANKS {
            assert!(!NOT_A_B_FILES.get_bit(Square::from_file_rank(File::B, Rank::from_index(rank_index))));
        }
        // verify that all other bits are set
        for file_index in File::C.to_index()..NUM_FILES {
            for rank_index in 0..NUM_RANKS {
                assert!(NOT_A_B_FILES.get_bit(Square::from_file_rank(File::from_index(file_index), Rank::from_index(rank_index))));
            }
        }
    }

    #[test]
    fn not_g_h_files_squares_on_g_h_files_not_set() {
        // verify that G file bits are not set
        for rank_index in Rank::First.to_index()..NUM_RANKS {
            assert!(!NOT_G_H_FILES.get_bit(Square::from_file_rank(File::G, Rank::from_index(rank_index))));
        }
        // verify that H file bits are not set
        for rank_index in Rank::First.to_index()..NUM_RANKS {
            assert!(!NOT_G_H_FILES.get_bit(Square::from_file_rank(File::H, Rank::from_index(rank_index))));
        }
        // verify that all other bits are set
        for file_index in File::A.to_index()..NUM_FILES - 2 {
            for rank_index in 0..NUM_RANKS {
                assert!(NOT_G_H_FILES.get_bit(Square::from_file_rank(File::from_index(file_index), Rank::from_index(rank_index))));
            }
        }
    }
}
