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

    /// Returns true if the given square is set to 1, false if not.
    pub fn is_square_set(&self, square: Square) -> bool {
        (1 << square.index) & self.value > 0
    }
}

/// Prints the bitboard with '.' marking empty squares and 'X' marking occupied squares.
impl Display for Bitboard {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output: String = String::from("");
        for rank in (0..NUM_RANKS).rev() {
            output += format!("{}  ", rank + 1).as_str();
            for file in 0..NUM_FILES {
                if self.is_square_set(Square::from_file_rank(File::from_index(file), Rank::from_index(rank))) {
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
    fn is_square_set_returns_correct_state_of_square() {
        // test every square, initializing the bitboard using hexadecimal, so it is shorter to write
        assert!(Bitboard::new(0x1).is_square_set(Square::from_file_rank(File::A, Rank::First)));
        assert!(Bitboard::new(0x2).is_square_set(Square::from_file_rank(File::B, Rank::First)));
        assert!(Bitboard::new(0x4).is_square_set(Square::from_file_rank(File::C, Rank::First)));
        assert!(Bitboard::new(0x8).is_square_set(Square::from_file_rank(File::D, Rank::First)));
        assert!(Bitboard::new(0x10).is_square_set(Square::from_file_rank(File::E, Rank::First)));
        assert!(Bitboard::new(0x20).is_square_set(Square::from_file_rank(File::F, Rank::First)));
        assert!(Bitboard::new(0x40).is_square_set(Square::from_file_rank(File::G, Rank::First)));
        assert!(Bitboard::new(0x80).is_square_set(Square::from_file_rank(File::H, Rank::First)));

        assert!(Bitboard::new(0x100).is_square_set(Square::from_file_rank(File::A, Rank::Second)));
        assert!(Bitboard::new(0x200).is_square_set(Square::from_file_rank(File::B, Rank::Second)));
        assert!(Bitboard::new(0x400).is_square_set(Square::from_file_rank(File::C, Rank::Second)));
        assert!(Bitboard::new(0x800).is_square_set(Square::from_file_rank(File::D, Rank::Second)));
        assert!(Bitboard::new(0x1000).is_square_set(Square::from_file_rank(File::E, Rank::Second)));
        assert!(Bitboard::new(0x2000).is_square_set(Square::from_file_rank(File::F, Rank::Second)));
        assert!(Bitboard::new(0x4000).is_square_set(Square::from_file_rank(File::G, Rank::Second)));
        assert!(Bitboard::new(0x8000).is_square_set(Square::from_file_rank(File::H, Rank::Second)));

        assert!(Bitboard::new(0x10000).is_square_set(Square::from_file_rank(File::A, Rank::Third)));
        assert!(Bitboard::new(0x20000).is_square_set(Square::from_file_rank(File::B, Rank::Third)));
        assert!(Bitboard::new(0x40000).is_square_set(Square::from_file_rank(File::C, Rank::Third)));
        assert!(Bitboard::new(0x80000).is_square_set(Square::from_file_rank(File::D, Rank::Third)));
        assert!(Bitboard::new(0x100000).is_square_set(Square::from_file_rank(File::E, Rank::Third)));
        assert!(Bitboard::new(0x200000).is_square_set(Square::from_file_rank(File::F, Rank::Third)));
        assert!(Bitboard::new(0x400000).is_square_set(Square::from_file_rank(File::G, Rank::Third)));
        assert!(Bitboard::new(0x800000).is_square_set(Square::from_file_rank(File::H, Rank::Third)));

        assert!(Bitboard::new(0x1000000).is_square_set(Square::from_file_rank(File::A, Rank::Fourth)));
        assert!(Bitboard::new(0x2000000).is_square_set(Square::from_file_rank(File::B, Rank::Fourth)));
        assert!(Bitboard::new(0x4000000).is_square_set(Square::from_file_rank(File::C, Rank::Fourth)));
        assert!(Bitboard::new(0x8000000).is_square_set(Square::from_file_rank(File::D, Rank::Fourth)));
        assert!(Bitboard::new(0x10000000).is_square_set(Square::from_file_rank(File::E, Rank::Fourth)));
        assert!(Bitboard::new(0x20000000).is_square_set(Square::from_file_rank(File::F, Rank::Fourth)));
        assert!(Bitboard::new(0x40000000).is_square_set(Square::from_file_rank(File::G, Rank::Fourth)));
        assert!(Bitboard::new(0x80000000).is_square_set(Square::from_file_rank(File::H, Rank::Fourth)));

        assert!(Bitboard::new(0x100000000).is_square_set(Square::from_file_rank(File::A, Rank::Fifth)));
        assert!(Bitboard::new(0x200000000).is_square_set(Square::from_file_rank(File::B, Rank::Fifth)));
        assert!(Bitboard::new(0x400000000).is_square_set(Square::from_file_rank(File::C, Rank::Fifth)));
        assert!(Bitboard::new(0x800000000).is_square_set(Square::from_file_rank(File::D, Rank::Fifth)));
        assert!(Bitboard::new(0x1000000000).is_square_set(Square::from_file_rank(File::E, Rank::Fifth)));
        assert!(Bitboard::new(0x2000000000).is_square_set(Square::from_file_rank(File::F, Rank::Fifth)));
        assert!(Bitboard::new(0x4000000000).is_square_set(Square::from_file_rank(File::G, Rank::Fifth)));
        assert!(Bitboard::new(0x8000000000).is_square_set(Square::from_file_rank(File::H, Rank::Fifth)));

        assert!(Bitboard::new(0x10000000000).is_square_set(Square::from_file_rank(File::A, Rank::Sixth)));
        assert!(Bitboard::new(0x20000000000).is_square_set(Square::from_file_rank(File::B, Rank::Sixth)));
        assert!(Bitboard::new(0x40000000000).is_square_set(Square::from_file_rank(File::C, Rank::Sixth)));
        assert!(Bitboard::new(0x80000000000).is_square_set(Square::from_file_rank(File::D, Rank::Sixth)));
        assert!(Bitboard::new(0x100000000000).is_square_set(Square::from_file_rank(File::E, Rank::Sixth)));
        assert!(Bitboard::new(0x200000000000).is_square_set(Square::from_file_rank(File::F, Rank::Sixth)));
        assert!(Bitboard::new(0x400000000000).is_square_set(Square::from_file_rank(File::G, Rank::Sixth)));
        assert!(Bitboard::new(0x800000000000).is_square_set(Square::from_file_rank(File::H, Rank::Sixth)));

        assert!(Bitboard::new(0x1000000000000).is_square_set(Square::from_file_rank(File::A, Rank::Seventh)));
        assert!(Bitboard::new(0x2000000000000).is_square_set(Square::from_file_rank(File::B, Rank::Seventh)));
        assert!(Bitboard::new(0x4000000000000).is_square_set(Square::from_file_rank(File::C, Rank::Seventh)));
        assert!(Bitboard::new(0x8000000000000).is_square_set(Square::from_file_rank(File::D, Rank::Seventh)));
        assert!(Bitboard::new(0x10000000000000).is_square_set(Square::from_file_rank(File::E, Rank::Seventh)));
        assert!(Bitboard::new(0x20000000000000).is_square_set(Square::from_file_rank(File::F, Rank::Seventh)));
        assert!(Bitboard::new(0x40000000000000).is_square_set(Square::from_file_rank(File::G, Rank::Seventh)));
        assert!(Bitboard::new(0x80000000000000).is_square_set(Square::from_file_rank(File::H, Rank::Seventh)));

        assert!(Bitboard::new(0x100000000000000).is_square_set(Square::from_file_rank(File::A, Rank::Eighth)));
        assert!(Bitboard::new(0x200000000000000).is_square_set(Square::from_file_rank(File::B, Rank::Eighth)));
        assert!(Bitboard::new(0x400000000000000).is_square_set(Square::from_file_rank(File::C, Rank::Eighth)));
        assert!(Bitboard::new(0x800000000000000).is_square_set(Square::from_file_rank(File::D, Rank::Eighth)));
        assert!(Bitboard::new(0x1000000000000000).is_square_set(Square::from_file_rank(File::E, Rank::Eighth)));
        assert!(Bitboard::new(0x2000000000000000).is_square_set(Square::from_file_rank(File::F, Rank::Eighth)));
        assert!(Bitboard::new(0x4000000000000000).is_square_set(Square::from_file_rank(File::G, Rank::Eighth)));
        assert!(Bitboard::new(0x8000000000000000).is_square_set(Square::from_file_rank(File::H, Rank::Eighth)));
    }
}