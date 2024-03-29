use crate::board::file::File;
use crate::board::rank::Rank;

/// A square on the chessboard, represented by an index ranging from 0 to 63.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Square {
    pub index: u8,
}

/// The number of squares on a chessboard.
pub const NUM_SQUARES: u8 = 64;

impl Square {
    /// Constructs a new square from a given index.
    pub fn new(index: u8) -> Self {
        Self { index }
    }

    /// Constructs a new square from a string.
    pub fn from_string(square_str: &str) -> Result<Self, String> {
        // square_str can not be longer than 2
        if square_str.len() != 2 {
            return Err(String::from("Invalid square string"));
        }
        
        // get chars
        let chars: Vec<char> = square_str.chars().collect();
        
        // first char must be ascii a-h
        match chars[0] {
            file_char if file_char.is_ascii_lowercase() && ('a'..='h').contains(&file_char) => {}
            _other => return Err(String::from("Invalid square string")) 
        }
        
        // second char must be a number 1-8
        match chars[1] {
            rank_char if rank_char.is_numeric() && ('1'..='8').contains(&rank_char) => {}
            _other => return Err(String::from("Invalid square string"))
        }

        // get file
        let file = File::from_char(&chars[0]);
        if file.is_err() {
            return Err(String::from("Invalid square string"));
        }
        let file = file.unwrap();
        
        // get rank
        let rank = chars[1].to_digit(10);
        if rank.is_none() {
            return Err(String::from("Invalid square string"));
        }
        let rank = Rank::from_index(rank.unwrap() as u8 - 1);
        
        // get square
        Ok(Square::from_file_rank(file, rank))
    }

    /// Returns the file of the square.
    pub fn get_file(&self) -> File {
        File::from_index(self.index % 8)
    }

    /// Returns the rank of the square.
    pub fn get_rank(&self) -> Rank {
        Rank::from_index(self.index / 8)
    }

    /// Constructs a new square based on rank and file
    pub fn from_file_rank(file: File, rank: Rank) -> Self {
        Self { index : 8*rank.to_index() + file.to_index() }
    }
}

pub const A1: Square = Square { index: 0 };
pub const B1: Square = Square { index: 1 };
pub const C1: Square = Square { index: 2 };
pub const D1: Square = Square { index: 3 };
pub const E1: Square = Square { index: 4 };
pub const F1: Square = Square { index: 5 };
pub const G1: Square = Square { index: 6 };
pub const H1: Square = Square { index: 7 };
pub const A2: Square = Square { index: 8 };
pub const B2: Square = Square { index: 9 };
pub const C2: Square = Square { index: 10 };
pub const D2: Square = Square { index: 11 };
pub const E2: Square = Square { index: 12 };
pub const F2: Square = Square { index: 13 };
pub const G2: Square = Square { index: 14 };
pub const H2: Square = Square { index: 15 };
pub const A3: Square = Square { index: 16 };
pub const B3: Square = Square { index: 17 };
pub const C3: Square = Square { index: 18 };
pub const D3: Square = Square { index: 19 };
pub const E3: Square = Square { index: 20 };
pub const F3: Square = Square { index: 21 };
pub const G3: Square = Square { index: 22 };
pub const H3: Square = Square { index: 23 };
pub const A4: Square = Square { index: 24 };
pub const B4: Square = Square { index: 25 };
pub const C4: Square = Square { index: 26 };
pub const D4: Square = Square { index: 27 };
pub const E4: Square = Square { index: 28 };
pub const F4: Square = Square { index: 29 };
pub const G4: Square = Square { index: 30 };
pub const H4: Square = Square { index: 31 };
pub const A5: Square = Square { index: 32 };
pub const B5: Square = Square { index: 33 };
pub const C5: Square = Square { index: 34 };
pub const D5: Square = Square { index: 35 };
pub const E5: Square = Square { index: 36 };
pub const F5: Square = Square { index: 37 };
pub const G5: Square = Square { index: 38 };
pub const H5: Square = Square { index: 39 };
pub const A6: Square = Square { index: 40 };
pub const B6: Square = Square { index: 41 };
pub const C6: Square = Square { index: 42 };
pub const D6: Square = Square { index: 43 };
pub const E6: Square = Square { index: 44 };
pub const F6: Square = Square { index: 45 };
pub const G6: Square = Square { index: 46 };
pub const H6: Square = Square { index: 47 };
pub const A7: Square = Square { index: 48 };
pub const B7: Square = Square { index: 49 };
pub const C7: Square = Square { index: 50 };
pub const D7: Square = Square { index: 51 };
pub const E7: Square = Square { index: 52 };
pub const F7: Square = Square { index: 53 };
pub const G7: Square = Square { index: 54 };
pub const H7: Square = Square { index: 55 };
pub const A8: Square = Square { index: 56 };
pub const B8: Square = Square { index: 57 };
pub const C8: Square = Square { index: 58 };
pub const D8: Square = Square { index: 59 };
pub const E8: Square = Square { index: 60 };
pub const F8: Square = Square { index: 61 };
pub const G8: Square = Square { index: 62 };
pub const H8: Square = Square { index: 63 };

#[cfg(test)]
mod tests {
    use crate::board::file::File;
    use crate::board::rank::Rank;
    use super::*;

    #[test]
    fn new_constructs_square_with_correct_index() {
        assert_eq!(0, Square::new(0).index);
        assert_eq!(30, Square::new(30).index);
        assert_eq!(54, Square::new(54).index);
        assert_eq!(63, Square::new(63).index);
    }


    #[test]
    fn from_string_with_valid_string_returns_square() {
        assert_eq!(A1, Square::from_string("a1").unwrap());
        assert_eq!(A4, Square::from_string("a4").unwrap());
        assert_eq!(H8, Square::from_string("h8").unwrap());
        assert_eq!(E3, Square::from_string("e3").unwrap());
        assert_eq!(G6, Square::from_string("g6").unwrap());
        assert_eq!(H1, Square::from_string("h1").unwrap());
        assert_eq!(F3, Square::from_string("f3").unwrap());
        assert_eq!(B8, Square::from_string("b8").unwrap());
    }
    
    #[test]
    fn from_string_with_invalid_string_returns_error() {
        assert_eq!(Err(String::from("Invalid square string")), Square::from_string("ab2"));
        assert_eq!(Err(String::from("Invalid square string")), Square::from_string("123"));
        assert_eq!(Err(String::from("Invalid square string")), Square::from_string("h9"));
        assert_eq!(Err(String::from("Invalid square string")), Square::from_string("j1"));
        assert_eq!(Err(String::from("Invalid square string")), Square::from_string("nonsense"));
        assert_eq!(Err(String::from("Invalid square string")), Square::from_string("2e"));
        assert_eq!(Err(String::from("Invalid square string")), Square::from_string("G9"));
    }

    #[test]
    fn get_file_returns_correct_file() {
        assert_eq!(File::A, A1.get_file());
        assert_eq!(File::A, A2.get_file());
        assert_eq!(File::A, A3.get_file());
        assert_eq!(File::A, A4.get_file());
        assert_eq!(File::A, A5.get_file());
        assert_eq!(File::A, A6.get_file());
        assert_eq!(File::A, A7.get_file());
        assert_eq!(File::A, A8.get_file());

        assert_eq!(File::B, B1.get_file());
        assert_eq!(File::B, B2.get_file());
        assert_eq!(File::B, B3.get_file());
        assert_eq!(File::B, B4.get_file());
        assert_eq!(File::B, B5.get_file());
        assert_eq!(File::B, B6.get_file());
        assert_eq!(File::B, B7.get_file());
        assert_eq!(File::B, B8.get_file());

        assert_eq!(File::C, C1.get_file());
        assert_eq!(File::C, C2.get_file());
        assert_eq!(File::C, C3.get_file());
        assert_eq!(File::C, C4.get_file());
        assert_eq!(File::C, C5.get_file());
        assert_eq!(File::C, C6.get_file());
        assert_eq!(File::C, C7.get_file());
        assert_eq!(File::C, C8.get_file());

        assert_eq!(File::D, D1.get_file());
        assert_eq!(File::D, D2.get_file());
        assert_eq!(File::D, D3.get_file());
        assert_eq!(File::D, D4.get_file());
        assert_eq!(File::D, D5.get_file());
        assert_eq!(File::D, D6.get_file());
        assert_eq!(File::D, D7.get_file());
        assert_eq!(File::D, D8.get_file());

        assert_eq!(File::E, E1.get_file());
        assert_eq!(File::E, E2.get_file());
        assert_eq!(File::E, E3.get_file());
        assert_eq!(File::E, E4.get_file());
        assert_eq!(File::E, E5.get_file());
        assert_eq!(File::E, E6.get_file());
        assert_eq!(File::E, E7.get_file());
        assert_eq!(File::E, E8.get_file());

        assert_eq!(File::F, F1.get_file());
        assert_eq!(File::F, F2.get_file());
        assert_eq!(File::F, F3.get_file());
        assert_eq!(File::F, F4.get_file());
        assert_eq!(File::F, F5.get_file());
        assert_eq!(File::F, F6.get_file());
        assert_eq!(File::F, F7.get_file());
        assert_eq!(File::F, F8.get_file());

        assert_eq!(File::G, G1.get_file());
        assert_eq!(File::G, G2.get_file());
        assert_eq!(File::G, G3.get_file());
        assert_eq!(File::G, G4.get_file());
        assert_eq!(File::G, G5.get_file());
        assert_eq!(File::G, G6.get_file());
        assert_eq!(File::G, G7.get_file());
        assert_eq!(File::G, G8.get_file());

        assert_eq!(File::H, H1.get_file());
        assert_eq!(File::H, H2.get_file());
        assert_eq!(File::H, H3.get_file());
        assert_eq!(File::H, H4.get_file());
        assert_eq!(File::H, H5.get_file());
        assert_eq!(File::H, H6.get_file());
        assert_eq!(File::H, H7.get_file());
        assert_eq!(File::H, H8.get_file());

        assert_ne!(File::A, H1.get_file());
        assert_ne!(File::B, G7.get_file());
        assert_ne!(File::C, F4.get_file());
    }

    #[test]
    fn get_rank_returns_correct_rank() {
        assert_eq!(Rank::First, A1.get_rank());
        assert_eq!(Rank::First, B1.get_rank());
        assert_eq!(Rank::First, C1.get_rank());
        assert_eq!(Rank::First, D1.get_rank());
        assert_eq!(Rank::First, E1.get_rank());
        assert_eq!(Rank::First, F1.get_rank());
        assert_eq!(Rank::First, G1.get_rank());
        assert_eq!(Rank::First, H1.get_rank());

        assert_eq!(Rank::Second, A2.get_rank());
        assert_eq!(Rank::Second, B2.get_rank());
        assert_eq!(Rank::Second, C2.get_rank());
        assert_eq!(Rank::Second, D2.get_rank());
        assert_eq!(Rank::Second, E2.get_rank());
        assert_eq!(Rank::Second, F2.get_rank());
        assert_eq!(Rank::Second, F2.get_rank());
        assert_eq!(Rank::Second, H2.get_rank());

        assert_eq!(Rank::Third, A3.get_rank());
        assert_eq!(Rank::Third, B3.get_rank());
        assert_eq!(Rank::Third, C3.get_rank());
        assert_eq!(Rank::Third, D3.get_rank());
        assert_eq!(Rank::Third, E3.get_rank());
        assert_eq!(Rank::Third, F3.get_rank());
        assert_eq!(Rank::Third, F3.get_rank());
        assert_eq!(Rank::Third, H3.get_rank());

        assert_eq!(Rank::Fourth, A4.get_rank());
        assert_eq!(Rank::Fourth, B4.get_rank());
        assert_eq!(Rank::Fourth, C4.get_rank());
        assert_eq!(Rank::Fourth, D4.get_rank());
        assert_eq!(Rank::Fourth, E4.get_rank());
        assert_eq!(Rank::Fourth, F4.get_rank());
        assert_eq!(Rank::Fourth, F4.get_rank());
        assert_eq!(Rank::Fourth, H4.get_rank());

        assert_eq!(Rank::Fifth, A5.get_rank());
        assert_eq!(Rank::Fifth, B5.get_rank());
        assert_eq!(Rank::Fifth, C5.get_rank());
        assert_eq!(Rank::Fifth, D5.get_rank());
        assert_eq!(Rank::Fifth, E5.get_rank());
        assert_eq!(Rank::Fifth, F5.get_rank());
        assert_eq!(Rank::Fifth, F5.get_rank());
        assert_eq!(Rank::Fifth, H5.get_rank());

        assert_eq!(Rank::Sixth, A6.get_rank());
        assert_eq!(Rank::Sixth, B6.get_rank());
        assert_eq!(Rank::Sixth, C6.get_rank());
        assert_eq!(Rank::Sixth, D6.get_rank());
        assert_eq!(Rank::Sixth, E6.get_rank());
        assert_eq!(Rank::Sixth, F6.get_rank());
        assert_eq!(Rank::Sixth, F6.get_rank());
        assert_eq!(Rank::Sixth, H6.get_rank());

        assert_eq!(Rank::Seventh, A7.get_rank());
        assert_eq!(Rank::Seventh, B7.get_rank());
        assert_eq!(Rank::Seventh, C7.get_rank());
        assert_eq!(Rank::Seventh, D7.get_rank());
        assert_eq!(Rank::Seventh, E7.get_rank());
        assert_eq!(Rank::Seventh, F7.get_rank());
        assert_eq!(Rank::Seventh, F7.get_rank());
        assert_eq!(Rank::Seventh, H7.get_rank());

        assert_eq!(Rank::Eighth, A8.get_rank());
        assert_eq!(Rank::Eighth, B8.get_rank());
        assert_eq!(Rank::Eighth, C8.get_rank());
        assert_eq!(Rank::Eighth, D8.get_rank());
        assert_eq!(Rank::Eighth, E8.get_rank());
        assert_eq!(Rank::Eighth, F8.get_rank());
        assert_eq!(Rank::Eighth, F8.get_rank());
        assert_eq!(Rank::Eighth, H8.get_rank());

        assert_ne!(Rank::First, F8.get_rank());
        assert_ne!(Rank::Sixth, H8.get_rank());
        assert_ne!(Rank::Eighth, F2.get_rank());
    }

    #[test]
    fn from_file_rank_returns_correct_square() {
        assert_eq!(0, Square::from_file_rank(File::A, Rank::First).index);
        assert_eq!(1, Square::from_file_rank(File::B, Rank::First).index);
        assert_eq!(2, Square::from_file_rank(File::C, Rank::First).index);
        assert_eq!(3, Square::from_file_rank(File::D, Rank::First).index);
        assert_eq!(4, Square::from_file_rank(File::E, Rank::First).index);
        assert_eq!(5, Square::from_file_rank(File::F, Rank::First).index);
        assert_eq!(6, Square::from_file_rank(File::G, Rank::First).index);
        assert_eq!(7, Square::from_file_rank(File::H, Rank::First).index);

        assert_eq!(8, Square::from_file_rank(File::A, Rank::Second).index);
        assert_eq!(9, Square::from_file_rank(File::B, Rank::Second).index);
        assert_eq!(10, Square::from_file_rank(File::C, Rank::Second).index);
        assert_eq!(11, Square::from_file_rank(File::D, Rank::Second).index);
        assert_eq!(12, Square::from_file_rank(File::E, Rank::Second).index);
        assert_eq!(13, Square::from_file_rank(File::F, Rank::Second).index);
        assert_eq!(14, Square::from_file_rank(File::G, Rank::Second).index);
        assert_eq!(15, Square::from_file_rank(File::H, Rank::Second).index);

        assert_eq!(16, Square::from_file_rank(File::A, Rank::Third).index);
        assert_eq!(17, Square::from_file_rank(File::B, Rank::Third).index);
        assert_eq!(18, Square::from_file_rank(File::C, Rank::Third).index);
        assert_eq!(19, Square::from_file_rank(File::D, Rank::Third).index);
        assert_eq!(20, Square::from_file_rank(File::E, Rank::Third).index);
        assert_eq!(21, Square::from_file_rank(File::F, Rank::Third).index);
        assert_eq!(22, Square::from_file_rank(File::G, Rank::Third).index);
        assert_eq!(23, Square::from_file_rank(File::H, Rank::Third).index);

        assert_eq!(24, Square::from_file_rank(File::A, Rank::Fourth).index);
        assert_eq!(25, Square::from_file_rank(File::B, Rank::Fourth).index);
        assert_eq!(26, Square::from_file_rank(File::C, Rank::Fourth).index);
        assert_eq!(27, Square::from_file_rank(File::D, Rank::Fourth).index);
        assert_eq!(28, Square::from_file_rank(File::E, Rank::Fourth).index);
        assert_eq!(29, Square::from_file_rank(File::F, Rank::Fourth).index);
        assert_eq!(30, Square::from_file_rank(File::G, Rank::Fourth).index);
        assert_eq!(31, Square::from_file_rank(File::H, Rank::Fourth).index);

        assert_eq!(32, Square::from_file_rank(File::A, Rank::Fifth).index);
        assert_eq!(33, Square::from_file_rank(File::B, Rank::Fifth).index);
        assert_eq!(34, Square::from_file_rank(File::C, Rank::Fifth).index);
        assert_eq!(35, Square::from_file_rank(File::D, Rank::Fifth).index);
        assert_eq!(36, Square::from_file_rank(File::E, Rank::Fifth).index);
        assert_eq!(37, Square::from_file_rank(File::F, Rank::Fifth).index);
        assert_eq!(38, Square::from_file_rank(File::G, Rank::Fifth).index);
        assert_eq!(39, Square::from_file_rank(File::H, Rank::Fifth).index);

        assert_eq!(40, Square::from_file_rank(File::A, Rank::Sixth).index);
        assert_eq!(41, Square::from_file_rank(File::B, Rank::Sixth).index);
        assert_eq!(42, Square::from_file_rank(File::C, Rank::Sixth).index);
        assert_eq!(43, Square::from_file_rank(File::D, Rank::Sixth).index);
        assert_eq!(44, Square::from_file_rank(File::E, Rank::Sixth).index);
        assert_eq!(45, Square::from_file_rank(File::F, Rank::Sixth).index);
        assert_eq!(46, Square::from_file_rank(File::G, Rank::Sixth).index);
        assert_eq!(47, Square::from_file_rank(File::H, Rank::Sixth).index);

        assert_eq!(48, Square::from_file_rank(File::A, Rank::Seventh).index);
        assert_eq!(49, Square::from_file_rank(File::B, Rank::Seventh).index);
        assert_eq!(50, Square::from_file_rank(File::C, Rank::Seventh).index);
        assert_eq!(51, Square::from_file_rank(File::D, Rank::Seventh).index);
        assert_eq!(52, Square::from_file_rank(File::E, Rank::Seventh).index);
        assert_eq!(53, Square::from_file_rank(File::F, Rank::Seventh).index);
        assert_eq!(54, Square::from_file_rank(File::G, Rank::Seventh).index);
        assert_eq!(55, Square::from_file_rank(File::H, Rank::Seventh).index);

        assert_eq!(56, Square::from_file_rank(File::A, Rank::Eighth).index);
        assert_eq!(57, Square::from_file_rank(File::B, Rank::Eighth).index);
        assert_eq!(58, Square::from_file_rank(File::C, Rank::Eighth).index);
        assert_eq!(59, Square::from_file_rank(File::D, Rank::Eighth).index);
        assert_eq!(60, Square::from_file_rank(File::E, Rank::Eighth).index);
        assert_eq!(61, Square::from_file_rank(File::F, Rank::Eighth).index);
        assert_eq!(62, Square::from_file_rank(File::G, Rank::Eighth).index);
        assert_eq!(63, Square::from_file_rank(File::H, Rank::Eighth).index);
    }
}