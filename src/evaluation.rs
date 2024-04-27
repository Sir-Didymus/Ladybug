use crate::board::color::{Color, NUM_COLORS};
use crate::board::piece::{NUM_PIECES, Piece};
use crate::board::position::Position;

pub mod pst;
/// The highest possible value.
pub const POSITIVE_INFINITY: i32 = i32::MAX - 1;
/// The lowest possible value.
pub const NEGATIVE_INFINITY: i32 = i32::MIN + 1;


/// Returns the static evaluation for the given position.
///
/// The evaluation is always done from the point of view of the side whose turn it is.
/// E.g. if it is Black's turn, and black is up a queen, the evaluation will return +900, 
/// even though chess players usually refer to such a position, from White's point of view, as -9.
pub fn evaluate(position: Position) -> i32 {
    evaluate_material(position)
}

/// Returns the purely materialistic evaluation of the position.
fn evaluate_material(position: Position) -> i32 {
    let mut material_score: i32 = 0;
    for color_index in 0..NUM_COLORS {
        for piece_index in 0..NUM_PIECES {
            let active_bits = position.pieces[color_index as usize][piece_index as usize].get_active_bits();
            for square in active_bits {
                match Color::from_index(color_index) {
                    Color::White => material_score += pst::get_piece_value(Piece::from_index(piece_index), square, Color::from_index(color_index)),
                    Color::Black => material_score -= pst::get_piece_value(Piece::from_index(piece_index), square, Color::from_index(color_index)),
                }
            }
        }
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
    use crate::evaluation;
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
        assert!(evaluate_material(position) < -800);

        // White is missing a queen - Black to move
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR b KQkq - 0 1").unwrap().position;
        assert!(evaluate_material(position) > 800);

        // Black is missing a knight - White to move
        let position = Board::from_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert!(evaluate_material(position) > 200);

        // Black is missing a knight - Black to move
        let position = Board::from_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap().position;
        assert!(evaluate_material(position) < -200);
    }
    
    #[test]
    fn test_evaluation_symmetry() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 2
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNB1KBNR w KQkq - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 3
        let position = Board::from_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 4
        let position = Board::from_fen("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 5
        let position = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 6
        let position = Board::from_fen("5k2/8/8/8/8/8/8/4K2R w K - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 7
        let position = Board::from_fen("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 8
        let position = Board::from_fen("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 9
        let position = Board::from_fen("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 10
        let position = Board::from_fen("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 11
        let position = Board::from_fen("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 12
        let position = Board::from_fen("4k3/1P6/8/8/8/8/K7/8 w - - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 13
        let position = Board::from_fen("8/P1k5/K7/8/8/8/8/8 w - - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 14
        let position = Board::from_fen("K1k5/8/P7/8/8/8/8/8 w - - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 15
        let position = Board::from_fen("8/k1P5/8/1K6/8/8/8/8 w - - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));

        // position 16
        let position = Board::from_fen("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1").unwrap().position;
        let flipped = position.color_flip();
        assert_eq!(evaluation::evaluate(position), evaluation::evaluate(flipped));
    }
}