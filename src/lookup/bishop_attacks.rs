use crate::board::bitboard::Bitboard;
use crate::board::file::File;
use crate::board::rank::Rank;
use crate::board::square::Square;

/// Returns the attack bitboard for a bishop, depending on the bishop's square and the blocker bitboard.
fn get_attack_bb(square: Square,  blockers:  Bitboard) -> Bitboard {
    // the result attack bitboard
    let mut attack_bb = Bitboard::new(0);

    // file on which the bishop resides
    let bishop_file = square.get_file();

    // rank on which the bishop resides
    let bishop_rank = square.get_rank();

    // -------------------------------------------------------------------------------------------
    // generate attacks North-East
    // -------------------------------------------------------------------------------------------
    
    // file to iterate over the target files
    let mut target_file = bishop_file;
    // rank to iterate over the target ranks
    let mut target_rank = bishop_rank;

    // iterate in direction North-East
    while (target_file != File::H) && (target_rank != Rank::Eighth) {
        target_file = target_file.right();
        target_rank = target_rank.up();
        attack_bb.set_bit(Square::from_file_rank(target_file, target_rank));
        if blockers.get_bit(Square::from_file_rank(target_file, target_rank)) {
            break;
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate attacks North-West
    // -------------------------------------------------------------------------------------------

    // file to iterate over the target files
    let mut target_file = bishop_file;
    // rank to iterate over the target ranks
    let mut target_rank = bishop_rank;

    // iterate in direction North-West
    while (target_file != File::A) && (target_rank != Rank::Eighth) {
        target_file = target_file.left();
        target_rank = target_rank.up();
        attack_bb.set_bit(Square::from_file_rank(target_file, target_rank));
        if blockers.get_bit(Square::from_file_rank(target_file, target_rank)) {
            break;
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate attacks South-East
    // -------------------------------------------------------------------------------------------

    // file to iterate over the target files
    let mut target_file = bishop_file;
    // rank to iterate over the target ranks
    let mut target_rank = bishop_rank;

    // iterate in direction South-East
    while (target_file != File::H) && (target_rank != Rank::First) {
        target_file = target_file.right();
        target_rank = target_rank.down();
        attack_bb.set_bit(Square::from_file_rank(target_file, target_rank));
        if blockers.get_bit(Square::from_file_rank(target_file, target_rank)) {
            break;
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate attacks South-West
    // -------------------------------------------------------------------------------------------

    // file to iterate over the target files
    let mut target_file = bishop_file;
    // rank to iterate over the target ranks
    let mut target_rank = bishop_rank;

    // iterate in direction South-West
    while (target_file != File::A) && (target_rank != Rank::First) {
        target_file = target_file.left();
        target_rank = target_rank.down();
        attack_bb.set_bit(Square::from_file_rank(target_file, target_rank));
        if blockers.get_bit(Square::from_file_rank(target_file, target_rank)) {
            break;
        }
    }
    
    attack_bb
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::square::{D8, E4, G6, H7, NUM_SQUARES, Square};
    use crate::lookup::bishop_attacks::get_attack_bb;

    #[test]
    fn get_attack_bb_returns_bitboard_with_attack_bits_set() {
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = get_attack_bb(Square::new(square_index), Bitboard::new(0));
            // print attack_bb with empty blockers for debugging purposes
            println!("{attack_bb}");
        }

        // If I wanted to test for every square like I did for pawns, I'd basically have to rewrite the get_attack_bb function in here again, just for this test.
        // Also, the number of test cases would be very large, since in this case blockers play a role, too.
        // So I will only test for a few exemplary cases (using hard-coded hex values for the result and blocker bitboard).
        
        assert_eq!(0x182442800284482, get_attack_bb(E4, Bitboard::new(0)).value);
        assert_eq!(0x182442800284080, get_attack_bb(E4, Bitboard::new(0x80000)).value);
        assert_eq!(0x82442800284080, get_attack_bb(E4, Bitboard::new(0x2000000080000)).value);
        assert_eq!(0x4000402010080000, get_attack_bb(H7, Bitboard::new(0x80000)).value);
        assert_eq!(0x10a000a010080402, get_attack_bb(G6, Bitboard::new(0)).value);
        assert_eq!(0x10a000a000000000, get_attack_bb(G6, Bitboard::new(0x2000000000)).value);
        assert_eq!(0xa000a000000000, get_attack_bb(G6, Bitboard::new(0xa000a000000000)).value);
        assert_eq!(0x14224080000000, get_attack_bb(D8, Bitboard::new(0x20080000000)).value);
    }
}