use crate::board::bitboard::Bitboard;
use crate::board::Board;
use crate::board::color::Color::{Black, White};
use crate::board::file::File;
use crate::board::piece::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};
use crate::board::rank::Rank;
use crate::board::square::Square;

/// Parses a [FEN](https://www.chessprogramming.org/Forsyth-Edwards_Notation) string and returns a result.
/// If the FEN could be parsed successfully, the result will contain a board. Otherwise, it will contain an error.
pub fn parse_fen(fen: String) -> Result<Board, String> {
    Ok(Board::default())
}

/// Takes a FEN and splits it into its 6 parts.
fn split_fen(fen: String) -> Result<Vec<String>, String> {
    let fen_parts: Vec<String> = fen.split_whitespace().map(|s| s.to_string()).collect();
    match fen_parts.len() {
        6 => Ok(fen_parts),
        _other => Err(String::from("Invalid FEN")),
    }
}

/// Parses the first part of the FEN (pieces).
fn parse_pieces(piece_fen: String) -> Result<[[Bitboard; 6]; 2], String> {
    let mut pieces = [[Bitboard::new(0); 6]; 2];
    let piece_parts: Vec<String> = piece_fen.split('/').map(|s| s.to_string()).collect();
    if piece_parts.len() != 8 {
        return Err(String::from("Invalid FEN"));
    }
    for (rank_index, piece_str) in piece_parts.iter().enumerate() {
        let mut file_index: usize = 0;
        for char in piece_str.chars() {
            match char {
                'p' => pieces[Black.to_index() as usize][Pawn.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'n' => pieces[Black.to_index() as usize][Knight.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'b' => pieces[Black.to_index() as usize][Bishop.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'r' => pieces[Black.to_index() as usize][Rook.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'q' => pieces[Black.to_index() as usize][Queen.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'k' => pieces[Black.to_index() as usize][King.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'P' => pieces[White.to_index() as usize][Pawn.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'N' => pieces[White.to_index() as usize][Knight.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'B' => pieces[White.to_index() as usize][Bishop.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'R' => pieces[White.to_index() as usize][Rook.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'Q' => pieces[White.to_index() as usize][Queen.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                'K' => pieces[White.to_index() as usize][King.to_index() as usize].set_bit(Square::from_file_rank(File::from_index(file_index as u8), Rank::from_index(7 - rank_index as u8))),
                '1' => (),
                '2'..='8' => {
                    let files_to_skip = char.to_digit(10);
                    match files_to_skip {
                        Some(files_to_skip) => file_index += files_to_skip as usize - 1,
                        None => return Err(String::from("Invalid FEN")),
                    }
                }
                _other => return Err(String::from("Invalid FEN")),
            }
            file_index += 1;
        }
    }
    Ok(pieces)
}

#[cfg(test)]
mod tests {
    use crate::board::bitboard::Bitboard;
    use crate::board::color::Color::{Black, White};
    use crate::board::fen::{parse_pieces, split_fen};
    use crate::board::piece::Piece::{Bishop, King, Knight, Pawn, Queen, Rook};

    #[test]
    fn split_fen_returns_vec_with_6_strings() {
        let fen_parts = split_fen(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2")).unwrap();
        assert_eq!(6, fen_parts.len());
        assert_eq!("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R", fen_parts[0]);
        assert_eq!("b", fen_parts[1]);
        assert_eq!("KQkq", fen_parts[2]);
        assert_eq!("-", fen_parts[3]);
        assert_eq!("1", fen_parts[4]);
        assert_eq!("2", fen_parts[5]);
    }

    #[test]
    fn split_fen_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), split_fen(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1")));
        assert_eq!(Err(String::from("Invalid FEN")), split_fen(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1       ")));
        assert_eq!(Err(String::from("Invalid FEN")), split_fen(String::from(" b KQkq - 1       ")));
        assert_ne!(Err(String::from("Invalid FEN")), split_fen(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2")));
    }

    #[test]
    fn parser_pieces_with_valid_fen_returns_piece_bitboards() {
        let pieces = parse_pieces(String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")).unwrap();
        assert_eq!(0xff000000000000, pieces[Black.to_index() as usize][Pawn.to_index() as usize].value); // black pawns
        assert_eq!(0x4200000000000000, pieces[Black.to_index() as usize][Knight.to_index() as usize].value); // black knights
        assert_eq!(0x2400000000000000, pieces[Black.to_index() as usize][Bishop.to_index() as usize].value); // black bishops
        assert_eq!(0x8100000000000000, pieces[Black.to_index() as usize][Rook.to_index() as usize].value); // black rooks
        assert_eq!(0x800000000000000, pieces[Black.to_index() as usize][Queen.to_index() as usize].value); // black queens
        assert_eq!(0x1000000000000000, pieces[Black.to_index() as usize][King.to_index() as usize].value); // black kings
        assert_eq!(0xff00, pieces[White.to_index() as usize][Pawn.to_index() as usize].value); // white pawns
        assert_eq!(0x42, pieces[White.to_index() as usize][Knight.to_index() as usize].value); // white knights
        assert_eq!(0x24, pieces[White.to_index() as usize][Bishop.to_index() as usize].value); // white bishops
        assert_eq!(0x81, pieces[White.to_index() as usize][Rook.to_index() as usize].value); // white rooks
        assert_eq!(0x8, pieces[White.to_index() as usize][Queen.to_index() as usize].value); // white queens
        assert_eq!(0x10, pieces[White.to_index() as usize][King.to_index() as usize].value); // white kings

        let pieces = parse_pieces(String::from("r5k1/p1p1q1pp/1p1P4/3n1r2/2Q2B2/2N5/PP3PPP/R4RK1")).unwrap();
        assert_eq!(0xc5020000000000, pieces[Black.to_index() as usize][Pawn.to_index() as usize].value); // black pawns
        assert_eq!(0x800000000, pieces[Black.to_index() as usize][Knight.to_index() as usize].value); // black knights
        assert_eq!(0x0, pieces[Black.to_index() as usize][Bishop.to_index() as usize].value); // black bishops
        assert_eq!(0x100002000000000, pieces[Black.to_index() as usize][Rook.to_index() as usize].value); // black rooks
        assert_eq!(0x10000000000000, pieces[Black.to_index() as usize][Queen.to_index() as usize].value); // black queens
        assert_eq!(0x4000000000000000, pieces[Black.to_index() as usize][King.to_index() as usize].value); // black kings
        assert_eq!(0x8000000e300, pieces[White.to_index() as usize][Pawn.to_index() as usize].value); // white pawns
        assert_eq!(0x40000, pieces[White.to_index() as usize][Knight.to_index() as usize].value); // white knights
        assert_eq!(0x20000000, pieces[White.to_index() as usize][Bishop.to_index() as usize].value); // white bishops
        assert_eq!(0x21, pieces[White.to_index() as usize][Rook.to_index() as usize].value); // white rooks
        assert_eq!(0x4000000, pieces[White.to_index() as usize][Queen.to_index() as usize].value); // white queens
        assert_eq!(0x40, pieces[White.to_index() as usize][King.to_index() as usize].value); // white kings

        let pieces = parse_pieces(String::from("8/8/8/8/8/8/8/8")).unwrap();
        assert_eq!([[Bitboard::new(0); 6]; 2], pieces);
    }

    #[test]
    fn parse_pieces_with_invalid_fen_returns_error() {
        assert_eq!(Err(String::from("Invalid FEN")), parse_pieces(String::from("/rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R/")));
        assert_eq!(Err(String::from("Invalid FEN")), parse_pieces(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP")));
        assert_ne!(Err(String::from("Invalid FEN")), parse_pieces(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R")));
    }
}