use crate::board::bitboard::Bitboard;
use crate::board::file::{File};
use crate::board::rank::{Rank};
use crate::board::square::{NUM_SQUARES, Square};

/// Generates the bishop occupancies table.
pub fn generate_bishop_occupancies() -> [Bitboard; 64] {
    let mut bishop_occupancies = [Bitboard::new(0); 64];
    for square_index in 0..NUM_SQUARES {
        bishop_occupancies[square_index as usize] = get_occupancy_bb(Square::new(square_index));
    }
    bishop_occupancies
}

/// Returns the occupancy bitboard for a bishop on a specified square.
fn get_occupancy_bb(square: Square) -> Bitboard {
    // the result occupancy bitboard
    let mut occupancy_bb = Bitboard::new(0);
    
    // file on which the bishop resides
    let bishop_file = square.get_file();
    
    // rank on which the bishop resides
    let bishop_rank = square.get_rank();
    
    // -------------------------------------------------------------------------------------------
    // generate occupancies North-East
    // -------------------------------------------------------------------------------------------
    
    // If bishop is on file H or rank 8, can't go further North-East
    if !((bishop_file == File::H) || (bishop_rank == Rank::Eighth)) {
        // file to iterate over the target files
        let mut target_file = bishop_file.right();
        // rank to iterate over the target ranks
        let mut target_rank = bishop_rank.up();

        // iterate in direction North-East
        while (target_file != File::H) && (target_rank != Rank::Eighth) {
            occupancy_bb.set_bit(Square::from_file_rank(target_file, target_rank));
            target_file = target_file.right();
            target_rank = target_rank.up();
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate occupancies North-West
    // -------------------------------------------------------------------------------------------

    // If bishop is on file A or rank 8, can't go further North-West
    if !((bishop_file == File::A) || (bishop_rank == Rank::Eighth)) {
        // file to iterate over the target files
        let mut target_file = bishop_file.left();
        // rank to iterate over the target ranks
        let mut target_rank = bishop_rank.up();

        // iterate in direction North-West
        while (target_file != File::A) && (target_rank != Rank::Eighth) {
            occupancy_bb.set_bit(Square::from_file_rank(target_file, target_rank));
            target_file = target_file.left();
            target_rank = target_rank.up();
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate occupancies South-East
    // -------------------------------------------------------------------------------------------

    // // If bishop is on file H or rank 1, can't go further South-East
    if !((bishop_file == File::H) || (bishop_rank == Rank::First)) {
        // file to iterate over the target files
        let mut target_file = bishop_file.right();
        // rank to iterate over the target ranks
        let mut target_rank = bishop_rank.down();

        // iterate in direction South-East
        while (target_file != File::H) && (target_rank != Rank::First) {
            occupancy_bb.set_bit(Square::from_file_rank(target_file, target_rank));
            target_file = target_file.right();
            target_rank = target_rank.down();
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate occupancies South-West
    // -------------------------------------------------------------------------------------------

    // // If bishop is on file A or rank 1, can't go further South-West
    if !((bishop_file == File::A) || (bishop_rank == Rank::First)) {
        // file to iterate over the target files
        let mut target_file = bishop_file.left();
        // rank to iterate over the target ranks
        let mut target_rank = bishop_rank.down();

        // iterate in direction South-East
        while (target_file != File::A) && (target_rank != Rank::First) {
            occupancy_bb.set_bit(Square::from_file_rank(target_file, target_rank));
            target_file = target_file.left();
            target_rank = target_rank.down();
        }
    }
    
    occupancy_bb
}

#[cfg(test)]
mod tests {
    use crate::board::file::{File, NUM_FILES};
    use crate::board::rank::{NUM_RANKS, Rank};
    use crate::board::square::{A1, A8, B3, C7, D8, E4, F6, G1, H1, H8, NUM_SQUARES, Square};
    use crate::lookup::bishop_occupancies::get_occupancy_bb;

    #[test]
    fn get_occupancy_bb_returns_bitboard_with_relevant_occupancy_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get occupancies for square
            let occupancy_bb = get_occupancy_bb(Square::new(square_index));
            // print occupancy_bb for debugging purposes
            println!("{occupancy_bb}");
        }

        // If I wanted to test for every square like I did for pawns, I'd basically have to rewrite the get_occupancy_bb function in here again, just for this test.
        // So I will only test for a few exemplary cases (using hard-coded hex values for the result bitboard). 
        // The correctness of the function can also be verified by checking the console output (see println! above).
        // You can see the bishop's occupancy squares shifting up the bitboard. Notice that file A, file H, rank 1, and rank 8, are not relevant for
        // the bishop occupancies and are thus not set to 1.

        assert_eq!(0x2442800284400, get_occupancy_bb(E4).value);
        assert_eq!(0x40201008040200, get_occupancy_bb(A1).value);
        assert_eq!(0x2040810204000, get_occupancy_bb(H1).value);
        assert_eq!(0x2040810204000, get_occupancy_bb(A8).value);
        assert_eq!(0x40201008040200, get_occupancy_bb(H8).value);
        assert_eq!(0x14224000000000,  get_occupancy_bb(D8).value);
        assert_eq!(0x20408102000, get_occupancy_bb(G1).value);
        assert_eq!(0x20100804000400, get_occupancy_bb(B3).value);
        assert_eq!(0x50005008040200, get_occupancy_bb(F6).value);
        assert_eq!(0xa1020400000, get_occupancy_bb(C7).value);
    }

    #[test]
    fn get_occupancy_bb_returns_bitboard_with_a_h_files_and_first_second_rank_and_bishop_square_unset() {
        for square_index in 0..NUM_SQUARES {
            // get occupancies for square
            let occupancy_bb = get_occupancy_bb(Square::new(square_index));

            // assert that bishop square is unset
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