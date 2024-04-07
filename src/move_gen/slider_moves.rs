use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::lookup::LOOKUP_TABLE;
use crate::move_gen::ply::Ply;

/// Generates all legal slider moves for the given position.
pub fn generate_slider_moves(position: Position) -> Vec<Ply> {
    let mut move_list: Vec<Ply> = Vec::new();
    move_list.append(&mut generate_slider_moves_by_piece(position, Piece::Bishop));
    move_list.append(&mut generate_slider_moves_by_piece(position, Piece::Rook));
    move_list.append(&mut generate_slider_moves_by_piece(position, Piece::Queen));
    move_list
}

/// Generates all legal slider moves for a given piece type in the given position.
fn generate_slider_moves_by_piece(position: Position, piece: Piece) -> Vec<Ply> {
    // get a reference to the lookup table
    let lookup = LOOKUP_TABLE.get().unwrap();

    let mut move_list: Vec<Ply> = Vec::new();

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
            Piece::Queen => lookup.get_rook_attacks(source_square, occupancies),
            _non_slider => return move_list
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
            move_list.push(Ply {
                source: source_square,
                target: target_square,
                piece,
                captured_piece: attacked_piece,
                promotion_piece: None,
            })
        }
    }
    // check for legality
    let mut legal_move_list: Vec<Ply> = Vec::new();
    for ply in move_list {
        if position.make_move(ply).is_legal() {
            legal_move_list.push(ply);
        }
    }
    legal_move_list
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::piece::Piece;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::slider_moves::generate_slider_moves_by_piece;

    #[test]
    fn test_generate_slider_moves_by_piece_for_bishop() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR b KQkq - 0 2").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(5, move_list.len());

        // position 3

        let position = Board::from_fen("r2qr1k1/pp2bppp/5nb1/3p4/6P1/2P1Bn1P/PPBQNP2/RN2K2R w KQ - 3 16").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(0, move_list.len());

        // position 4

        let position = Board::from_fen("r2qr1k1/pp2bppp/5nb1/3pn3/6P1/2P1B2P/PPBQNP2/RN2K2R b KQ - 2 15").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(10, move_list.len());

        // position 5

        let position = Board::from_fen("N6r/pp1kpp1p/5npb/2n5/7P/4BP2/PP1K1P2/5B1R w - - 3 18").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(10, move_list.len());

        // position 6

        let position = Board::from_fen("rnb1kbnr/ppp2ppp/3p1q2/4p3/3P1B2/2P1P3/PP3PPP/RN1QKBNR w KQkq - 0 5").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(9, move_list.len());

        // position 7

        let position = Board::from_fen("r1b1k1nr/2p1bpp1/3p2qp/1BnPp3/4P1P1/4BN1P/PPP1QP2/2KR2R1 b kq - 0 14").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(1, move_list.len());

        // position 8

        let position = Board::from_fen("r1bqkb1r/1p2ppp1/p1n4p/3p4/3PnB2/2PB1N1P/PP3PP1/RN1QK2R w KQkq - 2 9").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(18, move_list.len());

        // position 9

        let position = Board::from_fen("r1bqk2r/ppp1bpp1/2n1p2p/3p4/3Pn2B/P1N1PN2/1PP2PPP/R2QKB1R w KQkq - 3 8").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(9, move_list.len());

        // position 10

        let position = Board::from_fen("rn2kbnr/ppp1pppp/8/1b1p4/3PP3/5P1N/PPPKB1qP/RNBQ3R w kq - 3 7").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Bishop);
        assert_eq!(0, move_list.len());
    }

    #[test]
    fn test_generate_slider_moves_by_piece_for_rook() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/1p1pp3/2p2ppp/p6P/4P3/P6R/1PPP1PP1/RNBQKBN1 w Qkq - 0 6").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(10, move_list.len());

        // position 3

        let position = Board::from_fen("3rr1k1/ppp2p1p/3p2p1/3P2P1/5P2/4Q3/P1B3P1/5RK1 b - - 0 33").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(10, move_list.len());

        // position 4

        let position = Board::from_fen("5rk1/4bppp/4p3/4Bb2/2rPn3/1Q3N1P/5PP1/2R2RK1 b - - 0 27").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(15, move_list.len());

        // position 5

        let position = Board::from_fen("r4b1r/4nkpp/pq6/1p1n4/4NB2/5P2/PP4PP/2RQR1K1 w - - 4 21").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(2, move_list.len());

        // position 6

        let position = Board::from_fen("r4b1r/4nkpp/p7/1p1n4/4N3/4qP2/PP4PP/2RQR1K1 w - - 0 22").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(1, move_list.len());

        // position 7

        let position = Board::from_fen("rnb1kbnr/ppp5/3p1ppp/4p3/P2P4/3KR2q/1PP1PPP1/RNBQ1BN1 w kq - 0 8").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(5, move_list.len());

        // position 8

        let position = Board::from_fen("8/2pr4/5k2/7p/1P6/4RPPP/r2pK3/3R4 b - - 5 45").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(18, move_list.len());

        // position 9

        let position = Board::from_fen("6k1/3R3R/R7/4R3/2R5/5R2/6K1/1R5R w - - 0 1").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(101, move_list.len());

        // position 10

        let position = Board::from_fen("4R2b/8/5R2/r1R1K3/3R1R2/2b5/5r1b/7k w - - 0 1").unwrap().position;
        let move_list = generate_slider_moves_by_piece(position, Piece::Rook);
        assert_eq!(12, move_list.len());
    }
}