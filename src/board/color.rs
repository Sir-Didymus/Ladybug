/// The two colors in the game of chess.
#[derive(Copy, Clone)]
pub enum Color {
    White,
    Black,
}

/// The number of colors.
pub const NUM_COLORS: u8 = 2;

impl Color {
    /// Returns the index of the color: 0 for White and 1 for Black.
    pub fn to_index(&self) -> u8 {
        *self as u8
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
}