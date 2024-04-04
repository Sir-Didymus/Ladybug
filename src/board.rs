//! The board module deals with basic board representation.
//! It contains the important position and bitboard submodules, as well other useful ones such as color, file, rank, and square.
//! This module is the foundation on which the rest of the engine builds upon.

use position::Position;

pub mod bitboard;
pub mod color;
pub mod file;
pub mod rank;
pub mod square;
pub mod castling_rights;
pub mod piece;
pub mod position;
pub mod fen;

/// The board struct holds the current position of the board.
/// It also keeps track of the half-move (ply) history, the full move counter, the number of reversible half-moves (50 move rule),
/// and a list of all positions that have been on the board before (threefold repetition).
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board {
    /// The current position of the chess board.
    pub position: Position,
    /// The current full move count (incremented after Black's play).
    pub fullmove_counter: u32,
    /// The number of reversible ply (no pawn moves or captures).
    pub halfmove_clock: u32,
}

impl Default for Board {
    /// Default constructor for Board.
    /// Returns a board with default values.
    fn default() -> Self {
        Self {
            position: Position::default(),
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}

impl Board {
    /// Constructs a new board from a FEN string.
    /// If the FEN could be parsed successfully, the result will contain the newly constructed board.
    /// Otherwise, it will contain an error.
    pub fn from_fen(fen: &str) -> Result<Board, String> {
        Self::parse_fen(fen)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::Board;
    use crate::board::castling_rights::CastlingRights;
    use crate::board::color::Color::{Black, White};
    use crate::board::position::Position;
    use crate::lookup::lookup_table::LookupTable;

    #[test]
    fn default_returns_board_with_default_values() {
        let board = Board::default();
        assert_eq!(Position::default(), board.position);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(1, board.fullmove_counter);
    }

    #[test]
    fn from_fen_with_valid_fen_returns_board() {
        let mut lookup_table = LookupTable::default();
        lookup_table.initialize_tables();

        // -----------------------------------------------------------------------------------------
        // position 1
        // -----------------------------------------------------------------------------------------

        let board = Board::from_fen("3r2k1/ppp2p1p/3p2p1/3P2P1/5P2/4r3/P1B3P1/5RK1 w - - 0 34").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x4820004100), Bitboard::new(0), Bitboard::new(0x400), Bitboard::new(0x20), Bitboard::new(0), Bitboard::new(0x40)],
            [Bitboard::new(0xa7480000000000), Bitboard::new(0), Bitboard::new(0), Bitboard::new(0x800000000100000), Bitboard::new(0), Bitboard::new(0x4000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(White, board.position.color_to_move);
        assert_eq!([CastlingRights::NoRights; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(34, board.fullmove_counter);

        // -----------------------------------------------------------------------------------------
        // position 2
        // -----------------------------------------------------------------------------------------

        let board = Board::from_fen("r2qk2r/pp3Qpp/2n1p3/3pN1b1/3P4/2P5/PP3PPP/RN2K2R b KQkq - 0 13").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x804e300), Bitboard::new(0x1000000002), Bitboard::new(0), Bitboard::new(0x81), Bitboard::new(0x20000000000000), Bitboard::new(0x10)],
            [Bitboard::new(0xc3100800000000), Bitboard::new(0x40000000000), Bitboard::new(0x4000000000), Bitboard::new(0x8100000000000000), Bitboard::new(0x800000000000000), Bitboard::new(0x1000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(Black, board.position.color_to_move);
        assert_eq!([CastlingRights::Both; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(0, board.halfmove_clock);
        assert_eq!(13, board.fullmove_counter);

        // -----------------------------------------------------------------------------------------
        // position 3
        // -----------------------------------------------------------------------------------------

        let board = Board::from_fen("4k3/1pp4r/2p2P1b/p1n1P1Rp/6p1/P1N3P1/1PP4P/2K5 w - - 4 29").unwrap();
        // expected piece bitboards of the resulting position
        let bitboards = [
            [Bitboard::new(0x201000418600), Bitboard::new(0x40000), Bitboard::new(0), Bitboard::new(0x4000000000), Bitboard::new(0), Bitboard::new(0x4)],
            [Bitboard::new(0x6048140000000), Bitboard::new(0x400000000), Bitboard::new(0x800000000000), Bitboard::new(0x80000000000000), Bitboard::new(0), Bitboard::new(0x1000000000000000)]
        ];
        assert_eq!(bitboards, board.position.pieces);
        assert_eq!(White, board.position.color_to_move);
        assert_eq!([CastlingRights::NoRights; 2], board.position.castling_rights);
        assert_eq!(None, board.position.en_passant);
        assert_eq!(4, board.halfmove_clock);
        assert_eq!(29, board.fullmove_counter);
    }
}
