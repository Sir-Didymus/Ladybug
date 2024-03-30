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
}

#[cfg(test)]
mod tests {
    use crate::board::color::Color;

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
}