use crate::board::color::Color;
use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::move_gen::ply::Ply;

/// Generates all legal pawn moves for the given position.
pub fn generate_pawn_moves(position: Position) -> Vec<Ply> {
    let mut move_list: Vec<Ply> = Vec::new();
    let mut quiet_pawn_moves = generate_quiet_pawn_moves(position);
    move_list.append(&mut quiet_pawn_moves);
    move_list
}

/// Generates all legal quiet pawn moves for the given position.
fn generate_quiet_pawn_moves(position: Position) -> Vec<Ply> {
    let mut move_list: Vec<Ply> = Vec::new();

    // get occupancies
    let occupancies = position.get_occupancies();

    // get pawn bitboard for the color to move
    let pawn_bb = position.pieces[position.color_to_move.to_index() as usize][Piece::Pawn.to_index() as usize];

    // get all squares with a pawn on it
    let active_squares = pawn_bb.get_active_bits();

    // loop over squares and calculate possible moves
    for source in active_squares {
        let target = match position.color_to_move {
            Color::White => source.up(),
            Color::Black => source.down(),
        };

        // check if target square is empty
        if occupancies.get_bit(target) {
            continue;
        }

        // check if target square is on the promotion rank
        if target.get_rank() == position.color_to_move.promotion_rank() {
            // move is a promotion - add all possible promotion moves
            for piece_index in Piece::Knight.to_index() as usize..Piece::Queen.to_index() as usize + 1 {
                move_list.push(Ply {
                    source,
                    target,
                    piece: Piece::Pawn,
                    captured_piece: None,
                    promotion_piece: Some(Piece::from_index(piece_index as u8)),
                });
            }
        } else {
            // move is not a promotion
            move_list.push(Ply {
                source,
                target,
                piece: Piece::Pawn,
                captured_piece: None,
                promotion_piece: None,
            });

            // check if double pawn push is possible
            if source.get_rank() == position.color_to_move.pawn_rank() {
                let mut double_pawn_push_target = target;
                match position.color_to_move {
                    Color::White => double_pawn_push_target = double_pawn_push_target.up(),
                    Color::Black => double_pawn_push_target = double_pawn_push_target.down(),
                }
                if !occupancies.get_bit(double_pawn_push_target) {
                    // no piece on double pawn push target square, so double pawn move is possible
                    move_list.push(Ply {
                        source,
                        target: double_pawn_push_target,
                        piece: Piece::Pawn,
                        captured_piece: None,
                        promotion_piece: None,
                    });
                }
            }
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
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::pawn_moves;

    #[test]
    fn test_generate_quiet_pawn_moves() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1 (starting position)

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(16, move_list.len());

        // position 2

        let position = Board::from_fen("r4rk1/6pp/pp2b3/3pPp2/4nP1q/1PNQ2bP/PB2B1PK/R4R2 w - - 11 22").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(0, move_list.len());

        // position 3

        let position = Board::from_fen("r1bqkbnr/1pp3pp/p1np4/4pp2/2P5/1P2PN2/PB1P1PPP/RN1QKB1R w KQkq - 0 6").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(11, move_list.len());

        // position 4

        let position = Board::from_fen("r1b1kbnr/1pp3pp/p1n5/4Bp2/2P4q/1P2P3/P2P1PPP/RN1QKB1R w KQkq - 1 8").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(10, move_list.len());

        // position 5

        let position = Board::from_fen("r3kbnr/1p4pp/2p5/p1PbB3/Pn1PPp1q/1P3PPP/8/RN1QKB1R w KQkq - 1 14").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(0, move_list.len());

        // position 6

        let position = Board::from_fen("r3kbnr/8/8/2PbB3/Pn1PP2q/1P3PPP/7R/RN1QKB2 b Qkq - 2 14").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(0, move_list.len());

        // position 7

        let position = Board::from_fen("r3kbnr/8/8/p1PbB3/Pn1PP2q/1P3PPP/7R/RN1QKB2 b Qkq - 2 14").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(0, move_list.len());

        // position 8

        let position = Board::from_fen("r3kbnr/1p6/8/2PbB3/Pn1PP2q/1P3PPP/7R/RN1QKB2 b Qkq - 2 14").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(2, move_list.len());

        // position 9

        let position = Board::from_fen("r3kbnr/1p6/8/1QPbB3/Pn1PP2q/1P3PPP/7R/R3KB2 b Qkq - 2 14").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(0, move_list.len());

        // position 10

        let position = Board::from_fen("r3kbnr/1p4Q1/8/1RPbB3/Pn1PP2q/1P3PPP/7R/4KB2 b kq - 2 14").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(1, move_list.len());
    }
}