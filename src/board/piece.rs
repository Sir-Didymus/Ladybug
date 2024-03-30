/// Represents a piece on a chessboard.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

/// The number of different pieces.
pub const NUM_PIECES: u8 = 6;

impl Piece {
    /// Returns the index of the piece.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Constructs a piece based on the piece's index.
    pub fn from_index(index: u8) -> Piece {
        match index % 6 {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::Piece;
    use crate::board::piece::Piece::*;

    #[test]
    fn to_index_returns_correct_index() {
        assert_eq!(0, Pawn.to_index());
        assert_eq!(1, Knight.to_index());
        assert_eq!(2, Bishop.to_index());
        assert_eq!(3, Rook.to_index());
        assert_eq!(4, Queen.to_index());
        assert_eq!(5, King.to_index());
    }

    #[test]
    fn from_index_with_valid_index_returns_piece() {
        assert_eq!(Pawn, Piece::from_index(0));
        assert_eq!(Knight, Piece::from_index(1));
        assert_eq!(Bishop, Piece::from_index(2));
        assert_eq!(Rook, Piece::from_index(3));
        assert_eq!(Queen, Piece::from_index(4));
        assert_eq!(King, Piece::from_index(5));
    }

    #[test]
    fn from_index_with_invalid_index_wraps_around() {
        assert_eq!(Pawn, Piece::from_index(6));
        assert_eq!(Knight, Piece::from_index(7));
        assert_eq!(Bishop, Piece::from_index(8));
        assert_eq!(Rook, Piece::from_index(9));
        assert_eq!(Queen, Piece::from_index(10));
        assert_eq!(King, Piece::from_index(11));
        assert_eq!(Pawn, Piece::from_index(12));
    }
}