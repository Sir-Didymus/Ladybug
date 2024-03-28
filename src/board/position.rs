use crate::board::bitboard::Bitboard;
use crate::board::castling_rights::CastlingRights;
use crate::board::castling_rights::CastlingRights::NoRights;
use crate::board::color::{Color};
use crate::board::color::Color::White;
use crate::board::file::File;
use crate::board::piece::Piece;
use crate::board::square::Square;

/// This struct uniquely encodes a chess position.
/// It contains 12 bitboards, one for each piece for each color.
/// It also contains information on whether en passant is possible, whose side it is to move,
/// and the castling rights for each player.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Position {
    /// Bitboards for all pieces for both White and Black.
    pub pieces: [[Bitboard; 6]; 2],
    
    /// The castling rights for both White and Black.
    pub castling_rights: [CastlingRights; 2],

    /// If en passant is possible, this Option contains the target file for the en passant move.
    pub en_passant: Option<File>,

    /// The color whose turn it is.
    pub color_to_move: Color,
}

impl Default for Position {
    /// Default constructor for Position.
    /// Returns a position with all bitboards having the value 0, meaning no pieces are on the board.
    fn default() -> Self {
        Self {
            pieces: [[Bitboard::new(0); 6]; 2],
            castling_rights: [NoRights; 2],
            en_passant: None,
            color_to_move: White,
        }
    }
}

impl Position  {
    /// Sets a piece of the specified color on the specified square.
    pub fn set_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[color.to_index() as usize][piece.to_index() as usize].set_bit(square);
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::castling_rights::CastlingRights::NoRights;
    use crate::board::color::Color::{Black, White};
    use crate::board::piece::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
    use crate::board::position::Position;
    use crate::board::square::{E4, G3, H7};

    #[test]
    fn default_returns_position_with_default_values() {
        let position = Position::default();
        assert_eq!([[Bitboard::new(0); 6]; 2], position.pieces);
        assert_eq!([NoRights; 2], position.castling_rights);
        assert_eq!(None, position.en_passant);
        assert_eq!(White, position.color_to_move);
    }
    
    #[test]
    pub fn set_piece_sets_piece_on_correct_square_and_correct_bitboard() {
        let mut position = Position::default();
        
        position.set_piece(Knight, White, E4);
        assert!(position.pieces[White.to_index() as usize][Knight.to_index() as usize].get_bit(E4));
        assert!(!position.pieces[White.to_index() as usize][Knight.to_index() as usize].get_bit(H7));
        
        position.set_piece(Knight, White, H7);
        assert!(position.pieces[White.to_index() as usize][Knight.to_index() as usize].get_bit(H7));
        assert!(!position.pieces[Black.to_index() as usize][Knight.to_index() as usize].get_bit(H7));
        assert!(!position.pieces[White.to_index() as usize][Queen.to_index() as usize].get_bit(H7));
        
        position.set_piece(Knight, Black, H7);
        assert!(position.pieces[Black.to_index() as usize][Knight.to_index() as usize].get_bit(H7));
        
        let position_before = Position::default();
        let mut position_after = position_before;
        position_after.set_piece(Bishop, Black, G3);

        // test that black's knight bitboard changed
        assert_ne!(position_before.pieces[Black.to_index() as usize][Bishop.to_index() as usize], position_after.pieces[Black.to_index() as usize][Bishop.to_index() as usize]);
        
        // test that other bitboards are still the same
        assert_eq!(position_before.pieces[White.to_index() as usize], position_after.pieces[White.to_index() as usize]);
        assert_eq!(position_before.pieces[Black.to_index() as usize][Pawn.to_index() as usize], position_after.pieces[Black.to_index() as usize][Pawn.to_index() as usize]);
        assert_eq!(position_before.pieces[Black.to_index() as usize][Knight.to_index() as usize], position_after.pieces[Black.to_index() as usize][Knight.to_index() as usize]);
        assert_eq!(position_before.pieces[Black.to_index() as usize][Rook.to_index() as usize], position_after.pieces[Black.to_index() as usize][Rook.to_index() as usize]);
        assert_eq!(position_before.pieces[Black.to_index() as usize][Queen.to_index() as usize], position_after.pieces[Black.to_index() as usize][Queen.to_index() as usize]);
        assert_eq!(position_before.pieces[Black.to_index() as usize][King.to_index() as usize], position_after.pieces[Black.to_index() as usize][King.to_index() as usize]);
    }
}