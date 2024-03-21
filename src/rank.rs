/// This enum represents a rank on a chessboard.
///
/// Can be compared using "==", thanks to the [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) trait.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    First = 0,
    Second = 1,
    Third = 2,
    Fourth = 3,
    Fifth = 4,
    Sixth = 5,
    Seventh = 6,
    Eighth = 7,
}

/// The number of ranks on a chessboard.
pub const NUM_RANKS: u8 = 8;

impl Rank {
    /// Returns the index of the rank, ranging from 0 (rank 1) to 7 (rank 8).
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Returns a rank based on the rank's index.
    pub fn from_index(index: u8) -> Rank {
        match index {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("Invalid index, should be between 0 and 7"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rank::Rank;

    #[test]
    fn to_index_returns_correct_index() {
        assert_eq!(0, Rank::First.to_index());
        assert_eq!(1, Rank::Second.to_index());
        assert_eq!(2, Rank::Third.to_index());
        assert_eq!(3, Rank::Fourth.to_index());
        assert_eq!(4, Rank::Fifth.to_index());
        assert_eq!(5, Rank::Sixth.to_index());
        assert_eq!(6, Rank::Seventh.to_index());
        assert_eq!(7, Rank::Eighth.to_index());
    }

    #[test]
    fn from_index_returns_correct_rank() {
        assert_eq!(Rank::First, Rank::from_index(0));
        assert_eq!(Rank::Second, Rank::from_index(1));
        assert_eq!(Rank::Third, Rank::from_index(2));
        assert_eq!(Rank::Fourth, Rank::from_index(3));
        assert_eq!(Rank::Fifth, Rank::from_index(4));
        assert_eq!(Rank::Sixth, Rank::from_index(5));
        assert_eq!(Rank::Seventh, Rank::from_index(6));
        assert_eq!(Rank::Eighth, Rank::from_index(7));

        assert_ne!(Rank::First, Rank::from_index(5));
        assert_ne!(Rank::Fifth, Rank::from_index(2));
        assert_ne!(Rank::Eighth, Rank::from_index(6));

        assert!(!(Rank::First == Rank::Fifth));
    }

    #[test]
    #[should_panic(expected = "Invalid index, should be between 0 and 7")]
    fn from_index_with_invalid_index_panics() {
        Rank::from_index(9);
    }
}
