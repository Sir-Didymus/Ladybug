use crate::bitboard::Bitboard;
use crate::file::File;

/// This struct represents a chess position. 
/// It contains 12 bitboards, one for each peace for each color.
/// It also contains information on whether en passant is possible.
pub struct Position {
    /// White's king bitboard
    white_king_bb: Bitboard,
    /// Black's king bitboard
    black_king_bb: Bitboard,

    /// White's queen bitboard
    white_queen_bb: Bitboard,
    /// Black's queen bitboard
    black_queen_bb: Bitboard,

    /// White's rook bitboard
    white_rook_bb: Bitboard,
    /// Black's rook bitboard
    black_rook_bb: Bitboard,

    /// White's knight bitboard
    white_knight_bb: Bitboard,
    /// Black's knight bitboard
    black_knight_bb: Bitboard,

    /// White's bishop bitboard
    white_bishop_bb: Bitboard,
    /// Black's bishop bitboard
    black_bishop_bb: Bitboard,

    /// White's pawn bitboard
    white_pawn_bb: Bitboard,
    /// Black's pawn bitboard
    black_pawn_bb: Bitboard,

    /// If en passant is possible, this Option contains the target file for the en passant move.
    en_passant: Option<File>,
}