use crate::board::color::Color;

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
    
    /// Returns a unicode character based on piece type and color.
    pub fn to_char(&self, color: Color) -> char {
        match color {
            Color::White => match self {
                Piece::Pawn => '\u{2659}',
                Piece::Knight => '\u{2658}',
                Piece::Bishop => '\u{2657}',
                Piece::Rook => '\u{2656}',
                Piece::Queen => '\u{2655}',
                Piece::King => '\u{2654}',
            }
            Color::Black => match self {
                Piece::Pawn => '\u{265F}',
                Piece::Knight => '\u{265E}',
                Piece::Bishop => '\u{265D}',
                Piece::Rook => '\u{265C}',
                Piece::Queen => '\u{265B}',
                Piece::King => '\u{265A}',
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::color::Color::{Black, White};
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
    
    #[test]
    fn to_char_returns_correct_unicode_char() {
        assert_eq!('\u{2659}', Pawn.to_char(White));
        assert_eq!('\u{2658}', Knight.to_char(White));
        assert_eq!('\u{2657}', Bishop.to_char(White));
        assert_eq!('\u{2656}', Rook.to_char(White));
        assert_eq!('\u{2655}', Queen.to_char(White));
        assert_eq!('\u{2654}', King.to_char(White));

        assert_eq!('\u{265F}', Pawn.to_char(Black));
        assert_eq!('\u{265E}', Knight.to_char(Black));
        assert_eq!('\u{265D}', Bishop.to_char(Black));
        assert_eq!('\u{265C}', Rook.to_char(Black));
        assert_eq!('\u{265B}', Queen.to_char(Black));
        assert_eq!('\u{265A}', King.to_char(Black));
    }
}