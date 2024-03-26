use crate::board::bitboard::Bitboard;
use crate::board::file::File;
use crate::board::rank::Rank;
use crate::board::square::Square;

/// Returns the attack bitboard for a rook, depending on the rook's square and the blocker bitboard.
fn get_attack_bb(square: Square, blockers: Bitboard) -> Bitboard {
    let mut attack_bb = Bitboard::new(0);

    // file on which the rook resides
    let rook_file = square.get_file();

    // rank on which the rook resides
    let rook_rank = square.get_rank();

    // -------------------------------------------------------------------------------------------
    // generate attacks North
    // -------------------------------------------------------------------------------------------

    // rank to iterate over the target ranks
    let mut target_rank = rook_rank;

    // iterate in direction North
    while target_rank != Rank::Eighth {
        target_rank = target_rank.up();
        attack_bb.set_bit(Square::from_file_rank(rook_file, target_rank));
        if blockers.get_bit(Square::from_file_rank(rook_file, target_rank)) {
            break;
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate attacks South
    // -------------------------------------------------------------------------------------------

    // rank to iterate over the target ranks
    let mut target_rank = rook_rank;

    // iterate in direction South
    while target_rank != Rank::First {
        target_rank = target_rank.down();
        attack_bb.set_bit(Square::from_file_rank(rook_file, target_rank));
        if blockers.get_bit(Square::from_file_rank(rook_file, target_rank)) {
            break;
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate attacks East
    // -------------------------------------------------------------------------------------------

    // file to iterate over the target files
    let mut target_file = rook_file;

    // iterate in direction East
    while target_file != File::H {
        target_file = target_file.right();
        attack_bb.set_bit(Square::from_file_rank(target_file, rook_rank));
        if blockers.get_bit(Square::from_file_rank(target_file, rook_rank)) {
            break;
        }
    }

    // -------------------------------------------------------------------------------------------
    // generate attacks West
    // -------------------------------------------------------------------------------------------

    // file to iterate over the target files
    let mut target_file = rook_file;

    // iterate in direction West
    while target_file != File::A {
        target_file = target_file.left();
        attack_bb.set_bit(Square::from_file_rank(target_file, rook_rank));
        if blockers.get_bit(Square::from_file_rank(target_file, rook_rank)) {
            break;
        }
    }

    attack_bb
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::square::{A1, C3, D5, G6, NUM_SQUARES, Square};
    use crate::lookup::rook_attacks::get_attack_bb;

    #[test]
    fn get_attack_bb_returns_bitboard_with_attack_bits_set() {
        // get attacks for square
        let attack_bb = get_attack_bb(A1, Bitboard::new(0x100000000000010));
        // print attack_bb with empty blockers for debugging purposes
        println!("{attack_bb}");
        for square_index in 0..NUM_SQUARES {
            // get attacks for square
            let attack_bb = get_attack_bb(Square::new(square_index), Bitboard::new(0));
            // print attack_bb with empty blockers for debugging purposes
            println!("{attack_bb}");
        }

        // If I wanted to test for every square like I did for pawns, I'd basically have to rewrite the get_attack_bb function in here again, just for this test.
        // Also, the number of test cases would be very large, since in this case blockers play a role, too.
        // So I will only test for a few exemplary cases (using hard-coded hex values for the result and blocker bitboard).

        assert_eq!(0x1010101010101fe, get_attack_bb(A1, Bitboard::new(0)).value);
        assert_eq!(0x1010101010101fe, get_attack_bb(A1, Bitboard::new(0x100000000000080)).value);
        assert_eq!(0x10101010101011e, get_attack_bb(A1, Bitboard::new(0x100000000000010)).value);
        assert_eq!(0x1011e, get_attack_bb(A1, Bitboard::new(0x10010)).value);
        assert_eq!(0x80808f708080808, get_attack_bb(D5, Bitboard::new(0)).value);
        assert_eq!(0x80808f708080808, get_attack_bb(D5, Bitboard::new(0x800008100000008)).value);
        assert_eq!(0x8083608000000, get_attack_bb(D5, Bitboard::new(0x80800e308080808)).value);
        assert_eq!(0x40be4040404040, get_attack_bb(G6, Bitboard::new(0x40020000000000)).value);
        assert_eq!(0x40404041a0404, get_attack_bb(C3, Bitboard::new(0x4000000120004)).value);
    }
}