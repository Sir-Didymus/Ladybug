use std::fmt::{Display, Formatter};
use crate::board::color::Color;
use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::square;
use crate::board::square::Square;
use crate::evaluation::pst;
use crate::move_gen;

const SOURCE_SQUARE_MASK: u32 = 0b11111100_00000000_00000000_00000000;
const SHIFT_SOURCE_SQUARE: u32 = 26;

const TARGET_SQUARE_MASK: u32 = 0b00000011_11110000_00000000_00000000;
const SHIFT_TARGET_SQUARE: u32 = 20;

const PIECE_MASK: u32 = 0b00000000_00001110_00000000_00000000;
const SHIFT_PIECE: u32 = 17;

const CAPTURED_PIECE_MASK: u32 = 0b00000000_00000001_11000000_00000000;
const SHIFT_CAPTURED_PIECE: u32 = 14;

const PROMOTION_PIECE_MASK: u32 = 0b00000000_00000000_00111000_00000000;
const SHIFT_PROMOTION_PIECE: u32 = 11;



/// This struct represents a halfmove, also known as [ply](https://www.chessprogramming.org/Ply).
///
/// In the comments, I will often refer to a ply as a move, even though a move technically involves
/// both White's and Black's responses. Unless stated otherwise, move and ply mean basically the same in this repository.
/// Moves in the actual sense will be referred to as "fullmove".
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ply {
    /// The source square.
    pub source: Square,
    /// The target square.
    pub target: Square,
    /// The type of the piece to move.
    pub piece: Piece,
    /// If the move is a capture move, this field will contain the type of the captured piece.
    pub captured_piece: Option<Piece>,
    /// If the move is a pawn promotion, this field will contain the promotion piece.
    pub promotion_piece: Option<Piece>,
}

impl Default for Ply {
    /// Returns an illegal default ply of a pawn move from a1 to a1.
    /// It is only used to initialize a ply array with default values.
    fn default() -> Self {
        Ply {
            source: square::A1,
            target: square::A1,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        }
    }
}

/// Prints the ply as text.
impl Display for Ply {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output: String = String::from("");
        output += format!("{}", self.source).as_str();
        output += format!("{}", self.target).as_str();
        if self.promotion_piece.is_some() {
            output.push(self.promotion_piece.unwrap().to_char(Color::Black));
        }
        write!(f, "{}", output)
    }
}

impl Ply {
    /// Returns the score of the ply based on [MVV-LVA](https://www.chessprogramming.org/MVV-LVA).
    pub fn score(&self) -> i32 {
        let mut score = 0;
        
        // add the value of the captured piece (if any)
        if let Some(piece) = self.captured_piece {
            score += pst::PIECE_VALUES[piece.to_index() as usize];
        }
        
        // add the value of the promotion piece (if any)
        if let Some(piece) = self.promotion_piece {
            score += pst::PIECE_VALUES[piece.to_index() as usize];
        }
        
        // subtract the index of the moving piece
        score -= self.piece.to_index() as i32;
        
        score
    }
    
    /// Encodes the ply as 32-bit unsigned integer.
    ///
    /// The format is as follows:
    /// 11111100 00000000 00000000 00000000 : source square
    /// 00000011 11110000 00000000 00000000 : target square
    /// 00000000 00001110 00000000 00000000 : piece
    /// 00000000 00000001 11000000 00000000 : captured piece
    /// 00000000 00000000 00111000 00000000 : promotion piece
    /// 00000000 00000000 00000111 11111111 : unused bits - may use later
    pub fn encode(&self) -> u32 {
        let mut encoded_ply: u32 = 0;
        
        // set the source square bits
        encoded_ply |= (self.source.index as u32) << SHIFT_SOURCE_SQUARE;

        // set the target square bits
        encoded_ply |= (self.target.index as u32) << SHIFT_TARGET_SQUARE;
        
        // set the piece bits
        encoded_ply |= (self.piece.to_index() as u32) << SHIFT_PIECE;

        // set the captured piece bits
        encoded_ply |= match self.captured_piece {
            None => 6 << SHIFT_CAPTURED_PIECE, // 6 represents no piece
            Some(captured_piece) => (captured_piece.to_index() as u32) << SHIFT_CAPTURED_PIECE,
        };

        // set the promotion piece bits
        encoded_ply |= match self.promotion_piece {
            None => 6 << SHIFT_PROMOTION_PIECE, // 6 represents no piece
            Some(captured_piece) => (captured_piece.to_index() as u32) << SHIFT_PROMOTION_PIECE,
        };
        
        encoded_ply
    }

    /// Decodes a 32-bit unsigned integer into a ply.
    pub fn decode(encoded_ply: u32) -> Ply {
        // decode source square
        let source = Square::new(((encoded_ply & SOURCE_SQUARE_MASK) >> SHIFT_SOURCE_SQUARE) as u8);
        
        // decode target square
        let target = Square::new(((encoded_ply & TARGET_SQUARE_MASK) >> SHIFT_TARGET_SQUARE) as u8);
        
        // decode piece
        let piece = Piece::from_index(((encoded_ply & PIECE_MASK) >> SHIFT_PIECE) as u8);
        
        // decode captured piece
        let piece_index = ((encoded_ply & CAPTURED_PIECE_MASK) >> SHIFT_CAPTURED_PIECE) as u8;
        let captured_piece = match piece_index {
            6 => None,
            other => Some(Piece::from_index(other)),
        };

        // decode promotion piece
        let piece_index = ((encoded_ply & PROMOTION_PIECE_MASK) >> SHIFT_PROMOTION_PIECE) as u8;
        let promotion_piece = match piece_index {
            6 => None,
            other => Some(Piece::from_index(other)),
        };
        
        Ply {
            source,
            target,
            piece,
            captured_piece,
            promotion_piece,
        }
    }
    
    /// Tries to construct a ply from the given string for the given position.
    pub fn from_string(ply_str: &str, position: Position) -> Option<Ply> {
        // get the chars from the ply string
        let char_vec: Vec<char> = ply_str.chars().collect();
        if !(4..=5).contains(&char_vec.len()) {
            return None;
        }

        // source square as string
        let mut source_str = String::from("");
        source_str.push(char_vec[0]);
        source_str.push(char_vec[1]);

        // target square as string
        let mut target_str = String::from("");
        target_str.push(char_vec[2]);
        target_str.push(char_vec[3]);

        // try to construct squares from the strings
        let source_square = Square::from_string(source_str.as_str());
        let target_square = Square::from_string(target_str.as_str());
        if source_square.is_err() || target_square.is_err() {
            return None;
        }
        let source_square = source_square.unwrap();
        let target_square = target_square.unwrap();

        let mut promotion_piece: Option<Piece> = None;

        if char_vec.len() == 5 {
            // move is a promotion
            let mut promotion_piece_string = String::from("");
            promotion_piece_string.push(char_vec[4]);
            match Piece::from_string(promotion_piece_string.as_str()) {
                None => return None, // invalid promotion piece string
                Some(piece) => promotion_piece = Some(piece),
            };
        }

        // generate all legal moves for the given position
        let move_list = move_gen::generate_moves(position);
        let mut move_list_vec: Vec<Ply> = Vec::new();

        for i in 0..move_list.len() {
            move_list_vec.push(move_list.get(i));
        }

        // search for ply in the move list
        let ply = match move_list_vec.iter().find(|r| r.source == source_square && r.target == target_square && r.promotion_piece == promotion_piece) {
            None => return None, // if the move list does not contain a ply with the specified source and target squares, the move is not legal
            Some(ply) => *ply,
        };

        Some(ply)
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::Piece;
    use crate::board::{Board, square};
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::ply::Ply;
    
    #[test]
    fn default_returns_illegal_ply() {
        let ply = Ply::default();
        assert_eq!(square::A1, ply.source);
        assert_eq!(square::A1, ply.target);
        assert_eq!(Piece::Pawn, ply.piece);
        assert_eq!(None, ply.captured_piece);
        assert_eq!(None, ply.promotion_piece);
    }
    
    #[test]
    fn test_score() {
        let ply = Ply {source: square::A1, target: square::A2, piece: Piece::Rook, captured_piece: None, promotion_piece: None};
        assert_eq!(-3, ply.score());

        let ply = Ply {source: square::A1, target: square::A2, piece: Piece::Pawn, captured_piece: None, promotion_piece: Some(Piece::Knight)};
        assert_eq!(320, ply.score());

        let ply = Ply {source: square::H7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)};
        assert_eq!(1270, ply.score());
    }
    
    #[test]
    fn test_encode_and_decode() {
        let ply = Ply {source: square::A1, target: square::A2, piece: Piece::Rook, captured_piece: None, promotion_piece: None};
        assert_eq!(ply, Ply::decode(ply.encode()));

        let ply = Ply {source: square::H8, target: square::A8, piece: Piece::Rook, captured_piece: Some(Piece::Rook), promotion_piece: None};
        assert_eq!(ply, Ply::decode(ply.encode()));

        let ply = Ply {source: square::E4, target: square::D5, piece: Piece::Pawn, captured_piece: Some(Piece::Pawn), promotion_piece: None};
        assert_eq!(ply, Ply::decode(ply.encode()));

        let ply = Ply {source: square::G7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)};
        assert_eq!(ply, Ply::decode(ply.encode()));

        let ply = Ply {source: square::H3, target: square::C8, piece: Piece::Bishop, captured_piece: Some(Piece::Rook), promotion_piece: None};
        assert_eq!(ply, Ply::decode(ply.encode()));
    }

    #[test]
    fn ply_formats_correctly() {
        let mut ply = Ply {
            source: square::E2,
            target: square::E4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        };
        assert_eq!("e2e4", format!("{ply}"));

        ply.source = square::A4;
        ply.target = square::E8;
        assert_eq!("a4e8", format!("{ply}"));

        ply.source = square::H3;
        ply.target = square::H6;
        assert_eq!("h3h6", format!("{ply}"));
        
        ply.source = square::H7;
        ply.target = square::H8;
        ply.promotion_piece = Some(Piece::Queen);
        assert_eq!("h7h8q", format!("{ply}"));
    }

    #[test]
    fn from_string_with_invalid_move_returns_none() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(None, Ply::from_string("nonsense", position));
        assert_eq!(None, Ply::from_string("a1a1", position));
        assert_eq!(None, Ply::from_string("e1d1", position));

        let position = Board::from_fen("r1bqk1nr/pppp3p/2n2p2/2b1p2Q/2B1P2N/2P5/PP1P1P1p/RNB1K3 b Qkq - 1 9").unwrap().position;
        assert_eq!(None, Ply::from_string("h2h1q", position));

        let position = Board::from_fen("r1bqk1nr/pppp1p1p/2n5/2b1p3/2B1P3/2P2N2/PP1P1P1p/RNBQK3 w Qkq - 0 8").unwrap().position;
        assert_eq!(None, Ply::from_string("b1c3", position));
    }

    #[test]
    fn from_string_with_valid_move_returns_ply() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(Some(Ply {
            source: square::E2,
            target: square::E4,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: None,
        }), Ply::from_string("e2e4", position));

        let position = Board::from_fen("r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 4 4").unwrap().position;
        assert_eq!(Some(Ply {
            source: square::E1,
            target: square::G1,
            piece: Piece::King,
            captured_piece: None,
            promotion_piece: None,
        }), Ply::from_string("e1g1", position));

        let position = Board::from_fen("r1bqk1nr/pppp1p1p/2n5/2b1p1N1/2B1P3/2P5/PP1P1P1p/RNBQK3 b Qkq - 1 8").unwrap().position;
        assert_eq!(Some(Ply {
            source: square::H2,
            target: square::H1,
            piece: Piece::Pawn,
            captured_piece: None,
            promotion_piece: Some(Piece::Knight),
        }), Ply::from_string("h2h1n", position));

        let position = Board::from_fen("r1bq2nr/1pppk2p/2n2p2/p1b1p3/2B1P2N/2P5/PP1P1P1p/RNB1K1Q1 b Q - 1 11").unwrap().position;
        assert_eq!(Some(Ply {
            source: square::H2,
            target: square::G1,
            piece: Piece::Pawn,
            captured_piece: Some(Piece::Queen),
            promotion_piece: Some(Piece::Queen),
        }), Ply::from_string("h2g1Q", position));
    }
}