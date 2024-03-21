/// This enum represents a file on a chessboard.
#[derive(Debug, Clone, Copy)]
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
    /// Returns the index of the file, ranging from 0 (File A) to 7 (File B).
    pub fn index(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod tests  {
    use crate::file::File;

    #[test]
    fn index_returns_correct_index() {
        assert_eq!(0, File::A.index());
        assert_eq!(1, File::B.index());
        assert_eq!(2, File::C.index());
        assert_eq!(3, File::D.index());
        assert_eq!(4, File::E.index());
        assert_eq!(5, File::F.index());
        assert_eq!(6, File::G.index());
        assert_eq!(7, File::H.index());
    }
}