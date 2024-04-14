use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::lookup::LOOKUP_TABLE;
use crate::move_gen::move_list::MoveList;
use crate::move_gen::ply::Ply;

/// Generates all legal slider moves for the given position.
pub fn generate_slider_moves(position: Position, move_list: &mut MoveList) {
    generate_slider_moves_by_piece(position, Piece::Bishop, move_list);
    generate_slider_moves_by_piece(position, Piece::Rook, move_list);
    generate_slider_moves_by_piece(position, Piece::Queen, move_list);
}

/// Generates all legal slider moves for a given piece type in the given position.
fn generate_slider_moves_by_piece(position: Position, piece: Piece, move_list: &mut MoveList) {
    // get a reference to the lookup table
    let lookup = LOOKUP_TABLE.get().unwrap();

    // get occupancies
    let occupancies = position.get_occupancies();

    // get all squares with the piece type on it
    let active_squares = position.pieces[position.color_to_move.to_index() as usize][piece.to_index() as usize].get_active_bits();

    // loop over squares and calculate possible moves
    for source_square in active_squares {
        // get the attack_bb for the piece
        let mut attack_bb = match piece {
            Piece::Bishop => lookup.get_bishop_attacks(source_square, occupancies),
            Piece::Rook => lookup.get_rook_attacks(source_square, occupancies),
            Piece::Queen => lookup.get_queen_attacks(source_square, occupancies),
            _non_slider => return,
        };

        // mask the squares of the attack bb which are not suitable targets because they are occupied by a friendly piece
        let friendly_pieces_mask = attack_bb.value & position.get_occupancy(position.color_to_move).value;

        // `xor` the attack_bb with the friendly_pieces_mask to exclude squares with friendly pieces from the attack bb
        attack_bb.value ^= friendly_pieces_mask;

        // get target squares from the attack bb
        let target_squares = attack_bb.get_active_bits();

        // loop over target squares and add ply
        for target_square in target_squares {
            // get the type of the attacked piece
            let attacked_piece = position.get_piece(target_square).map(|(piece, _color)| piece);

            let ply = Ply { source: source_square, target: target_square, piece, captured_piece: attacked_piece, promotion_piece:None};
            if position.make_move(ply).is_legal() {
                move_list.push(ply);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::piece::Piece;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::move_list::MoveList;
    use crate::move_gen::slider_moves::{generate_slider_moves, generate_slider_moves_by_piece};

    #[test]
    fn test_generate_slider_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/ppp2ppp/8/3p4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 4").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(16, move_list.len());

        // position 3

        let position = Board::from_fen("rn2k2r/2pq1ppp/p2bb3/1p1N2B1/2pP4/5N2/PP2BPPP/R2Q1RK1 b kq - 0 12").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(20, move_list.len());

        // position 4

        let position = Board::from_fen("r1kq4/p4Qp1/n1pp2p1/8/1P1N2b1/2P5/P4PPP/RN4K1 w - - 0 21").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(18, move_list.len());

        // position 5

        let position = Board::from_fen("r2qk2r/pp1nBpp1/7p/4p3/3pn3/3P1N2/PPP2PPP/R2QK2R b KQkq - 0 12").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(11, move_list.len());

        // position 6

        let position = Board::from_fen("r4rk1/p2n1pp1/1p5p/4p3/3p2n1/3P1N2/PPPBK1PP/R6R w - - 4 18").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(21, move_list.len());

        // position 7

        let position = Board::from_fen("3q2k1/1pp1br2/5n2/4pb2/8/2Pn2K1/1P4P1/r7 b - - 0 26").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(39, move_list.len());

        // position 8

        let position = Board::from_fen("r4rk1/ppp2qpn/3p1p1p/3B2b1/4P3/1Q2BP2/PPP3PP/3R1RK1 b - - 3 18").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(13, move_list.len());

        // position 9

        let position = Board::from_fen("3r2k1/1pp2r1n/3Q1ppp/p5b1/4P3/3KBP2/PPP3PP/3R1R2 w - - 0 24").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(23, move_list.len());

        // position 10

        let position = Board::from_fen("r4r1k/ppp1q1pn/3p1p1p/3Bn3/2N1PR2/P2PQ3/1PP3PP/5RK1 w - - 3 20").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves(position, &mut move_list);
        assert_eq!(30, move_list.len());
    }

    #[test]
    fn test_generate_slider_moves_by_piece_for_bishop() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR b KQkq - 0 2").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(5, move_list.len());

        // position 3

        let position = Board::from_fen("r2qr1k1/pp2bppp/5nb1/3p4/6P1/2P1Bn1P/PPBQNP2/RN2K2R w KQ - 3 16").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 4

        let position = Board::from_fen("r2qr1k1/pp2bppp/5nb1/3pn3/6P1/2P1B2P/PPBQNP2/RN2K2R b KQ - 2 15").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 5

        let position = Board::from_fen("N6r/pp1kpp1p/5npb/2n5/7P/4BP2/PP1K1P2/5B1R w - - 3 18").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 6

        let position = Board::from_fen("rnb1kbnr/ppp2ppp/3p1q2/4p3/3P1B2/2P1P3/PP3PPP/RN1QKBNR w KQkq - 0 5").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(9, move_list.len());

        // position 7

        let position = Board::from_fen("r1b1k1nr/2p1bpp1/3p2qp/1BnPp3/4P1P1/4BN1P/PPP1QP2/2KR2R1 b kq - 0 14").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(1, move_list.len());

        // position 8

        let position = Board::from_fen("r1bqkb1r/1p2ppp1/p1n4p/3p4/3PnB2/2PB1N1P/PP3PP1/RN1QK2R w KQkq - 2 9").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(18, move_list.len());

        // position 9

        let position = Board::from_fen("r1bqk2r/ppp1bpp1/2n1p2p/3p4/3Pn2B/P1N1PN2/1PP2PPP/R2QKB1R w KQkq - 3 8").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(9, move_list.len());

        // position 10

        let position = Board::from_fen("rn2kbnr/ppp1pppp/8/1b1p4/3PP3/5P1N/PPPKB1qP/RNBQ3R w kq - 3 7").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Bishop, &mut move_list);
        assert_eq!(0, move_list.len());
    }

    #[test]
    fn test_generate_slider_moves_by_piece_for_rook() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/1p1pp3/2p2ppp/p6P/4P3/P6R/1PPP1PP1/RNBQKBN1 w Qkq - 0 6").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 3

        let position = Board::from_fen("3rr1k1/ppp2p1p/3p2p1/3P2P1/5P2/4Q3/P1B3P1/5RK1 b - - 0 33").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 4

        let position = Board::from_fen("5rk1/4bppp/4p3/4Bb2/2rPn3/1Q3N1P/5PP1/2R2RK1 b - - 0 27").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(15, move_list.len());

        // position 5

        let position = Board::from_fen("r4b1r/4nkpp/pq6/1p1n4/4NB2/5P2/PP4PP/2RQR1K1 w - - 4 21").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(2, move_list.len());

        // position 6

        let position = Board::from_fen("r4b1r/4nkpp/p7/1p1n4/4N3/4qP2/PP4PP/2RQR1K1 w - - 0 22").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(1, move_list.len());

        // position 7

        let position = Board::from_fen("rnb1kbnr/ppp5/3p1ppp/4p3/P2P4/3KR2q/1PP1PPP1/RNBQ1BN1 w kq - 0 8").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(5, move_list.len());

        // position 8

        let position = Board::from_fen("8/2pr4/5k2/7p/1P6/4RPPP/r2pK3/3R4 b - - 5 45").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(18, move_list.len());

        // position 9

        let position = Board::from_fen("6k1/3R3R/R7/4R3/2R5/5R2/6K1/1R5R w - - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(101, move_list.len());

        // position 10

        let position = Board::from_fen("4R2b/8/5R2/r1R1K3/3R1R2/2b5/5r1b/7k w - - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Rook, &mut move_list);
        assert_eq!(12, move_list.len());
    }

    #[test]
    fn test_generate_slider_moves_by_piece_for_queen() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/pp1ppppp/2p5/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(4, move_list.len());

        // position 3

        let position = Board::from_fen("1nbqkbnr/r2ppppp/p1p5/1p5Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQk - 2 5").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(16, move_list.len());

        // position 4

        let position = Board::from_fen("1nb1kbnr/r2pqQpp/p1p1p3/1p6/2B1P3/P4P2/1PPP2PP/RNB1K1NR b KQk - 0 7").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(1, move_list.len());

        // position 5

        let position = Board::from_fen("1nb1kbnr/r4qpp/p1ppp3/1p6/2B1P2P/P4P2/1PPP2P1/RNB1K1NR b KQk - 0 9").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 6

        let position = Board::from_fen("1nb1kbn1/B4qpr/p1ppp3/1p5p/2B1P2P/P2P1P2/1PP3P1/RN2K1NR b KQ - 0 12").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(10, move_list.len());

        // position 7

        let position = Board::from_fen("r1bqkb1r/pppp1Qpp/2n2n2/6N1/4P3/8/PPP2PPP/RNB1K2R b KQ - 4 9").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(0, move_list.len());

        // position 8

        let position = Board::from_fen("r1bqr1k1/ppp2ppp/2nb1n2/8/3pP2Q/2NB1P2/PPPB2PP/2KR2NR w - - 0 12").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(11, move_list.len());

        // position 9

        let position = Board::from_fen("r2q1rk1/pp2bppp/3pn3/2pN4/4PPb1/1PPBQ2P/P5P1/R1B2RK1 b - - 0 17").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(7, move_list.len());

        // position 10

        let position = Board::from_fen("r2qk1nr/ppp2p1p/7b/1b1QP3/3n1B1P/8/PPP3P1/RN3RK1 w kq - 0 12").unwrap().position;
        let mut move_list = MoveList::default();
        generate_slider_moves_by_piece(position, Piece::Queen, &mut move_list);
        assert_eq!(14, move_list.len());
    }
}