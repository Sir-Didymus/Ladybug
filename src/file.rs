/// This enum represents a file on a chessboard.
///
/// Can be compared using "==", thanks to the [PartialEq](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) trait.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

/// The number of files on a chessboard.
pub const NUM_FILES: u8 = 8;

impl File {
    /// Returns the index of the file, ranging from 0 (File A) to 7 (File H).
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Returns a file based on an index.
    pub fn from_index(index: u8) -> File {
        match index % 8 {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::file::File;

    #[test]
    fn to_index_returns_correct_index() {
        assert_eq!(0, File::A.to_index());
        assert_eq!(1, File::B.to_index());
        assert_eq!(2, File::C.to_index());
        assert_eq!(3, File::D.to_index());
        assert_eq!(4, File::E.to_index());
        assert_eq!(5, File::F.to_index());
        assert_eq!(6, File::G.to_index());
        assert_eq!(7, File::H.to_index());
    }

    #[test]
    fn from_index_returns_correct_file() {
        assert_eq!(File::A, File::from_index(0));
        assert_eq!(File::B, File::from_index(1));
        assert_eq!(File::C, File::from_index(2));
        assert_eq!(File::D, File::from_index(3));
        assert_eq!(File::E, File::from_index(4));
        assert_eq!(File::F, File::from_index(5));
        assert_eq!(File::G, File::from_index(6));
        assert_eq!(File::H, File::from_index(7));

        assert_ne!(File::A, File::from_index(1));
        assert_ne!(File::B, File::from_index(6));
        assert_ne!(File::C, File::from_index(0));
        
        assert!(!(File::A == File::B));
    }
}