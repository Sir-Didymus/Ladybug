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
    
    /// Returns a char based on piece type and color.
    /// The chars are chosen like in the FEN notation, with capital letters for white pieces
    /// and lower case letters for black pieces.
    pub fn to_char(&self, color: Color) -> char {
        match color {
            Color::White => match self {
                Piece::Pawn => 'P',
                Piece::Knight => 'N',
                Piece::Bishop => 'B',
                Piece::Rook => 'R',
                Piece::Queen => 'Q',
                Piece::King => 'K',
            }
            Color::Black => match self {
                Piece::Pawn => 'p',
                Piece::Knight => 'n',
                Piece::Bishop => 'b',
                Piece::Rook => 'r',
                Piece::Queen => 'q',
                Piece::King => 'k',
            }
        }
    }

    /// Constructs a piece from the given string slice.
    pub fn from_string(piece_str: &str) -> Option<Piece> {
        match piece_str.to_ascii_lowercase().as_str() {
            "p" => Some(Piece::Pawn),
            "n" => Some(Piece::Knight),
            "b" => Some(Piece::Bishop),
            "r" => Some(Piece::Rook),
            "q" => Some(Piece::Queen),
            "k" => Some(Piece::King),
            _other => None,
        }
    }
    
    /// Returns true if the piece is a slider piece, otherwise false.
    pub fn is_slider(&self) -> bool {
        match self {
            Piece::Bishop => true,
            Piece::Rook => true,
            Piece::Queen => true,
            _other => false,
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
    fn to_char_returns_correct_char() {
        assert_eq!('P', Pawn.to_char(White));
        assert_eq!('N', Knight.to_char(White));
        assert_eq!('B', Bishop.to_char(White));
        assert_eq!('R', Rook.to_char(White));
        assert_eq!('Q', Queen.to_char(White));
        assert_eq!('K', King.to_char(White));

        assert_eq!('p', Pawn.to_char(Black));
        assert_eq!('n', Knight.to_char(Black));
        assert_eq!('b', Bishop.to_char(Black));
        assert_eq!('r', Rook.to_char(Black));
        assert_eq!('q', Queen.to_char(Black));
        assert_eq!('k', King.to_char(Black));
    }

    #[test]
    fn from_string_with_invalid_string_returns_none() {
        assert_eq!(None, Piece::from_string("test"));
        assert_eq!(None, Piece::from_string("invalid pieces string"));
        assert_eq!(None, Piece::from_string("1"));
        assert_eq!(None, Piece::from_string("p n b r q k"));
    }

    #[test]
    fn from_string_with_valid_string_returns_piece() {
        assert_eq!(Some(Pawn), Piece::from_string("p"));
        assert_eq!(Some(Pawn), Piece::from_string("P"));

        assert_eq!(Some(Knight), Piece::from_string("n"));
        assert_eq!(Some(Knight), Piece::from_string("N"));

        assert_eq!(Some(Bishop), Piece::from_string("b"));
        assert_eq!(Some(Bishop), Piece::from_string("B"));

        assert_eq!(Some(Rook), Piece::from_string("r"));
        assert_eq!(Some(Rook), Piece::from_string("R"));

        assert_eq!(Some(Queen), Piece::from_string("q"));
        assert_eq!(Some(Queen), Piece::from_string("Q"));

        assert_eq!(Some(King), Piece::from_string("k"));
        assert_eq!(Some(King), Piece::from_string("K"));
    }
    
    #[test]
    fn is_slider_returns_correct_bool() {
        assert!(!Pawn.is_slider());
        assert!(!Knight.is_slider());
        assert!(!King.is_slider());
        
        assert!(Bishop.is_slider());
        assert!(Rook.is_slider());
        assert!(Queen.is_slider());
    }
}