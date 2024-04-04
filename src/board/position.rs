use std::fmt::{Display, Formatter};
use crate::board::bitboard::Bitboard;
use crate::board::castling_rights::CastlingRights;
use crate::board::castling_rights::CastlingRights::NoRights;
use crate::board::color::{Color, NUM_COLORS};
use crate::board::color::Color::White;
use crate::board::file::{File, NUM_FILES};
use crate::board::piece::{NUM_PIECES, Piece};
use crate::board::rank::{NUM_RANKS, Rank};
use crate::board::square::Square;
use crate::lookup::LOOKUP_TABLE;

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

    /// If en passant is possible, this Option contains the target square for the en passant move.
    pub en_passant: Option<Square>,

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
    /// 
    /// This method DOES NOT check if there already is another piece on that square,
    /// so use `get_piece` to check if the square is unoccupied first.
    pub fn set_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[color.to_index() as usize][piece.to_index() as usize].set_bit(square);
    }
    
    /// Returns the piece and the piece's color on the specified square.
    /// Returns None if no piece occupies the square.
    pub fn get_piece(&self, square: Square) -> Option<(Piece, Color)> {
        for color_index in 0..NUM_COLORS {
            for piece_index in 0..NUM_PIECES {
                match self.pieces[color_index as usize][piece_index as usize].get_bit(square) {
                    true => return Some((Piece::from_index(piece_index), Color::from_index(color_index))),
                    false => {},
                }
            }
        }
        None
    }
    
    /// Returns the occupancy bitboard for the specified color.
    pub fn get_occupancy(&self, color: Color) -> Bitboard {
        let mut occupancy_bb = Bitboard::new(0);
        for bitboard in self.pieces[color.to_index() as usize] {
            occupancy_bb.value |= bitboard.value;
        }
        occupancy_bb
    }

    /// Returns the occupancies bitboard for both colors.
    pub fn get_occupancies(&self) -> Bitboard {
        let mut occupancy_bb = Bitboard::new(0);
        for color_index in 0..NUM_COLORS {
            for bitboard in self.pieces[color_index as usize] {
                occupancy_bb.value |= bitboard.value;
            }
        }
        occupancy_bb
    }

    /// Returns the attack bitboard for all pieces of the given color.
    ///
    /// For example `get_attack_bb(Color::White)` will return a bitboard with all squares
    /// set that are attacked by any of White's pieces.
    pub fn get_attack_bb(&self, color: Color) -> Bitboard {
        // the result attack_bb
        let mut attack_bb = Bitboard::new(0);
        // `or` the attack bitboards for all pieces of the given color
        for piece_index in 0..NUM_PIECES {
            attack_bb.value |= self.get_piece_attack_bb(Piece::from_index(piece_index), color).value;
        }
        attack_bb
    }

    /// Returns the attack bitboard for the given type of piece of the given color.
    ///
    /// For example `get_attack_bb(Piece::Bishop, Color::White)` will return a bitboard with all squares
    /// set that are attacked by White's bishops.
    pub fn get_piece_attack_bb(&self, piece: Piece, color: Color) -> Bitboard {
        // get a reference to the lookup table
        let lookup = LOOKUP_TABLE.get().unwrap();
        // the result attack_bb
        let mut attack_bb = Bitboard::new(0);
        // the bitboard for the given piece of the given color
        let piece_bb = self.pieces[color.to_index() as usize][piece.to_index() as usize];
        // get squares with pieces on them
        let active_squares = piece_bb.get_active_bits();
        // get blocker bitboard (needed for slider pieces)
        let blockers = self.get_occupancies();

        // loop over active squares and `or` the result attack_bb with the attack bitboard of the piece on each square
        for square in active_squares {
            match piece {
                Piece::Pawn => attack_bb.value |= lookup.get_pawn_attacks(square, color).value,
                Piece::Knight => attack_bb.value |= lookup.get_knight_attacks(square).value,
                Piece::Bishop => attack_bb.value |= lookup.get_bishop_attacks(square, blockers).value,
                Piece::Rook => attack_bb.value |= lookup.get_rook_attacks(square, blockers).value,
                Piece::Queen => attack_bb.value |= lookup.get_queen_attacks(square, blockers).value,
                Piece::King => attack_bb.value |= lookup.get_king_attacks(square).value,
            };
        }

        attack_bb
    }
}

/// Prints the position with '.' marking empty squares, capital letters marking white pieces,
/// and lower case letter marking black pieces.
/// It also prints the color to move, castling rights, and en passant target square.
impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output: String = String::from("");
        for rank in (0..NUM_RANKS).rev() {
            output += format!("{}  ", rank + 1).as_str();
            for file in 0..NUM_FILES {
                let piece = self.get_piece(Square::from_file_rank(File::from_index(file), Rank::from_index(rank)));
                if piece.is_none() {
                    output += ".  ";
                } else {
                    let (piece, color) = piece.unwrap();
                    output.push(piece.to_char(color));
                    output += "  ";
                }
            }
            output += "\n";
        }
        output += "   a  b  c  d  e  f  g  h\n";
        output += format!("\nMove: {}", self.color_to_move).as_str();
        output += format!("\nCastling: {} - {}", self.castling_rights[0],  self.castling_rights[1]).as_str();
        match self.en_passant {
            None => output += "\nEn Passant: None\n",
            Some(square) => output += format!("\nEn Passant: {square}\n").as_str(),
        }
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::castling_rights::CastlingRights::NoRights;
    use crate::board::color::Color::{Black, White};
    use crate::board::{Board};
    use crate::board::color::Color;
    use crate::board::piece::Piece;
    use crate::board::piece::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
    use crate::board::position::Position;
    use crate::board::square::{A1, A3, E1, E4, F2, F3, G3, H7, H8};
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;

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
    
    #[test]
    fn get_piece_returns_piece_on_specified_square() {
        let mut position = Position::default();
        position.set_piece(Knight, Black, E4);
        position.set_piece(King, White, H8);
        position.set_piece(Bishop, White, F2);
        
        assert_eq!(None, position.get_piece(A3));
        assert_eq!(None, position.get_piece(F3));
        assert_eq!(None, position.get_piece(A1));
        assert_eq!(None, position.get_piece(E1));
        assert_eq!(None, position.get_piece(H7));

        assert_eq!(Some((Knight, Black)), position.get_piece(E4));
        assert_eq!(Some((King, White)), position.get_piece(H8));
        assert_eq!(Some((Bishop, White)), position.get_piece(F2));
    }
    
    #[test]
    fn get_occupancy_returns_occupancy_bb() {
        // position 1 (starting position)
        let position = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(0xffff, position.get_occupancy(White).value);
        assert_eq!(0xffff000000000000, position.get_occupancy(Black).value);
        
        // position 2
        let position = Board::parse_fen("2kr2r1/1pb1qp1p/2b1pp2/p1Q5/3P3B/P4N1P/2P1BPP1/3RRK2 b - - 0 22").unwrap().position;
        assert_eq!(0x488a17438, position.get_occupancy(White).value);
        assert_eq!(0x4cb6340100000000, position.get_occupancy(Black).value);

        // position 3
        let position = Board::parse_fen("8/8/4k2p/7P/5p2/K7/r2r4/1q6 w - - 10 59").unwrap().position;
        assert_eq!(0x8000010000, position.get_occupancy(White).value);
        assert_eq!(0x900020000902, position.get_occupancy(Black).value);
    }

    #[test]
    fn get_occupancies_returns_occupancy_bb_for_both_colors() {
        // position 1 (starting position)
        let position = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(0xffff00000000ffff, position.get_occupancies().value);

        // position 2
        let position = Board::parse_fen("2kr2r1/1pb1qp1p/2b1pp2/p1Q5/3P3B/P4N1P/2P1BPP1/3RRK2 b - - 0 22").unwrap().position;
        assert_eq!(0x4cb6340588a17438, position.get_occupancies().value);

        // position 3
        let position = Board::parse_fen("8/8/4k2p/7P/5p2/K7/r2r4/1q6 w - - 10 59").unwrap().position;
        assert_eq!(0x908020010902, position.get_occupancies().value);
    }
    
    #[test]
    fn position_formats_correctly() {
        let position = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let expected_output = "8  r  n  b  q  k  b  n  r  \n7  p  p  p  p  p  p  p  p  \n6  .  .  .  .  .  .  .  .  \n5  .  .  .  .  .  .  .  .  \n4  .  .  .  .  .  .  .  .  \n3  .  .  .  .  .  .  .  .  \n2  P  P  P  P  P  P  P  P  \n1  R  N  B  Q  K  B  N  R  \n   a  b  c  d  e  f  g  h\n\nMove: White\nCastling: Both - Both\nEn Passant: None\n";
        assert_eq!(expected_output, format!("{}", position));

        let position = Board::parse_fen("r1bq1rk1/1pp1bppp/p1n2n2/4p3/2PpP1P1/P2P1Q1P/1P1N1P2/R1B1KBNR b KQ g3 0 9").unwrap().position;
        let expected_output = "8  r  .  b  q  .  r  k  .  \n7  .  p  p  .  b  p  p  p  \n6  p  .  n  .  .  n  .  .  \n5  .  .  .  .  p  .  .  .  \n4  .  .  P  p  P  .  P  .  \n3  P  .  .  P  .  Q  .  P  \n2  .  P  .  N  .  P  .  .  \n1  R  .  B  .  K  B  N  R  \n   a  b  c  d  e  f  g  h\n\nMove: Black\nCastling: Both - NoRights\nEn Passant: g3\n";
        assert_eq!(expected_output, format!("{}", position));
    }

    #[test]
    fn test_get_attack_bb() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // -----------------------------------------------------------------------------------------
        // position 1 (starting position)
        // -----------------------------------------------------------------------------------------

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let attack_bb = position.get_attack_bb(White);
        assert_eq!(0xffff7e, attack_bb.value);

        // -----------------------------------------------------------------------------------------
        // position 2
        // -----------------------------------------------------------------------------------------

        let position = Board::from_fen("r1bq1rk1/p5pp/3p1p2/1ppP2b1/2Pp1B2/1P1P1B2/P2Q1PPP/R3R1K1 b - - 3 17").unwrap().position;
        let attack_bb = position.get_attack_bb(White);
        assert_eq!(0x10101cdb77feffff, attack_bb.value);

        // -----------------------------------------------------------------------------------------
        // position 3
        // -----------------------------------------------------------------------------------------

        let position = Board::from_fen("r3nrk1/2qn2pp/1p1bb3/1Q3p2/3P4/1N2P1N1/PP1BB1PP/R4RK1 w - - 2 21").unwrap().position;
        let attack_bb = position.get_attack_bb(Black);
        assert_eq!(0xfeffef3d77470504, attack_bb.value);
    }

    #[test]
    fn test_get_piece_attack_bb() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // -----------------------------------------------------------------------------------------
        // position 1 (starting position)
        // -----------------------------------------------------------------------------------------

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let pawn_attack_bb = position.get_piece_attack_bb(Pawn, Black);
        let knight_attack_bb = position.get_piece_attack_bb(Knight, Black);
        let bishop_attack_bb = position.get_piece_attack_bb(Piece::Bishop, Color::Black);
        let rook_attack_bb = position.get_piece_attack_bb(Piece::Rook, Color::Black);
        let queen_attack_bb = position.get_piece_attack_bb(Piece::Queen, Color::Black);
        let king_attack_bb = position.get_piece_attack_bb(Piece::King, Color::Black);

        assert_eq!(0xff0000000000, pawn_attack_bb.value);
        assert_eq!(6936818859638784, knight_attack_bb.value);
        assert_eq!(0x5a000000000000, bishop_attack_bb.value);
        assert_eq!(0x4281000000000000, rook_attack_bb.value);
        assert_eq!(0x141c000000000000, queen_attack_bb.value);
        assert_eq!(0x2838000000000000, king_attack_bb.value);

        // -----------------------------------------------------------------------------------------
        // position 2
        // -----------------------------------------------------------------------------------------

        let position = Board::from_fen("6k1/2p2pp1/7p/N7/8/1Pn3P1/4r1qP/R6K w - - 0 29").unwrap().position;
        let pawn_attack_bb = position.get_piece_attack_bb(Piece::Pawn, Color::White);
        let knight_attack_bb = position.get_piece_attack_bb(Piece::Knight, Color::White);
        let bishop_attack_bb = position.get_piece_attack_bb(Piece::Bishop, Color::White);
        let rook_attack_bb = position.get_piece_attack_bb(Piece::Rook, Color::White);
        let queen_attack_bb = position.get_piece_attack_bb(Piece::Queen, Color::White);
        let king_attack_bb = position.get_piece_attack_bb(Piece::King, Color::White);

        assert_eq!(0xa5400000, pawn_attack_bb.value);
        assert_eq!(0x2040004020000, knight_attack_bb.value);
        assert_eq!(0, bishop_attack_bb.value);
        assert_eq!(0x1010101fe, rook_attack_bb.value);
        assert_eq!(0, queen_attack_bb.value);
        assert_eq!(0xc040, king_attack_bb.value);

        // -----------------------------------------------------------------------------------------
        // position 3
        // -----------------------------------------------------------------------------------------

        let position = Board::from_fen("r7/pb3k2/1p4p1/3N1p2/2P2Np1/3BR3/PP4PP/6K1 b - - 1 28").unwrap().position;
        let pawn_attack_bb = position.get_piece_attack_bb(Piece::Pawn, Color::White);
        let knight_attack_bb = position.get_piece_attack_bb(Piece::Knight, Color::White);
        let bishop_attack_bb = position.get_piece_attack_bb(Piece::Bishop, Color::White);
        let rook_attack_bb = position.get_piece_attack_bb(Piece::Rook, Color::White);
        let queen_attack_bb = position.get_piece_attack_bb(Piece::Queen, Color::White);
        let king_attack_bb = position.get_piece_attack_bb(Piece::King, Color::White);

        assert_eq!(0xa00e70000, pawn_attack_bb.value);
        assert_eq!(0x147288229c5000, knight_attack_bb.value);
        assert_eq!(0x2014001422, bishop_attack_bb.value);
        assert_eq!(0x1010101010e81010, rook_attack_bb.value);
        assert_eq!(0, queen_attack_bb.value);
        assert_eq!(0xe0a0, king_attack_bb.value);
    }
}