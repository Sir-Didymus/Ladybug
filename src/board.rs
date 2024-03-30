//! The board module deals with basic board representation.
//! It contains the important position and bitboard submodules, as well other useful ones such as color, file, rank, and square.
//! This module is the foundation on which the rest of the engine builds upon.

use position::Position;

pub mod bitboard;
pub mod color;
pub mod file;
pub mod rank;
pub mod square;
pub mod castling_rights;
pub mod piece;
pub mod position;
pub mod fen;

/// The board struct holds the current position of the board.
/// It also keeps track of the half-move (ply) history, the full move counter, the number of reversible half-moves (50 move rule),
/// and a list of all positions that have been on the board before (threefold repetition).
#[derive(Copy, Clone)]
pub struct Board {
    /// The current position of the chess board.
    pub position: Position,
    /// The current full move count (incremented after Black's play).
    pub fullmove_counter: u32,
    /// The number of reversible ply (no pawn moves or captures).
    pub halfmove_clock: u32,
}

impl Default for Board {
    /// Default constructor for Board.
    /// Returns a board with default values.
    fn default() -> Self {
        Self {
            position: Position::default(),
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::position::Position;

    #[test]
    fn default_returns_board_with_default_values() {
        let board = Board::default();
        assert_eq!(Position::default(), board.position);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(1, board.fullmove_counter);
    }
}
