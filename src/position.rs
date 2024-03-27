use crate::board::bitboard::Bitboard;
use crate::board::castling_rights::CastlingRights;
use crate::board::castling_rights::CastlingRights::NoRights;
use crate::board::color::{Color};
use crate::board::color::Color::White;
use crate::board::file::File;

/// This struct uniquely encodes a chess position.
/// It contains 12 bitboards, one for each piece for each color.
/// It also contains information on whether en passant is possible, whose side it is to move,
/// and the castling rights for each player.
pub struct Position {
    /// Pawn bitboards for both White and Black.
    pawns: [Bitboard; 2],

    /// Knight bitboards for both White and Black.
    knights: [Bitboard; 2],

    /// Bishop bitboards for both White and Black.
    bishops: [Bitboard; 2],

    /// Rook bitboards for both White and Black.
    rooks: [Bitboard; 2],

    /// Queen bitboards for both White and Black.
    queens: [Bitboard; 2],

    /// King bitboards for both White and Black.
    kings: [Bitboard; 2],
    
    /// The castling rights for both White and Black.
    castling_rights: [CastlingRights; 2],

    /// If en passant is possible, this Option contains the target file for the en passant move.
    en_passant: Option<File>,

    /// The color whose turn it is.
    color_to_move: Color,
}

impl Default for Position {
    /// Default constructor for Position.
    /// Returns a position with all bitboards having the value 0, meaning no pieces are on the board.
    fn default() -> Self {
        Self {
            pawns: [Bitboard::new(0); 2],
            knights: [Bitboard::new(0); 2],
            bishops: [Bitboard::new(0); 2],
            rooks: [Bitboard::new(0); 2],
            queens: [Bitboard::new(0); 2],
            kings: [Bitboard::new(0); 2],
            castling_rights: [NoRights; 2],
            en_passant: None,
            color_to_move: White,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::castling_rights::CastlingRights::NoRights;
    use crate::board::color::Color::White;
    use crate::position::Position;

    #[test]
    fn default_returns_position_with_default_values() {
        let position = Position::default();
        assert_eq!([Bitboard::new(0); 2], position.pawns);
        assert_eq!([Bitboard::new(0); 2], position.knights);
        assert_eq!([Bitboard::new(0); 2], position.bishops);
        assert_eq!([Bitboard::new(0); 2], position.rooks);
        assert_eq!([Bitboard::new(0); 2], position.queens);
        assert_eq!([Bitboard::new(0); 2], position.kings);
        assert_eq!([NoRights; 2], position.castling_rights);
        assert_eq!(None, position.en_passant);
        assert_eq!(White, position.color_to_move);
    }
}