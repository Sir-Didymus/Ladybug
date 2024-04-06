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
                break;
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
                if source.get_rank() == position.color_to_move.pawn_rank()  {
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
    use crate::move_gen::pawn_moves;

    #[test]
    fn test_generate_quiet_pawn_moves() {
        // position 1 (starting position)
        
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let move_list = pawn_moves::generate_quiet_pawn_moves(position);
        assert_eq!(16, move_list.len());
    }
}