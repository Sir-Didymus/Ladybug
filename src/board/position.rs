use std::fmt::{Display, Formatter};
use crate::board::bitboard::Bitboard;
use crate::board::castling_rights::CastlingRights;
use crate::board::color::{Color, NUM_COLORS};
use crate::board::file::{File, NUM_FILES};
use crate::board::piece::{NUM_PIECES, Piece};
use crate::board::piece::Piece::{King};
use crate::board::rank::{NUM_RANKS, Rank};
use crate::board::square;
use crate::board::square::Square;
use crate::lookup::LOOKUP_TABLE;
use crate::move_gen::ply::Ply;

/// This struct uniquely encodes a chess position.
/// It contains 12 bitboards, one for each piece for each color.
/// It also contains information on whether en passant is possible, whose side it is to move,
/// and the castling rights for each player.
#[derive(Copy, Clone, Debug)]
pub struct Position {
    /// Bitboards for all pieces for both White and Black.
    pub pieces: [[Bitboard; 6]; 2],

    /// The castling rights for both White and Black.
    pub castling_rights: [CastlingRights; 2],

    /// If en passant is possible, this Option contains the target square for the en passant move.
    pub en_passant: Option<Square>,

    /// The color whose turn it is.
    pub color_to_move: Color,

    //-------------------------------------------------------------------------------------------
    // fields not necessary to uniquely identify a chess position, but convenient
    //-------------------------------------------------------------------------------------------

    /// The attack_bbs for White's and Black's pieces.
    attack_bb: [Bitboard; 2],
}

impl Default for Position {
    /// Default constructor for Position.
    /// Returns a position with all bitboards having the value 0, meaning no pieces are on the board.
    fn default() -> Self {
        let mut position = Self {
            pieces: [[Bitboard::new(0); 6]; 2],
            castling_rights: [CastlingRights::NoRights; 2],
            en_passant: None,
            color_to_move: Color::White,
            attack_bb: [Bitboard::new(0); 2],
        };
        position.initialize_attack_bb();
        position
    }
}

impl PartialEq for Position {
    /// Implement PartialEq for Position.
    /// This has to be done manually because only the fields necessary to uniquely encode a 
    /// chess position should be compared. The attack bitboards are irrelevant.
    fn eq(&self, other: &Self) -> bool {
        self.pieces == other.pieces && self.castling_rights == other.castling_rights &&
            self.en_passant == other.en_passant && self.color_to_move == other.color_to_move
    }
}

impl Position {
    /// Constructs a new Position.
    pub fn new(pieces: [[Bitboard; 6]; 2], castling_rights: [CastlingRights; 2], en_passant: Option<Square>, color_to_move: Color) -> Self {
        let mut position = Self {
            pieces,
            castling_rights,
            en_passant,
            color_to_move,
            attack_bb: [Bitboard::new(0); 2],
        };
        position.initialize_attack_bb();
        position
    }

    /// Sets a piece of the specified color on the specified square.
    ///
    /// This method DOES NOT check if there already is another piece on that square,
    /// so use `get_piece` to check if the square is unoccupied first.
    pub fn set_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[color.to_index() as usize][piece.to_index() as usize].set_bit(square);
    }

    /// Removes a piece of the given color from the given square.
    pub fn remove_piece(&mut self, piece: Piece, color: Color, square: Square) {
        self.pieces[color.to_index() as usize][piece.to_index() as usize].pop_bit(square);
    }

    /// Returns the piece and the piece's color on the specified square.
    /// Returns None if no piece occupies the square.
    pub fn get_piece(&self, square: Square) -> Option<(Piece, Color)> {
        for color_index in 0..NUM_COLORS {
            for piece_index in 0..NUM_PIECES {
                match self.pieces[color_index as usize][piece_index as usize].get_bit(square) {
                    true => return Some((Piece::from_index(piece_index), Color::from_index(color_index))),
                    false => {}
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
        self.attack_bb[color.to_index() as usize]
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

    /// Returns whether the given square is attacked by a piece of the given color.
    pub fn is_square_attacked(&self, square: Square, color: Color) -> bool {
        self.get_attack_bb(color).get_bit(square)
    }

    /// Returns whether the king of the given color is in check
    pub fn is_in_check(&self, color: Color) -> bool {
        let king_square = self.pieces[color.to_index() as usize][Piece::King.to_index() as usize].get_active_bits()[0];
        self.is_square_attacked(king_square, color.other())
    }

    /// Returns whether the position is legal.
    /// Specifically, it validates that:
    /// - both sides have exactly 1 king
    /// - the side whose turn it not is, is not in check
    pub fn is_legal(&self) -> bool {
        self.pieces[Color::White.to_index() as usize][King.to_index() as usize].get_active_bits().len() == 1 &&
            self.pieces[Color::Black.to_index() as usize][King.to_index() as usize].get_active_bits().len() == 1 &&
            !self.is_in_check(self.color_to_move.other())
    }

    /// Returns a new position that reflects the board state where the given move (ply) has been played.
    pub fn make_move(&self, ply: Ply) -> Position {
        let mut position = *self;

        // remove piece from old position
        position.remove_piece(ply.piece, self.color_to_move, ply.source);

        // remove capture piece
        if let Some(piece) = ply.captured_piece { position.remove_piece(piece, self.color_to_move.other(), ply.target) }

        // set piece on new position
        match ply.promotion_piece {
            // move is a promotion - set promotion piece
            Some(piece) => { position.set_piece(piece, self.color_to_move, ply.target) }
            // move is not a promotion - set piece specified in ply
            None => { position.set_piece(ply.piece, self.color_to_move, ply.target) }
        }

        // in case of castling, set the rook
        if ply.piece == Piece::King {
            match (ply.source, ply.target) {
                // black castles queenside
                (square::E8, square::C8) => {
                    // remove rook from old position
                    position.remove_piece(Piece::Rook, self.color_to_move, square::A8);
                    // set rook on new position
                    position.set_piece(Piece::Rook, self.color_to_move, square::D8);
                }
                // black castles kingside
                (square::E8, square::G8) => {
                    // remove rook from old position
                    position.remove_piece(Piece::Rook, self.color_to_move, square::H8);
                    // set rook on new position
                    position.set_piece(Piece::Rook, self.color_to_move, square::F8);
                }
                // white castles queenside
                (square::E1, square::C1) => {
                    // remove rook from old position
                    position.remove_piece(Piece::Rook, self.color_to_move, square::A1);
                    // set rook on new position
                    position.set_piece(Piece::Rook, self.color_to_move, square::D1);
                }
                // white castles kingside
                (square::E1, square::G1) => {
                    // remove rook from old position
                    position.remove_piece(Piece::Rook, self.color_to_move, square::H1);
                    // set rook on new position
                    position.set_piece(Piece::Rook, self.color_to_move, square::F1);
                }
                _other => {}
            }
        }

        // in case of en passant, remove opponent pawn from 4th or 5th rank
        if let Some(square) = self.en_passant {
            if ply.piece == Piece::Pawn && square == ply.target {
                position.remove_piece(Piece::Pawn, self.color_to_move.other(), Square::from_file_rank(ply.target.get_file(), self.color_to_move.other().double_pawn_push_target_rank()))
            }
        }

        // update castling_rights
        if ply.piece == King {
            // move is a king move - no rights
            position.castling_rights[self.color_to_move.to_index() as usize] = CastlingRights::NoRights;
        } else if ply.piece == Piece::Rook && ply.source == Square::from_file_rank(File::A, self.color_to_move.back_rank()) {
            // move is A file rook move - remove queenside rights
            match self.castling_rights[self.color_to_move.to_index() as usize] {
                CastlingRights::Both => position.castling_rights[self.color_to_move.to_index() as usize] = CastlingRights::KingSide,
                CastlingRights::KingSide => position.castling_rights[self.color_to_move.to_index() as usize] = CastlingRights::KingSide,
                _other => position.castling_rights[self.color_to_move.to_index() as usize] = CastlingRights::NoRights,
            }
        } else if ply.piece == Piece::Rook && ply.source == Square::from_file_rank(File::H, self.color_to_move.back_rank()) {
            // move is H file rook move - remove kingside rights
            match self.castling_rights[self.color_to_move.to_index() as usize] {
                CastlingRights::Both => position.castling_rights[self.color_to_move.to_index() as usize] = CastlingRights::QueenSide,
                CastlingRights::QueenSide => position.castling_rights[self.color_to_move.to_index() as usize] = CastlingRights::QueenSide,
                _other => position.castling_rights[self.color_to_move.to_index() as usize] = CastlingRights::NoRights,
            }
        }

        // update en_passant
        if ply.piece == Piece::Pawn && ply.source.get_rank() == self.color_to_move.pawn_rank() &&
            ply.target.get_rank() == self.color_to_move.double_pawn_push_target_rank() {
            position.en_passant = Some(Square::from_file_rank(ply.source.get_file(), self.color_to_move.other().en_passant_target_rank()))
        } else {
            position.en_passant = None;
        }

        // update color_to_move
        position.color_to_move = self.color_to_move.other();

        position
    }

    /// Initializes the attack bitboards for both colors.
    pub fn initialize_attack_bb(&mut self) {
        // calculate attack_bb for both colors
        for color_index in 0..NUM_COLORS {
            // the result attack_bb for the color
            let mut attack_bb = Bitboard::new(0);
            // `or` the attack bitboards for all pieces of the given color
            for piece_index in 0..NUM_PIECES {
                attack_bb.value |= self.get_piece_attack_bb(Piece::from_index(piece_index), Color::from_index(color_index)).value;
            }
            // add the calculated bb to the result array
            self.attack_bb[color_index as usize] = attack_bb;
        }
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
        output += format!("\nCastling: {} - {}", self.castling_rights[0], self.castling_rights[1]).as_str();
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
    use crate::board::{Board, square};
    use crate::board::color::Color;
    use crate::board::piece::Piece;
    use crate::board::piece::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
    use crate::board::position::Position;
    use crate::board::square::{A1, A3, E1, E4, F2, F3, G3, H7, H8};
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::ply::Ply;

    #[test]
    fn default_returns_position_with_default_values() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        let position = Position::default();
        assert_eq!([[Bitboard::new(0); 6]; 2], position.pieces);
        assert_eq!([NoRights; 2], position.castling_rights);
        assert_eq!(None, position.en_passant);
        assert_eq!(White, position.color_to_move);
    }

    #[test]
    fn test_partial_eq() {
        let position1 = Position::default();
        let position2 = Position::default();
        assert_eq!(position1, position2);
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
    fn test_remove_piece() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        let mut position = Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        position.remove_piece(Piece::Pawn, Color::Black, square::E2);
        assert_eq!(position, Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position);

        position.remove_piece(Piece::Pawn, Color::White, square::E2);
        assert_eq!(position, Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1").unwrap().position);

        position.remove_piece(Piece::Knight, Color::White, square::D3);
        assert_eq!(position, Board::parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1").unwrap().position);

        position.remove_piece(Piece::Knight, Color::Black, square::G8);
        assert_eq!(position, Board::parse_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1").unwrap().position);

        position.remove_piece(Piece::Knight, Color::Black, square::G8);
        assert_eq!(position, Board::parse_fen("rnbqkb1r/pppppppp/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1").unwrap().position);
    }

    #[test]
    fn get_piece_returns_piece_on_specified_square() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

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
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

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
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

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
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

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

    #[test]
    fn test_is_square_attacked() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        let position = Board::from_fen("5rk1/pppr1pp1/7p/3q4/3P4/P3R1PP/1P2Q1PK/8 w - - 5 33").unwrap().position;

        assert!(position.is_square_attacked(square::B3, White));
        assert!(position.is_square_attacked(square::B3, Black));

        assert!(position.is_square_attacked(square::E1, White));
        assert!(!position.is_square_attacked(square::E1, Black));

        assert!(!position.is_square_attacked(square::H6, White));
        assert!(position.is_square_attacked(square::H6, Black));

        assert!(!position.is_square_attacked(square::A4, White));
        assert!(!position.is_square_attacked(square::A4, Black));
    }

    #[test]
    fn test_is_in_check() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1
        let position = Board::from_fen("5rk1/pppr1pp1/7p/3q4/3P4/P3R1PP/1P2Q1PK/8 w - - 5 33").unwrap().position;
        assert!(!position.is_in_check(Color::White));
        assert!(!position.is_in_check(Color::Black));

        // position 2
        let position = Board::from_fen("8/8/7Q/8/6p1/5pBk/R4K2/8 b - - 0 67").unwrap().position;
        assert!(!position.is_in_check(Color::White));
        assert!(position.is_in_check(Color::Black));

        // position 3
        let position = Board::from_fen("rn2k2r/2pq1ppp/p2b4/1p4B1/2bP4/5N2/PP3PPP/R2QR1K1 b kq - 1 14").unwrap().position;
        assert!(!position.is_in_check(Color::White));
        assert!(position.is_in_check(Color::Black));

        // position 4
        let position = Board::from_fen("8/p1p5/2p2k2/8/2KPB1r1/8/8/8 w - - 4 37").unwrap().position;
        assert!(!position.is_in_check(Color::White));
        assert!(!position.is_in_check(Color::Black));

        // position 5
        let position = Board::from_fen("3k4/p1p5/2B5/1K1P3r/8/8/8/8 w - - 3 41").unwrap().position;
        assert!(!position.is_in_check(Color::White));
        assert!(!position.is_in_check(Color::Black));

        // position 6
        let position = Board::from_fen("8/ppp3kp/2b5/5P2/3P2N1/P2B4/1r3K2/8 w - - 0 28").unwrap().position;
        assert!(position.is_in_check(Color::White));
        assert!(!position.is_in_check(Color::Black));
    }

    #[test]
    fn is_legal_with_legal_position_returns_true() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1
        assert!(Board::from_fen("5rk1/pppr1pp1/7p/3q4/3P4/P3R1PP/1P2Q1PK/8 w - - 5 33").unwrap().position.is_legal());

        // position 2
        assert!(Board::from_fen("8/8/7Q/8/6p1/5pBk/R4K2/8 b - - 0 67").unwrap().position.is_legal());

        // position 3
        assert!(Board::from_fen("rn2k2r/2pq1ppp/p2b4/1p4B1/2bP4/5N2/PP3PPP/R2QR1K1 b kq - 1 14").unwrap().position.is_legal());

        // position 4
        assert!(Board::from_fen("8/p1p5/2p2k2/8/2KPB1r1/8/8/8 w - - 4 37").unwrap().position.is_legal());

        // position 5
        assert!(Board::from_fen("3k4/p1p5/2B5/1K1P3r/8/8/8/8 w - - 3 41").unwrap().position.is_legal());

        // position 6
        assert!(Board::from_fen("8/ppp3kp/2b5/5P2/3P2N1/P2B4/1r3K2/8 w - - 0 28").unwrap().position.is_legal());
    }

    #[test]
    fn is_legal_with_illegal_position_returns_false() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // position 1
        assert!(!Board::from_fen("8/8/7Q/8/6p1/5pBk/R4K2/8 w - - 0 67").unwrap().position.is_legal());

        // position 2
        assert!(!Board::from_fen("rn2k2r/2pq1ppp/p2b4/1p4B1/2bP4/5N2/PP3PPP/R2QR1K1 w kq - 1 14").unwrap().position.is_legal());

        // position 3
        assert!(!Board::from_fen("8/ppp3kp/2b5/5P2/3P2N1/P2B4/1r3K2/8 b - - 0 28").unwrap().position.is_legal());

        // position 4
        assert!(!Board::from_fen("7Q/ppp2Qkp/2b5/4BP2/3P2N1/P2B4/1r6/8 w - - 0 28").unwrap().position.is_legal());

        // position 5
        assert!(!Board::from_fen("7Q/ppp2Qkp/2b5/4BP2/3P2N1/P2B4/1r6/8 b - - 0 28").unwrap().position.is_legal());

        // position 6
        assert!(!Board::from_fen("2kR3r/pp5p/5p1b/2p5/8/4N3/PqP1NPPP/5RK1 w - - 2 19").unwrap().position.is_legal());
    }

    #[test]
    fn test_make_move() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // e2-e4
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position.make_move(Ply {
            source: square::E2,
            target: square::E4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap().position, position);

        // e7-e6
        let position = position.make_move(Ply {
            source: square::E7,
            target: square::E6,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2").unwrap().position, position);

        // g1-f3
        let position = position.make_move(Ply {
            source: square::G1,
            target: square::F3,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqkbnr/pppp1ppp/4p3/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2").unwrap().position, position);

        // d7-d5
        let position = position.make_move(Ply {
            source: square::D7,
            target: square::D5,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqkbnr/ppp2ppp/4p3/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq d6 0 3").unwrap().position, position);

        // e4-d5
        let position = position.make_move(Ply {
            source: square::E4,
            target: square::D5,
            piece: Piece::Pawn,
            captured_piece: Some(Piece::Pawn),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqkbnr/ppp2ppp/4p3/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 3").unwrap().position, position);

        // e6-d5
        let position = position.make_move(Ply {
            source: square::E6,
            target: square::D5,
            piece: Piece::Pawn,
            captured_piece: Some(Piece::Pawn),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqkbnr/ppp2ppp/8/3p4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 4").unwrap().position, position);

        // d1-e2
        let position = position.make_move(Ply {
            source: square::D1,
            target: square::E2,
            piece: Piece::Queen,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqkbnr/ppp2ppp/8/3p4/8/5N2/PPPPQPPP/RNB1KB1R b KQkq - 1 4").unwrap().position, position);

        // f8-e7
        let position = position.make_move(Ply {
            source: square::F8,
            target: square::E7,
            piece: Piece::Bishop,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1nr/ppp1bppp/8/3p4/8/5N2/PPPPQPPP/RNB1KB1R w KQkq - 2 5").unwrap().position, position);

        // e2-e7
        let position = position.make_move(Ply {
            source: square::E2,
            target: square::E7,
            piece: Piece::Queen,
            captured_piece: Some(Bishop),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1nr/ppp1Qppp/8/3p4/8/5N2/PPPP1PPP/RNB1KB1R b KQkq - 0 5").unwrap().position, position);

        // g8-e7
        let position = position.make_move(Ply {
            source: square::G8,
            target: square::E7,
            piece: Piece::Knight,
            captured_piece: Some(Queen),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk2r/ppp1nppp/8/3p4/8/5N2/PPPP1PPP/RNB1KB1R w KQkq - 0 6").unwrap().position, position);

        // e1-e2
        let position = position.make_move(Ply {
            source: square::E1,
            target: square::E2,
            piece: Piece::King,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk2r/ppp1nppp/8/3p4/8/5N2/PPPPKPPP/RNB2B1R b kq - 1 6").unwrap().position, position);

        // h8-g8
        let position = position.make_move(Ply {
            source: square::H8,
            target: square::G8,
            piece: Piece::Rook,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1r1/ppp1nppp/8/3p4/8/5N2/PPPPKPPP/RNB2B1R w q - 2 7").unwrap().position, position);

        // h2-h4
        let position = position.make_move(Ply {
            source: square::H2,
            target: square::H4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1r1/ppp1nppp/8/3p4/7P/5N2/PPPPKPP1/RNB2B1R b q h3 0 7").unwrap().position, position);

        // g7-g5
        let position = position.make_move(Ply {
            source: square::G7,
            target: square::G5,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1r1/ppp1np1p/8/3p2p1/7P/5N2/PPPPKPP1/RNB2B1R w q g6 0 8").unwrap().position, position);

        // h4-g5
        let position = position.make_move(Ply {
            source: square::H4,
            target: square::G5,
            piece: Piece::Pawn,
            captured_piece: Some(Pawn),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1r1/ppp1np1p/8/3p2P1/8/5N2/PPPPKPP1/RNB2B1R b q - 0 8").unwrap().position, position);

        // h7-h6
        let position = position.make_move(Ply {
            source: square::H7,
            target: square::H6,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1r1/ppp1np2/7p/3p2P1/8/5N2/PPPPKPP1/RNB2B1R w q - 0 9").unwrap().position, position);

        // g5-h6
        let position = position.make_move(Ply {
            source: square::G5,
            target: square::H6,
            piece: Piece::Pawn,
            captured_piece: Some(Piece::Pawn),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("rnbqk1r1/ppp1np2/7P/3p4/8/5N2/PPPPKPP1/RNB2B1R b q - 0 9").unwrap().position, position);

        // b8-c6
        let position = position.make_move(Ply {
            source: square::B8,
            target: square::C6,
            piece: Piece::Knight,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("r1bqk1r1/ppp1np2/2n4P/3p4/8/5N2/PPPPKPP1/RNB2B1R w q - 1 10").unwrap().position, position);

        // h6-h7
        let position = position.make_move(Ply {
            source: square::H6,
            target: square::H7,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("r1bqk1r1/ppp1np1P/2n5/3p4/8/5N2/PPPPKPP1/RNB2B1R b q - 0 10").unwrap().position, position);

        // c8-h3
        let position = position.make_move(Ply {
            source: square::C8,
            target: square::H3,
            piece: Piece::Bishop,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("r2qk1r1/ppp1np1P/2n5/3p4/8/5N1b/PPPPKPP1/RNB2B1R w q - 1 11").unwrap().position, position);

        // h1-h3
        let position = position.make_move(Ply {
            source: square::H1,
            target: square::H3,
            piece: Piece::Rook,
            captured_piece: Some(Bishop),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("r2qk1r1/ppp1np1P/2n5/3p4/8/5N1R/PPPPKPP1/RNB2B2 b q - 0 11").unwrap().position, position);

        // d8-d7
        let position = position.make_move(Ply {
            source: square::D8,
            target: square::D7,
            piece: Piece::Queen,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("r3k1r1/pppqnp1P/2n5/3p4/8/5N1R/PPPPKPP1/RNB2B2 w q - 1 12").unwrap().position, position);

        // a2-a4
        let position = position.make_move(Ply {
            source: square::A2,
            target: square::A4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("r3k1r1/pppqnp1P/2n5/3p4/P7/5N1R/1PPPKPP1/RNB2B2 b q a3 0 12").unwrap().position, position);

        // e8-C8
        let position = position.make_move(Ply {
            source: square::E8,
            target: square::C8,
            piece: Piece::King,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2r1/pppqnp1P/2n5/3p4/P7/5N1R/1PPPKPP1/RNB2B2 w - - 1 13").unwrap().position, position);

        // h7-g8
        let position = position.make_move(Ply {
            source: square::H7,
            target: square::G8,
            piece: Piece::Pawn,
            captured_piece: Some(Rook),
            promotion_piece: Some(Queen),
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2Q1/pppqnp2/2n5/3p4/P7/5N1R/1PPPKPP1/RNB2B2 b - - 0 13").unwrap().position, position);

        // d5-d4
        let position = position.make_move(Ply {
            source: square::D5,
            target: square::D4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2Q1/pppqnp2/2n5/8/P2p4/5N1R/1PPPKPP1/RNB2B2 w - - 0 14").unwrap().position, position);

        // c2-c4
        let position = position.make_move(Ply {
            source: square::C2,
            target: square::C4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2Q1/pppqnp2/2n5/8/P1Pp4/5N1R/1P1PKPP1/RNB2B2 b - c3 0 14").unwrap().position, position);

        // d4-c3
        let position = position.make_move(Ply {
            source: square::D4,
            target: square::C3,
            piece: Piece::Pawn,
            captured_piece: Some(Pawn),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2Q1/pppqnp2/2n5/8/P7/2p2N1R/1P1PKPP1/RNB2B2 w - - 0 15").unwrap().position, position);

        // a4-a5
        let position = position.make_move(Ply {
            source: square::A4,
            target: square::A5,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2Q1/pppqnp2/2n5/P7/8/2p2N1R/1P1PKPP1/RNB2B2 b - - 0 15").unwrap().position, position);

        // b7-b5
        let position = position.make_move(Ply {
            source: square::B7,
            target: square::B5,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2Q1/p1pqnp2/2n5/Pp6/8/2p2N1R/1P1PKPP1/RNB2B2 w - b6 0 16").unwrap().position, position);

        // a5-b6
        let position = position.make_move(Ply {
            source: square::A5,
            target: square::B6,
            piece: Piece::Pawn,
            captured_piece: Some(Pawn),
            promotion_piece: None,
        });
        println!("{position}");
        assert_eq!(Board::from_fen("2kr2Q1/p1pqnp2/1Pn5/8/8/2p2N1R/1P1PKPP1/RNB2B2 b - - 0 16").unwrap().position, position);
    }
}