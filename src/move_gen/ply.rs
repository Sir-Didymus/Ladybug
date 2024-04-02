use crate::board::piece::Piece;
use crate::board::square::Square;

/// This struct represents a halfmove, also known as [ply](https://www.chessprogramming.org/Ply).
/// 
/// In the comments, I will often refer to a ply as a move, even though a move technically involves
/// both White's and Black's responses. Unless stated otherwise, move and ply mean basically the same in this repository.
/// Moves in the actual sense will be referred to as "fullmove".
pub struct Ply {
    /// The source square.
    pub source: Square,
    /// The target square.
    pub target: Square,
    /// The type of the piece to move.
    pub piece: Piece,
    /// If the move is a capture move, this field will contain the type of the captured piece.
    pub captured_piece: Option<Piece>,
    /// If the move is a pawn promotion, this field will contain the promotion piece.
    pub promotion_piece: Option<Piece>,
}