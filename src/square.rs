#![allow(dead_code)]

/// A square on the chessboard, represented by an index ranging from 0 to 63.
pub struct Square {
    pub index: u8,
}

impl Square {
    /// Constructs a new square from a given index.
    pub fn new(index: u8) -> Square {
        Square { index }
    }

    /// Returns the file of the square.
    pub fn get_file(&self) -> u8 {
        self.index % 8
    }

    /// Returns the rank of the square.
    pub fn get_rank(&self) -> u8 {
        self.index / 8
    }
}

pub const A1: Square = Square { index: 7 };
pub const B1: Square = Square { index: 6 };
pub const C1: Square = Square { index: 5 };
pub const D1: Square = Square { index: 4 };
pub const E1: Square = Square { index: 3 };
pub const F1: Square = Square { index: 2 };
pub const G1: Square = Square { index: 1 };
pub const H1: Square = Square { index: 0 };
pub const A2: Square = Square { index: 15 };
pub const B2: Square = Square { index: 14 };
pub const C2: Square = Square { index: 13 };
pub const D2: Square = Square { index: 12 };
pub const E2: Square = Square { index: 11 };
pub const F2: Square = Square { index: 10 };
pub const G2: Square = Square { index: 9 };
pub const H2: Square = Square { index: 8 };
pub const A3: Square = Square { index: 23 };
pub const B3: Square = Square { index: 22 };
pub const C3: Square = Square { index: 21 };
pub const D3: Square = Square { index: 20 };
pub const E3: Square = Square { index: 19 };
pub const F3: Square = Square { index: 18 };
pub const G3: Square = Square { index: 17 };
pub const H3: Square = Square { index: 16 };
pub const A4: Square = Square { index: 31 };
pub const B4: Square = Square { index: 30 };
pub const C4: Square = Square { index: 29 };
pub const D4: Square = Square { index: 28 };
pub const E4: Square = Square { index: 27 };
pub const F4: Square = Square { index: 26 };
pub const G4: Square = Square { index: 25 };
pub const H4: Square = Square { index: 24 };
pub const A5: Square = Square { index: 39 };
pub const B5: Square = Square { index: 38 };
pub const C5: Square = Square { index: 37 };
pub const D5: Square = Square { index: 36 };
pub const E5: Square = Square { index: 35 };
pub const F5: Square = Square { index: 34 };
pub const G5: Square = Square { index: 33 };
pub const H5: Square = Square { index: 32 };
pub const A6: Square = Square { index: 47 };
pub const B6: Square = Square { index: 46 };
pub const C6: Square = Square { index: 45 };
pub const D6: Square = Square { index: 44 };
pub const E6: Square = Square { index: 43 };
pub const F6: Square = Square { index: 42 };
pub const G6: Square = Square { index: 41 };
pub const H6: Square = Square { index: 40 };
pub const A7: Square = Square { index: 55 };
pub const B7: Square = Square { index: 54 };
pub const C7: Square = Square { index: 53 };
pub const D7: Square = Square { index: 52 };
pub const E7: Square = Square { index: 51 };
pub const F7: Square = Square { index: 50 };
pub const G7: Square = Square { index: 49 };
pub const H7: Square = Square { index: 48 };
pub const A8: Square = Square { index: 63 };
pub const B8: Square = Square { index: 62 };
pub const C8: Square = Square { index: 61 };
pub const D8: Square = Square { index: 60 };
pub const E8: Square = Square { index: 59 };
pub const F8: Square = Square { index: 58 };
pub const G8: Square = Square { index: 57 };
pub const H8: Square = Square { index: 56 };