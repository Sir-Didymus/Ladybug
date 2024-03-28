//! This module deals with basic board representation.
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

/// The board struct holds the current position of the board.
/// It also keeps track of the half-move (ply) history, the full move count, the number of reversible half-moves (50 move draw),
/// and a list of all positions that have been on the board before (threefold repetition).
#[derive(Copy, Clone)]
pub struct Board {
    position: Position,
    full_move_count: u16,
    reversible_ply_count: u16,
}
