use std::fmt::{Display, Formatter};
use crate::file::{File, NUM_FILES};
use crate::rank::{NUM_RANKS, Rank};
use crate::square::{Square};

/// A bitboard representing the state of the board for one type of piece for one color.
///
/// The board representation is as follows: 
/// A1 has an index of 0, and is represented by the least significant bit of the integer.
/// H8 has an index of 63, and is represented by the most significant bit of the integer.
///
/// This mapping is called [Little-Endian Rank-File Mapping](https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian_Rank-File_Mapping)
pub struct Bitboard {
    pub value: u64,
}

impl Bitboard {
    /// Constructs a new Bitboard from an u64.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns true if the bit at the specified square is set to 1, false if not.
    pub fn get_bit(&self, square: Square) -> bool {
        (1 << square.index) & self.value > 0
    }

    /// Sets the bit at the specified square to 1
    pub fn set_bit(&mut self, square: Square) {
        self.value |= 1 << square.index;
    }

    /// Sets the bit at the specified square to 0
    pub fn pop_bit(&mut self, square: Square) { 
        if self.get_bit(square) {
            self.value ^= 1 << square.index;
        }
    }
}

/// Prints the bitboard with '.' marking empty squares and 'X' marking occupied squares.
impl Display for Bitboard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output: String = String::from("");
        for rank in (0..NUM_RANKS).rev() {
            output += format!("{}  ", rank + 1).as_str();
            for file in 0..NUM_FILES {
                if self.get_bit(Square::from_file_rank(File::from_index(file), Rank::from_index(rank))) {
                    output += "X  ";
                } else {
                    output += ".  ";
                }
            }
            output += "\n";
        }
        output += "   A  B  C  D  E  F  G  H";
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_constructs_bitboard_with_correct_value() {
        assert_eq!(0, Bitboard::new(0).value);
        assert_eq!(542525, Bitboard::new(542525).value);
        assert_eq!(18446744073709551615, Bitboard::new(18446744073709551615).value);
    }

    #[test]
    fn bitboard_formats_correctly() {
        let bitboard = Bitboard::new(9223372071214514192); // Bitboard with squares h8, d5, e1 occupied
        let expected_output = "8  .  .  .  .  .  .  .  X  \n7  .  .  .  .  .  .  .  .  \n6  .  .  .  .  .  .  .  .  \n5  .  .  .  X  .  .  .  .  \n4  .  .  .  .  .  .  .  .  \n3  .  .  .  .  .  .  .  .  \n2  .  .  .  .  .  .  .  .  \n1  .  .  .  .  X  .  .  .  \n   A  B  C  D  E  F  G  H";
        assert_eq!(expected_output, format!("{}", bitboard));

        let bitboard = Bitboard::new(4611688217584861249); // Bitboard with squares g8, b6, d4, a1, g1 occupied
        let expected_output = "8  .  .  .  .  .  .  X  .  \n7  .  .  .  .  .  .  .  .  \n6  .  X  .  .  .  .  .  .  \n5  .  .  .  .  .  .  .  .  \n4  .  .  .  X  .  .  .  .  \n3  .  .  .  .  .  .  .  .  \n2  .  .  .  .  .  .  .  .  \n1  X  .  .  .  .  .  X  .  \n   A  B  C  D  E  F  G  H";
        assert_eq!(expected_output, format!("{}", bitboard));

        let bitboard = Bitboard::new(1126037345798144); // Bitboard with squares c7, f5, d2 occupied
        let expected_output = "8  .  .  .  .  .  .  .  .  \n7  .  .  X  .  .  .  .  .  \n6  .  .  .  .  .  .  .  .  \n5  .  .  .  .  .  X  .  .  \n4  .  .  .  .  .  .  .  .  \n3  .  .  .  .  .  .  .  .  \n2  .  .  .  X  .  .  .  .  \n1  .  .  .  .  .  .  .  .  \n   A  B  C  D  E  F  G  H";
        assert_eq!(expected_output, format!("{}", bitboard));

        let bitboard = Bitboard::new(9223372054036742144); // Bitboard with squares h8, c5, f3 occupied
        let expected_output = "8  .  .  .  .  .  .  .  X  \n7  .  .  .  .  .  .  .  .  \n6  .  .  .  .  .  .  .  .  \n5  .  .  X  .  .  .  .  .  \n4  .  .  .  .  .  .  .  .  \n3  .  .  .  .  .  X  .  .  \n2  .  .  .  .  .  .  .  .  \n1  .  .  .  .  .  .  .  .  \n   A  B  C  D  E  F  G  H";
        assert_eq!(expected_output, format!("{}", bitboard));
    }

    #[test]
    fn set_bit_sets_bit_at_correct_square() {
        for i in 0..64 {
            let mut bitboard = Bitboard::new(0);
            bitboard.set_bit(Square::new(i));
            assert!((1 << i) & bitboard.value > 0); // test that square is set - I avoid using get_square here so the tests are independent of each other
            for j in 0..64 { // test that all other squares are not set
                if i != j {
                    assert!(!(1 << j) & bitboard.value > 0); // test that square is not set
                }
            }
        }
    }

    #[test]
    fn set_bit_on_square_that_is_already_set_square_is_still_set() {
        for i in 0..64 {
            let mut bitboard = Bitboard::new(0);
            bitboard.set_bit(Square::new(i));
            bitboard.set_bit(Square::new(i));
            assert!((1 << i) & bitboard.value > 0); // test that square is set - I avoid using get_square here so the tests are independent of each other
        }
    }

    #[test]
    fn pop_bit_unsets_bit_at_correct_square() {
        for i in 0..64 {
            let mut bitboard = Bitboard::new(0xffffffffffffffff); // bitboard with all bits set
            bitboard.pop_bit(Square::new(i));
            assert_eq!(0, (1 << i) & bitboard.value); // test that square is unset - I avoid using get_square here so the tests are independent of each other
        }
    }

    #[test]
    fn pop_bit_on_square_that_is_already_unset_square_is_still_unset() {
        for i in 0..64 {
            let mut bitboard = Bitboard::new(0);
            bitboard.pop_bit(Square::new(i));
            assert_eq!(0, (1 << i) & bitboard.value); // test that square is unset - I avoid using get_square here so the tests are independent of each other
        }
    }

    #[test]
    fn get_bit_returns_bit_at_correct_square() {
        // test every square, initializing the bitboard using hexadecimal, so it is shorter to write
        assert!(Bitboard::new(0x1).get_bit(Square::from_file_rank(File::A, Rank::First)));
        assert!(Bitboard::new(0x2).get_bit(Square::from_file_rank(File::B, Rank::First)));
        assert!(Bitboard::new(0x4).get_bit(Square::from_file_rank(File::C, Rank::First)));
        assert!(Bitboard::new(0x8).get_bit(Square::from_file_rank(File::D, Rank::First)));
        assert!(Bitboard::new(0x10).get_bit(Square::from_file_rank(File::E, Rank::First)));
        assert!(Bitboard::new(0x20).get_bit(Square::from_file_rank(File::F, Rank::First)));
        assert!(Bitboard::new(0x40).get_bit(Square::from_file_rank(File::G, Rank::First)));
        assert!(Bitboard::new(0x80).get_bit(Square::from_file_rank(File::H, Rank::First)));

        assert!(Bitboard::new(0x100).get_bit(Square::from_file_rank(File::A, Rank::Second)));
        assert!(Bitboard::new(0x200).get_bit(Square::from_file_rank(File::B, Rank::Second)));
        assert!(Bitboard::new(0x400).get_bit(Square::from_file_rank(File::C, Rank::Second)));
        assert!(Bitboard::new(0x800).get_bit(Square::from_file_rank(File::D, Rank::Second)));
        assert!(Bitboard::new(0x1000).get_bit(Square::from_file_rank(File::E, Rank::Second)));
        assert!(Bitboard::new(0x2000).get_bit(Square::from_file_rank(File::F, Rank::Second)));
        assert!(Bitboard::new(0x4000).get_bit(Square::from_file_rank(File::G, Rank::Second)));
        assert!(Bitboard::new(0x8000).get_bit(Square::from_file_rank(File::H, Rank::Second)));

        assert!(Bitboard::new(0x10000).get_bit(Square::from_file_rank(File::A, Rank::Third)));
        assert!(Bitboard::new(0x20000).get_bit(Square::from_file_rank(File::B, Rank::Third)));
        assert!(Bitboard::new(0x40000).get_bit(Square::from_file_rank(File::C, Rank::Third)));
        assert!(Bitboard::new(0x80000).get_bit(Square::from_file_rank(File::D, Rank::Third)));
        assert!(Bitboard::new(0x100000).get_bit(Square::from_file_rank(File::E, Rank::Third)));
        assert!(Bitboard::new(0x200000).get_bit(Square::from_file_rank(File::F, Rank::Third)));
        assert!(Bitboard::new(0x400000).get_bit(Square::from_file_rank(File::G, Rank::Third)));
        assert!(Bitboard::new(0x800000).get_bit(Square::from_file_rank(File::H, Rank::Third)));

        assert!(Bitboard::new(0x1000000).get_bit(Square::from_file_rank(File::A, Rank::Fourth)));
        assert!(Bitboard::new(0x2000000).get_bit(Square::from_file_rank(File::B, Rank::Fourth)));
        assert!(Bitboard::new(0x4000000).get_bit(Square::from_file_rank(File::C, Rank::Fourth)));
        assert!(Bitboard::new(0x8000000).get_bit(Square::from_file_rank(File::D, Rank::Fourth)));
        assert!(Bitboard::new(0x10000000).get_bit(Square::from_file_rank(File::E, Rank::Fourth)));
        assert!(Bitboard::new(0x20000000).get_bit(Square::from_file_rank(File::F, Rank::Fourth)));
        assert!(Bitboard::new(0x40000000).get_bit(Square::from_file_rank(File::G, Rank::Fourth)));
        assert!(Bitboard::new(0x80000000).get_bit(Square::from_file_rank(File::H, Rank::Fourth)));

        assert!(Bitboard::new(0x100000000).get_bit(Square::from_file_rank(File::A, Rank::Fifth)));
        assert!(Bitboard::new(0x200000000).get_bit(Square::from_file_rank(File::B, Rank::Fifth)));
        assert!(Bitboard::new(0x400000000).get_bit(Square::from_file_rank(File::C, Rank::Fifth)));
        assert!(Bitboard::new(0x800000000).get_bit(Square::from_file_rank(File::D, Rank::Fifth)));
        assert!(Bitboard::new(0x1000000000).get_bit(Square::from_file_rank(File::E, Rank::Fifth)));
        assert!(Bitboard::new(0x2000000000).get_bit(Square::from_file_rank(File::F, Rank::Fifth)));
        assert!(Bitboard::new(0x4000000000).get_bit(Square::from_file_rank(File::G, Rank::Fifth)));
        assert!(Bitboard::new(0x8000000000).get_bit(Square::from_file_rank(File::H, Rank::Fifth)));

        assert!(Bitboard::new(0x10000000000).get_bit(Square::from_file_rank(File::A, Rank::Sixth)));
        assert!(Bitboard::new(0x20000000000).get_bit(Square::from_file_rank(File::B, Rank::Sixth)));
        assert!(Bitboard::new(0x40000000000).get_bit(Square::from_file_rank(File::C, Rank::Sixth)));
        assert!(Bitboard::new(0x80000000000).get_bit(Square::from_file_rank(File::D, Rank::Sixth)));
        assert!(Bitboard::new(0x100000000000).get_bit(Square::from_file_rank(File::E, Rank::Sixth)));
        assert!(Bitboard::new(0x200000000000).get_bit(Square::from_file_rank(File::F, Rank::Sixth)));
        assert!(Bitboard::new(0x400000000000).get_bit(Square::from_file_rank(File::G, Rank::Sixth)));
        assert!(Bitboard::new(0x800000000000).get_bit(Square::from_file_rank(File::H, Rank::Sixth)));

        assert!(Bitboard::new(0x1000000000000).get_bit(Square::from_file_rank(File::A, Rank::Seventh)));
        assert!(Bitboard::new(0x2000000000000).get_bit(Square::from_file_rank(File::B, Rank::Seventh)));
        assert!(Bitboard::new(0x4000000000000).get_bit(Square::from_file_rank(File::C, Rank::Seventh)));
        assert!(Bitboard::new(0x8000000000000).get_bit(Square::from_file_rank(File::D, Rank::Seventh)));
        assert!(Bitboard::new(0x10000000000000).get_bit(Square::from_file_rank(File::E, Rank::Seventh)));
        assert!(Bitboard::new(0x20000000000000).get_bit(Square::from_file_rank(File::F, Rank::Seventh)));
        assert!(Bitboard::new(0x40000000000000).get_bit(Square::from_file_rank(File::G, Rank::Seventh)));
        assert!(Bitboard::new(0x80000000000000).get_bit(Square::from_file_rank(File::H, Rank::Seventh)));

        assert!(Bitboard::new(0x100000000000000).get_bit(Square::from_file_rank(File::A, Rank::Eighth)));
        assert!(Bitboard::new(0x200000000000000).get_bit(Square::from_file_rank(File::B, Rank::Eighth)));
        assert!(Bitboard::new(0x400000000000000).get_bit(Square::from_file_rank(File::C, Rank::Eighth)));
        assert!(Bitboard::new(0x800000000000000).get_bit(Square::from_file_rank(File::D, Rank::Eighth)));
        assert!(Bitboard::new(0x1000000000000000).get_bit(Square::from_file_rank(File::E, Rank::Eighth)));
        assert!(Bitboard::new(0x2000000000000000).get_bit(Square::from_file_rank(File::F, Rank::Eighth)));
        assert!(Bitboard::new(0x4000000000000000).get_bit(Square::from_file_rank(File::G, Rank::Eighth)));
        assert!(Bitboard::new(0x8000000000000000).get_bit(Square::from_file_rank(File::H, Rank::Eighth)));
    }
}