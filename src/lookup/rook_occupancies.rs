use crate::board::bitboard::Bitboard;
use crate::board::file::File;
use crate::board::rank::Rank;
use crate::board::square::{NUM_SQUARES, Square};
use crate::lookup::lookup_table::LookupTable;

impl LookupTable {
    /// Generates the rook occupancies table.
    pub(super) fn generate_rook_occupancies() -> [Bitboard; 64] {
        let mut rook_occupancies = [Bitboard::new(0); 64];
        for square_index in 0..NUM_SQUARES {
            rook_occupancies[square_index as usize] = Self::get_rook_occupancy_bb(Square::new(square_index));
        }
        rook_occupancies
    }

    /// Returns the occupancy bitboard for a rook on a specified square.
    fn get_rook_occupancy_bb(square: Square) -> Bitboard {
        // the result occupancy bitboard
        let mut occupancy_bb = Bitboard::new(0);

        // file on which the rook resides
        let rook_file = square.get_file();

        // rank on which the rook resides
        let rook_rank = square.get_rank();

        // -------------------------------------------------------------------------------------------
        // generate occupancies North
        // -------------------------------------------------------------------------------------------

        // If rook is on rank 8, can't go further North.
        // Also check that rook is not on file A or H, since the edges are irrelevant for the occupancy.
        if (rook_rank != Rank::Eighth) && (rook_file != File::A) && (rook_file != File::H) {
            // rank to iterate over the target ranks
            let mut target_rank = rook_rank.up();

            // iterate in direction North
            while target_rank != Rank::Eighth {
                occupancy_bb.set_bit(Square::from_file_rank(rook_file, target_rank));
                target_rank = target_rank.up();
            }
        }

        // -------------------------------------------------------------------------------------------
        // generate occupancies South
        // -------------------------------------------------------------------------------------------

        // If rook is on rank 1, can't go further South.
        // Also check that rook is not on file A or H, since the edges are irrelevant for the occupancy.
        if (rook_rank != Rank::First) && (rook_file != File::A) && (rook_file != File::H) {
            // rank to iterate over the target ranks
            let mut target_rank = rook_rank.down();

            // iterate in direction South
            while target_rank != Rank::First {
                occupancy_bb.set_bit(Square::from_file_rank(rook_file, target_rank));
                target_rank = target_rank.down();
            }
        }

        // -------------------------------------------------------------------------------------------
        // generate occupancies East
        // -------------------------------------------------------------------------------------------

        // If rook is on file H, can't go further East.
        // Also check that rook is not on rank 1 or 8, since the edges are irrelevant for the occupancy.
        if (rook_file != File::H) && (rook_rank != Rank::First) && (rook_rank != Rank::Eighth) {
            // rank to iterate over the target files
            let mut target_file = rook_file.right();

            // iterate in direction East
            while target_file != File::H {
                occupancy_bb.set_bit(Square::from_file_rank(target_file, rook_rank));
                target_file = target_file.right();
            }
        }

        // -------------------------------------------------------------------------------------------
        // generate occupancies West
        // -------------------------------------------------------------------------------------------

        // If rook is on file A, can't go further West.
        // Also check that rook is not on rank 1 or 8, since the edges are irrelevant for the occupancy.
        if (rook_file != File::A) && (rook_rank != Rank::First) && (rook_rank != Rank::Eighth) {
            // rank to iterate over the target files
            let mut target_file = rook_file.left();

            // iterate in direction East
            while target_file != File::A {
                occupancy_bb.set_bit(Square::from_file_rank(target_file, rook_rank));
                target_file = target_file.left();
            }
        }

        occupancy_bb
    }
}

#[cfg(test)]
mod tests {
    use crate::board::file::{File, NUM_FILES};
    use crate::board::rank::{NUM_RANKS, Rank};
    use crate::board::square::{A1, A8, B3, C7, D8, E4, F6, G1, H1, H8, NUM_SQUARES, Square};
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn get_rook_occupancy_bb_returns_bitboard_with_relevant_occupancy_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get occupancies for square
            let occupancy_bb = LookupTable::get_rook_occupancy_bb(Square::new(square_index));
            // print occupancy_bb for debugging purposes
            println!("{occupancy_bb}");
        }

        // If I wanted to test for every square like I did for pawns, I'd basically have to rewrite the get_occupancy_bb function in here again, just for this test.
        // So I will only test for a few exemplary cases (using hard-coded hex values for the result bitboard). 
        // The correctness of the function can also be verified by checking the console output (see println! above).
        // You can see the rook's occupancy squares shifting up the bitboard. Notice that file A, file H, rank 1, and rank 8, are not relevant for
        // the rook occupancies and are thus not set to 1.

        assert_eq!(0x1010106e101000, LookupTable::get_rook_occupancy_bb(E4).value);
        assert_eq!(0x0, LookupTable::get_rook_occupancy_bb(A1).value);
        assert_eq!(0x0, LookupTable::get_rook_occupancy_bb(H1).value);
        assert_eq!(0x0, LookupTable::get_rook_occupancy_bb(A8).value);
        assert_eq!(0x0, LookupTable::get_rook_occupancy_bb(H8).value);
        assert_eq!(0x8080808080800, LookupTable::get_rook_occupancy_bb(D8).value);
        assert_eq!(0x40404040404000, LookupTable::get_rook_occupancy_bb(G1).value);
        assert_eq!(0x20202027c0200, LookupTable::get_rook_occupancy_bb(B3).value);
        assert_eq!(0x205e2020202000, LookupTable::get_rook_occupancy_bb(F6).value);
        assert_eq!(0x7a040404040400, LookupTable::get_rook_occupancy_bb(C7).value);
    }

    #[test]
    fn get_rook_occupancy_bb_returns_bitboard_with_a_h_files_and_first_eighth_rank_and_rook_square_unset() {
        for square_index in 0..NUM_SQUARES {
            // get occupancies for square
            let occupancy_bb = LookupTable::get_rook_occupancy_bb(Square::new(square_index));

            // assert that rook square is unset
            assert!(!occupancy_bb.get_bit(Square::new(square_index)));

            // assert that A and H file bits are unset
            for rank_index in 0..NUM_RANKS {
                assert!(!occupancy_bb.get_bit(Square::from_file_rank(File::A, Rank::from_index(rank_index))));
                assert!(!occupancy_bb.get_bit(Square::from_file_rank(File::H, Rank::from_index(rank_index))));
            }

            // assert that first and eighth rank bits are unset
            for file_index in 0..NUM_FILES {
                assert!(!occupancy_bb.get_bit(Square::from_file_rank(File::from_index(file_index), Rank::First)));
                assert!(!occupancy_bb.get_bit(Square::from_file_rank(File::from_index(file_index), Rank::Eighth)));
            }
        }
    }
}