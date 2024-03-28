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
/// It also keeps track of the half-move (ply) history, the full move count, the number of reversible half-moves (50 move rule),
/// and a list of all positions that have been on the board before (threefold repetition).
#[derive(Copy, Clone)]
pub struct Board {
    pub position: Position,
    pub full_move_count: u16,
    pub reversible_ply_count: u16,
}

impl Default for Board {
    /// Default constructor for Board.
    /// Returns a board with the default position, and all other values set to zero.
    fn default() -> Self {
        Self {
            position: Position::default(),
            full_move_count: 0,
            reversible_ply_count: 0,
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
        assert_eq!(0, board.full_move_count);
        assert_eq!(0, board.reversible_ply_count);
    }
}
