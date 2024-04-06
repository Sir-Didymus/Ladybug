use std::fmt::{Display, Formatter};
use crate::board::piece::Piece;
use crate::board::square::Square;

/// This struct represents a halfmove, also known as [ply](https://www.chessprogramming.org/Ply).
/// 
/// In the comments, I will often refer to a ply as a move, even though a move technically involves
/// both White's and Black's responses. Unless stated otherwise, move and ply mean basically the same in this repository.
/// Moves in the actual sense will be referred to as "fullmove".
#[derive(Copy, Clone, Debug)]
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

/// Prints the ply as text.
impl Display for Ply {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output: String = String::from("");
        output += format!("{}", self.source).as_str();
        output += format!("{}", self.target).as_str();
        write!(f, "{}" ,output)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::Piece;
    use crate::board::square;
    use crate::move_gen::ply::Ply;

    #[test]
    fn ply_formats_correctly() {
        let mut ply = Ply {
            source: square::E2,
            target: square::E4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        };
        assert_eq!("e2e4", format!("{ply}"));
        
        ply.source = square::A4;
        ply.target = square::E8;
        assert_eq!("a4e8", format!("{ply}"));

        ply.source = square::H3;
        ply.target = square::H6;
        assert_eq!("h3h6", format!("{ply}"));
    }
}