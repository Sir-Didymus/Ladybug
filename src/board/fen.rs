use crate::board::bitboard::Bitboard;
use crate::board::Board;

/// Parses a [FEN](https://www.chessprogramming.org/Forsyth-Edwards_Notation) string and returns a result.
/// If the FEN could be parsed successfully, the result will contain a board. Otherwise, it will contain an error.
pub fn parse_fen(fen: String) -> Result<Board, String> {
    Ok(Board::default())
}

/// Takes a FEN and splits it into its 6 parts.
fn split_fen(fen: String) -> Result<Vec<String>, String> {
    let fen_parts : Vec<String> = fen.split_whitespace().map(|s| s.to_string()).collect();
    match fen_parts.len() {
        6 => Ok(fen_parts),
        _other => Err(String::from("Invalid FEN")),
    }
}

/// Parses the first part of the FEN (pieces).
fn parse_pieces(pieceFen: String) -> Result<[[Bitboard; 6]; 2], String> {
    let pieces = [[Bitboard::new(0); 6]; 2];
    
    Ok(pieces)
}

#[cfg(test)]
mod tests {
    use crate::board::fen::split_fen;

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
        assert_eq!(Err(String::from("Invalid FEN")),split_fen(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1")));
        assert_eq!(Err(String::from("Invalid FEN")),split_fen(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1       ")));
        assert_eq!(Err(String::from("Invalid FEN")),split_fen(String::from(" b KQkq - 1       ")));
        assert_ne!(Err(String::from("Invalid FEN")),split_fen(String::from("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2")));
    }
}