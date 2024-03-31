use std::fmt::{Display, Formatter};

/// Contains information on what (if any) castling rights a player has.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CastlingRights {
    NoRights,
    KingSide,
    QueenSide,
    Both,
}

/// Prints the castling rights as text.
impl Display for CastlingRights {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CastlingRights::NoRights => write!(f, "NoRights"),
            CastlingRights::KingSide => write!(f, "KingSide"),
            CastlingRights::QueenSide => write!(f, "QueenSide"),
            CastlingRights::Both => write!(f, "Both"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::castling_rights::CastlingRights;

    #[test]
    fn castling_rights_formats_correctly() {
        assert_eq!("NoRights", format!("{}", CastlingRights::NoRights));
        assert_eq!("KingSide", format!("{}", CastlingRights::KingSide));
        assert_eq!("QueenSide", format!("{}", CastlingRights::QueenSide));
        assert_eq!("Both", format!("{}", CastlingRights::Both));
    }
}