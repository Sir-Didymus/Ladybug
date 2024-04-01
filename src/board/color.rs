use std::fmt::{Display, Formatter};
use crate::board::rank::Rank;

/// The two colors in the game of chess.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    White = 0,
    Black = 1,
}

/// The number of colors.
pub const NUM_COLORS: u8 = 2;

impl Color {
    /// Returns the index of the color.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }
    
    /// Returns a color based on the color's index.
    pub fn from_index(index: u8) -> Color {
        match index % 2 {
            0 => Color::White,
            1 => Color::Black,
            _ => unreachable!(),
        }
    }
    
    /// Returns the promotion rank of the color.
    pub fn promotion_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Eighth,
            Color::Black => Rank::First,
        }
    }
    
    /// Returns the rank of the pawn starting positions for the color.
    pub fn pawn_rank(&self) -> Rank {
        match self {
            Color::White => Rank::Second,
            Color::Black => Rank::Seventh,
        }
    }
}

/// Prints the color as text.
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::color::Color;
    use crate::board::rank::Rank;

    #[test]
    fn to_index_returns_correct_index() {
        assert_eq!(0, Color::White.to_index());
        assert_eq!(1, Color::Black.to_index());
    }

    #[test]
    fn from_index_with_valid_index_returns_color() {
        assert_eq!(Color::White, Color::from_index(0));
        assert_eq!(Color::Black, Color::from_index(1));
    }
    
    #[test]
    fn from_index_with_invalid_index_wraps_around() {
        assert_eq!(Color::White, Color::from_index(2));
        assert_eq!(Color::Black, Color::from_index(3));
    }
    
    #[test]
    fn promotion_rank_returns_correct_rank() {
        assert_eq!(Rank::Eighth, Color::White.promotion_rank());
        assert_eq!(Rank::First, Color::Black.promotion_rank());
    }
    
    #[test]
    fn pawn_rank_returns_correct_rank() {
        assert_eq!(Rank::Second, Color::White.pawn_rank());
        assert_eq!(Rank::Seventh, Color::Black.pawn_rank());
    }
    
    #[test]
    fn color_formats_correctly() {
        assert_eq!("White", format!("{}", Color::White));
        assert_eq!("Black", format!("{}", Color::Black));
    }
}