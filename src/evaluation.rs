use crate::board::color::Color;
use crate::board::piece::{NUM_PIECES, Piece};
use crate::board::position::Position;

/// The values for each piece type.
pub const PIECE_VALUES: [i32; 6] = [100, 300, 300, 500, 900, POSITIVE_INFINITY];
/// The highest possible value.
pub const POSITIVE_INFINITY: i32 = i32::MAX - 1;
/// The lowest possible value.
pub const NEGATIVE_INFINITY: i32 = i32::MIN + 1;


/// Returns the static evaluation for the given position.
///
/// The evaluation is always done from the point of view of the side whose turn it is.
/// E.g. if it is Black's turn, and black is up a queen, the evaluation will return +9, 
/// even though chess players usually refer to such a position, from White's point of view, as -9.
pub fn evaluate(position: Position) -> i32 {
    evaluate_material(position)
}

/// Returns the purely materialistic evaluation of the position.
fn evaluate_material(position: Position) -> i32 {
    let mut material_score: i32 = 0;
    for piece_index in 0..NUM_PIECES {
        material_score += (position.get_num_pieces(Piece::from_index(piece_index), Color::White) as i32 
            - position.get_num_pieces(Piece::from_index(piece_index), Color::Black) as i32) * PIECE_VALUES[piece_index as usize];
    }
    // if it is Black's move, negate the material score so that the evaluation is from Black's perspective
    if position.color_to_move == Color::Black {
        material_score = -material_score;
    }
    material_score
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::evaluation::{evaluate_material};
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn test_evaluate_material() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);
        
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(0, evaluate_material(position));

        // White is missing a queen - White to move
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(-900, evaluate_material(position));

        // White is missing a queen - Black to move
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR b KQkq - 0 1").unwrap().position;
        assert_eq!(900, evaluate_material(position));

        // Black is missing a knight - White to move
        let position = Board::from_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(300, evaluate_material(position));

        // Black is missing a knight - Black to move
        let position = Board::from_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap().position;
        assert_eq!(-300, evaluate_material(position));
    }
}