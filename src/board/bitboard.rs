use std::fmt::{Display, Formatter};
use crate::board::file::{File, NUM_FILES};
use crate::board::rank::{NUM_RANKS, Rank};
use crate::board::square::{Square};

/// A bitboard representing the state of the board for one type of piece for one color.
///
/// The board representation is as follows: 
/// A1 has an index of 0, and is represented by the least significant bit of the integer.
/// H8 has an index of 63, and is represented by the most significant bit of the integer.
///
/// This mapping is called [Little-Endian Rank-File Mapping](https://www.chessprogramming.org/Square_Mapping_Considerations#Little-Endian_Rank-File_Mapping)
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Bitboard {
    pub value: u64,
}

impl Bitboard {
    /// Constructs a new bitboard from an u64.
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    /// Returns a bitboard with the bit at the specified square set to 1
    pub fn from_square(square: Square) -> Self {
        Self { value: 1 << square.index }
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
    
    /// Returns a list of all the squares that are set to 1.
    /// Implemented using a combination of `trailing_zeros()` and Brian Kernighan's algorithm.
    pub fn get_active_bits(&self) -> Vec<Square> {
        let mut active_bits: Vec<Square> = Vec::new();
        let mut bb_value = self.value;
        while bb_value > 0 {
            active_bits.push(Square::new(bb_value.trailing_zeros() as u8));
            bb_value &= bb_value - 1;
        }
        active_bits
    }
    
    /// Returns the number of active bits.
    /// Implemented using Brian Kernighan's algorithm.
    pub fn get_num_active_bits(&self) -> u8 {
        let mut active_bits = 0;
        let mut bb_value = self.value;
        while bb_value > 0 {
            active_bits += 1;
            bb_value &= bb_value - 1;
        }
        active_bits
    }
}

/// Prints the bitboard with '.' marking empty squares and 'X' marking occupied squares.
/// It also prints the value of the bitboard in hexadecimal.
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
        output += "   a  b  c  d  e  f  g  h\n";
        output += format!("\nValue: 0x{:x}\n", self.value).as_str();
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::square;
    use super::*;

    #[test]
    fn new_constructs_bitboard_with_correct_value() {
        assert_eq!(0, Bitboard::new(0).value);
        assert_eq!(542525, Bitboard::new(542525).value);
        assert_eq!(18446744073709551615, Bitboard::new(18446744073709551615).value);
    }
    
    #[test]
    fn from_square_returns_bitboard_with_correct_bit_set() {
        for i in 0..64 {
            let bitboard = Bitboard::from_square(Square::new(i));
            assert!((1 << i) & bitboard.value > 0); // test that square is set - I avoid using get_square here so the tests are independent of each other
            for j in 0..64 { // test that all other squares are not set
                if i != j {
                    assert!(!(1 << j) & bitboard.value > 0); // test that square is not set
                }
            }
        }
    }

    #[test]
    fn bitboard_formats_correctly() {
        let bitboard = Bitboard::new(0x8000000800000010); // Bitboard with squares h8, d5, e1 occupied
        let expected_output = "8  .  .  .  .  .  .  .  X  \n7  .  .  .  .  .  .  .  .  \n6  .  .  .  .  .  .  .  .  \n5  .  .  .  X  .  .  .  .  \n4  .  .  .  .  .  .  .  .  \n3  .  .  .  .  .  .  .  .  \n2  .  .  .  .  .  .  .  .  \n1  .  .  .  .  X  .  .  .  \n   a  b  c  d  e  f  g  h\n\nValue: 0x8000000800000010\n";
        assert_eq!(expected_output, format!("{}", bitboard));

        let bitboard = Bitboard::new(0x4000020008000041); // Bitboard with squares g8, b6, d4, a1, g1 occupied
        let expected_output = "8  .  .  .  .  .  .  X  .  \n7  .  .  .  .  .  .  .  .  \n6  .  X  .  .  .  .  .  .  \n5  .  .  .  .  .  .  .  .  \n4  .  .  .  X  .  .  .  .  \n3  .  .  .  .  .  .  .  .  \n2  .  .  .  .  .  .  .  .  \n1  X  .  .  .  .  .  X  .  \n   a  b  c  d  e  f  g  h\n\nValue: 0x4000020008000041\n";
        assert_eq!(expected_output, format!("{}", bitboard));

        let bitboard = Bitboard::new(0x4002000000800); // Bitboard with squares c7, f5, d2 occupied
        let expected_output = "8  .  .  .  .  .  .  .  .  \n7  .  .  X  .  .  .  .  .  \n6  .  .  .  .  .  .  .  .  \n5  .  .  .  .  .  X  .  .  \n4  .  .  .  .  .  .  .  .  \n3  .  .  .  .  .  .  .  .  \n2  .  .  .  X  .  .  .  .  \n1  .  .  .  .  .  .  .  .  \n   a  b  c  d  e  f  g  h\n\nValue: 0x4002000000800\n";
        assert_eq!(expected_output, format!("{}", bitboard));

        let bitboard = Bitboard::new(0x8000000400200000); // Bitboard with squares h8, c5, f3 occupied
        let expected_output = "8  .  .  .  .  .  .  .  X  \n7  .  .  .  .  .  .  .  .  \n6  .  .  .  .  .  .  .  .  \n5  .  .  X  .  .  .  .  .  \n4  .  .  .  .  .  .  .  .  \n3  .  .  .  .  .  X  .  .  \n2  .  .  .  .  .  .  .  .  \n1  .  .  .  .  .  .  .  .  \n   a  b  c  d  e  f  g  h\n\nValue: 0x8000000400200000\n";
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
    fn get_active_bits_returns_list_of_squares_with_set_bits() {
        assert_eq!(0, Bitboard::new(0).get_active_bits().len());
        
        let active_bits = Bitboard::new(1).get_active_bits();
        assert_eq!(1, active_bits.len());
        assert_eq!(square::A1, active_bits[0]);
        
        let bitboard = Bitboard::new(0x8000022000000801);
        let active_bits = bitboard.get_active_bits();
        assert_eq!(5, active_bits.len());
        assert_eq!(square::A1, active_bits[0]);
        assert_eq!(square::D2, active_bits[1]);
        assert_eq!(square::F5, active_bits[2]);
        assert_eq!(square::B6, active_bits[3]);
        assert_eq!(square::H8, active_bits[4]);

        let bitboard = Bitboard::new(0x260000002000024);
        let active_bits = bitboard.get_active_bits();
        assert_eq!(6, active_bits.len());
        assert_eq!(square::C1, active_bits[0]);
        assert_eq!(square::F1, active_bits[1]);
        assert_eq!(square::B4, active_bits[2]);
        assert_eq!(square::F7, active_bits[3]);
        assert_eq!(square::G7, active_bits[4]);
        assert_eq!(square::B8, active_bits[5]);
    }
    
    #[test]
    fn get_num_active_bits_returns_number_of_active_bits() {
        assert_eq!(0, Bitboard::new(0).get_num_active_bits());
        assert_eq!(64, Bitboard::new(0xffffffffffffffff).get_num_active_bits());
        assert_eq!(6, Bitboard::new(0x220000210008001).get_num_active_bits());
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