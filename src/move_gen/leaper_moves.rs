use crate::board::castling_rights::CastlingRights;
use crate::board::color::{Color};
use crate::board::file::File;
use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::rank::Rank;
use crate::board::square::Square;
use crate::lookup::LOOKUP_TABLE;
use crate::move_gen::ply::Ply;

/// Generates all legal leaper moves for the given position, including castling moves.
pub fn generate_leaper_moves(position: Position) -> Vec<Ply> {
    let mut move_list: Vec<Ply> = Vec::new();
    move_list.append(&mut generate_leaper_moves_by_piece(position, Piece::Knight));
    move_list.append(&mut generate_leaper_moves_by_piece(position, Piece::King));
    move_list.append(&mut generate_castling_moves(position));
    move_list
}

/// Generates all legal leaper moves (knights and kings) for a given leaper piece type in the given position.
fn generate_leaper_moves_by_piece(position: Position, piece: Piece) -> Vec<Ply> {
    // get a reference to the lookup table
    let lookup = LOOKUP_TABLE.get().unwrap();

    let mut move_list: Vec<Ply> = Vec::new();

    // get all squares with the piece type on it
    let active_squares = position.pieces[position.color_to_move.to_index() as usize][piece.to_index() as usize].get_active_bits();

    // loop over squares and calculate possible moves
    for source_square in active_squares {
        // get the attack_bb for the piece
        let mut attack_bb = match piece {
            Piece::Knight => lookup.get_knight_attacks(source_square),
            Piece::King => lookup.get_king_attacks(source_square),
            _non_leaper => return move_list
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

/// Generates all legal castling moves the given position.
fn generate_castling_moves(position: Position) -> Vec<Ply> {
    let mut move_list: Vec<Ply> = Vec::new();

    let castling_rights = position.castling_rights[position.color_to_move as usize];

    // the color to move has no castling rights - return empty list
    if castling_rights == CastlingRights::NoRights {
        return move_list;
    }

    // the king is in check - return empty list
    if position.is_in_check(position.color_to_move) {
        return move_list;
    }

    // get castling rank
    let rank = match position.color_to_move {
        Color::White => Rank::First,
        Color::Black => Rank::Eighth,
    };

    // get attack bb for opponent
    let attack_bb = position.get_attack_bb(position.color_to_move.other());

    // get occupancies
    let occupancies = position.get_occupancies();

    // queenside
    if (castling_rights == CastlingRights::QueenSide || castling_rights == CastlingRights::Both) // color to move has castling rights for queenside
        && !occupancies.get_bit(Square::from_file_rank(File::B, rank))  // B file square is unoccupied
        && !occupancies.get_bit(Square::from_file_rank(File::C, rank)) // C file square is unoccupied
        && !occupancies.get_bit(Square::from_file_rank(File::D, rank)) // D file square is unoccupied
        && !attack_bb.get_bit(Square::from_file_rank(File::C, rank))  // C file square is not attacked
        && !attack_bb.get_bit(Square::from_file_rank(File::D, rank)) // D file square is not attacked
        && position.pieces[position.color_to_move.to_index() as usize][Piece::Rook.to_index() as usize].get_bit(Square::from_file_rank(File::A, rank)) // Rook is still on A file square
    {
        move_list.push(Ply {
            source: Square::from_file_rank(File::E, rank),
            target: Square::from_file_rank(File::C, rank),
            piece: Piece::King,
            captured_piece: None,
            promotion_piece: None,
        })
    }

    // kingside
    if (castling_rights == CastlingRights::KingSide || castling_rights == CastlingRights::Both) // color to move has castling rights for kingside
        && !occupancies.get_bit(Square::from_file_rank(File::F, rank))  // F file square is unoccupied
        && !occupancies.get_bit(Square::from_file_rank(File::G, rank)) // G file square is unoccupied
        && !attack_bb.get_bit(Square::from_file_rank(File::F, rank))  // F file square is not attacked
        && !attack_bb.get_bit(Square::from_file_rank(File::G, rank)) // G file square is not attacked
        && position.pieces[position.color_to_move.to_index() as usize][Piece::Rook.to_index() as usize].get_bit(Square::from_file_rank(File::H, rank)) // Rook is still on H file square
    {
        move_list.push(Ply {
            source: Square::from_file_rank(File::E, rank),
            target: Square::from_file_rank(File::G, rank),
            piece: Piece::King,
            captured_piece: None,
            promotion_piece: None,
        })
    }
    move_list
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::piece::Piece;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::leaper_moves::{generate_castling_moves, generate_leaper_moves, generate_leaper_moves_by_piece};

    #[test]
    fn test_generate_leaper_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(4, move_list.len());

        // position 2

        let position = Board::from_fen("r1bqk2r/ppp1bppp/2nppn2/8/3PP3/2NB1N2/PPP2PPP/R1BQK2R w KQkq - 4 6").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(14, move_list.len());

        // position 3

        let position = Board::from_fen("r1bqk2r/1p2bppp/p1nppn2/8/3PP3/P1NB1N2/1P3PPP/R1BQ1RK1 b kq - 0 9").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(15, move_list.len());

        // position 4

        let position = Board::from_fen("rn2k2r/pppqbppp/4pn2/3p4/Q4Pb1/2P1PN2/PP1PB1PP/RNB2RK1 b kq - 2 7").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(8, move_list.len());

        // position 5

        let position = Board::from_fen("4k2r/3b1ppp/3pp3/2p5/8/2P5/PP1n1PPP/R3K1NR w KQk - 0 14").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(7, move_list.len());

        // position 6

        let position = Board::from_fen("rnbqk2N/2pp2pp/pp5n/4p3/1b2PP2/1PP5/P2P2PP/RNBQKB1R b KQq - 0 8").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(7, move_list.len());

        // position 7

        let position = Board::from_fen("2rq1r1k/pb1nb1pp/1p6/4p3/2n1Pp2/1QBN1P2/P3B1PP/RN3RK1 b - - 1 20").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(10, move_list.len());

        // position 8

        let position = Board::from_fen("rnbq1rk1/pppp1Npp/5n2/8/2B1P3/8/PPPP1bPP/RNBQK2R w KQ - 0 6").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(3, move_list.len());

        // position 9

        let position = Board::from_fen("r2qk2r/ppp1bppp/2np1n2/4p3/2BPP3/2P2Q1P/PB3PP1/RN3RK1 b kq - 2 10").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(13, move_list.len());

        // position 10

        let position = Board::from_fen("r1bqk2r/ppp2pbp/3p4/4n3/4P3/3P1N2/PPP3PP/RNBQK2R w KQkq - 0 9").unwrap().position;
        let move_list = generate_leaper_moves(position);
        assert_eq!(14, move_list.len());
    }

    #[test]
    fn test_generate_leaper_moves_by_piece_for_knights() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(4, move_list.len());

        // position 2

        let position = Board::from_fen("r4rk1/ppp2p1p/2q3p1/3p4/1Q1P2n1/P1N2N2/1P3PPP/1R4K1 b - - 2 21").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(6, move_list.len());

        // position 3

        let position = Board::from_fen("4rrk1/1pp2p1p/2q3p1/p2p2Q1/3Pn3/P1N2N2/1P3PPP/2R3K1 w - - 6 25").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(12, move_list.len());

        // position 4

        let position = Board::from_fen("3n1rk1/1rpq1pp1/1bppb2p/4p3/pP2P3/PRB1P1P1/2Q1NPBP/3R2K1 b - - 3 22").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(0, move_list.len());

        // position 5

        let position = Board::from_fen("2R3k1/p2rbpp1/4p2p/N2pPn2/1P1P4/P3B3/5PPP/6K1 b - - 1 30").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(0, move_list.len());

        // position 6

        let position = Board::from_fen("3r1b1r/Q3nk1p/6p1/8/2q5/NPP4P/P2P4/R1B1K1R1 b - - 0 24").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(0, move_list.len());

        // position 7

        let position = Board::from_fen("3rkb1r/4n2p/2Q3p1/8/8/N1P5/PP1P3q/R1BK1R2 b - - 1 24").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(1, move_list.len());

        // position 8

        let position = Board::from_fen("r4r1k/2p3pp/p1n1q3/3n1p2/2BPP3/1P2P3/P5PP/R2Q1RK1 b - - 0 21").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(15, move_list.len());

        // position 9

        let position = Board::from_fen("8/5k2/2N5/N7/3N4/1N3N2/3N4/K7 w - - 0 1").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(25, move_list.len());

        // position 10

        let position = Board::from_fen("8/8/8/p5k1/2R2p2/P7/2p1KP2/2r5 w - - 0 48").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::Knight);
        assert_eq!(0, move_list.len());
    }

    #[test]
    fn test_generate_leaper_moves_by_piece_for_kings() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(1, move_list.len());

        // position 3

        let position = Board::from_fen("rnbq1bnr/ppppkppp/8/4p3/4P3/8/PPPPKPPP/RNBQ1BNR w - - 2 3").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(4, move_list.len());

        // position 4

        let position = Board::from_fen("rnbq1bnr/ppppkp1p/6p1/4p1B1/3PP3/8/PPP1KPPP/RN1Q1BNR b - - 1 4").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(3, move_list.len());

        // position 5

        let position = Board::from_fen("rnbq1bnr/ppppk2p/5Bp1/4p3/3PP3/8/PPP1KPPP/RN1Q1BNR b - - 0 5").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(5, move_list.len());

        // position 6

        let position = Board::from_fen("7k/7P/7K/8/8/8/8/8 b - - 0 1").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(0, move_list.len());

        // position 7

        let position = Board::from_fen("7k/7P/7K/8/8/8/8/8 w - - 0 1").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(3, move_list.len());

        // position 8

        let position = Board::from_fen("8/p7/1ppR3k/6r1/8/8/PB2KPbP/8 b - - 0 31").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(2, move_list.len());

        // position 9

        let position = Board::from_fen("rn3r2/pb4R1/1ppp2kN/3n1p2/8/B2B4/P4PPP/3R2K1 b - - 1 22").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(4, move_list.len());

        // position 10

        let position = Board::from_fen("8/1Q6/p7/b1k3P1/5P2/8/7P/5K2 b - - 0 38").unwrap().position;
        let move_list = generate_leaper_moves_by_piece(position, Piece::King);
        assert_eq!(3, move_list.len());
    }

    #[test]
    fn test_generate_castling_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(0, move_list.len());

        // position 2

        let position = Board::from_fen("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B5/2NPPN2/PPP2PPP/R1BQK2R b KQkq - 0 5").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(1, move_list.len());

        // position 3

        let position = Board::from_fen("r1bq1rk1/pppp1pp1/2n2n2/2b1p2p/2B5/2NPPN2/PPPBQPPP/R3K2R w KQ - 0 8").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(2, move_list.len());

        // position 4

        let position = Board::from_fen("r1bq1rk1/pppp1pp1/2n2n2/4p3/P1B4p/bPNPPN2/2PBQPPP/R3K2R w KQ - 0 10").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(1, move_list.len());

        // position 5

        let position = Board::from_fen("r1bq1rk1/pppp1pp1/2n5/4p3/PPB4p/b1NPnN1P/2PBQPP1/R3K2R w KQ - 0 12").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(0, move_list.len());

        // position 6

        let position = Board::from_fen("r1bq1rk1/p1pp1p2/1pn3p1/4p3/PPB4p/R1NPP2P/2PBQ1P1/4K1NR b K - 0 14").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(0, move_list.len());

        // position 7

        let position = Board::from_fen("r1b2rk1/p1pp1p2/1pn3p1/8/PPBNpq1p/R1NPP2P/2PBQ1P1/4K2R w K - 4 17").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(0, move_list.len());

        // position 8

        let position = Board::from_fen("r1b2rk1/p1pp1p2/1pn3p1/P7/1PBNp2p/R1NPP2q/2PB1QP1/4K2R w K - 0 19").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(1, move_list.len());

        // position 9

        let position = Board::from_fen("r3k1nr/pQp1qppp/2np4/b7/2B1P1b1/P1N2N2/5PPP/R1B2RK1 b kq - 0 11").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(0, move_list.len());

        // position 10

        let position = Board::from_fen("r3k2r/Q1p1qppp/2np1n2/b7/2B1P1b1/P1N2N2/5PPP/R1B2RK1 b kq - 0 12").unwrap().position;
        let move_list = generate_castling_moves(position);
        assert_eq!(2, move_list.len());
    }
}