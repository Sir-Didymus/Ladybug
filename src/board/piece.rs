/// Represents a piece on a chessboard.
#[derive(Copy, Clone)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    /// Returns the index of the piece.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }
}