use std::fmt::{Display, Formatter};
use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::board::square::Square;
use crate::move_gen::generates_moves;

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

/// Prints the ply as text.
impl Display for Ply {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut output: String = String::from("");
        output += format!("{}", self.source).as_str();
        output += format!("{}", self.target).as_str();
        write!(f, "{}", output)
    }
}

impl Ply {
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
        let move_list = generates_moves(position);

        // search for ply in the move list
        let ply = match move_list.iter().find(|r| r.source == source_square && r.target == target_square && r.promotion_piece == promotion_piece) {
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