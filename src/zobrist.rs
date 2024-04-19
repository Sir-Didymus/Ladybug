use crate::board::color::Color;
use crate::board::position::Position;
use crate::board::square::{NUM_SQUARES, Square};

pub mod random;

/// Returns the zobrist hash for the given position.
pub fn get_hash(position: &Position) -> u64 {
    // pieces
    let mut pieces: u64 = 0;
    for square_index in 0..NUM_SQUARES {
        let square = Square::new(square_index);
        if let Some((piece, color)) = position.get_piece(square) {
            pieces ^= random::get_random_piece(piece, color, square);
        }
    }

    // castling rights
    let castling_rights = random::get_random_castle(position.castling_rights[Color::White.to_index() as usize], position.castling_rights[Color::Black.to_index() as usize]);

    // en passant
    let en_passant = random::get_random_en_passant(position.en_passant.map(|square| square.get_file()));

    // turn
    let turn = random::get_random_turn(position.color_to_move);

    // the result hash key
    pieces ^ castling_rights ^ en_passant ^ turn
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::board::position::Position;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen;
    use crate::zobrist::get_hash;

    /// This function is basically identical to the perft function, but instead of verifying move generation,
    /// the function is used to verify that incremental hash updating produces the same hashes as generating them from scratch.
    fn zobrist_perft(position: Position, depth: u64) {
        // if depth is zero, break out of recursion
        if depth == 0 {
            return;
        }

        // generate all legal moves for the position
        let move_list = move_gen::generate_moves(position);

        // call the zobrist_perft function recursively for all legal moves
        for i in 0..move_list.len() {
            let ply = move_list.get(i);

            let new_position = position.make_move(ply);
            let hash_from_scratch = get_hash(&new_position);

            // in case the hashes are not the same, print positions and ply for debugging purposes
            if hash_from_scratch != new_position.hash {
                println!("{position}");
                println!("{new_position}");
                println!("{ply}");
            }
            
            assert_eq!(hash_from_scratch, new_position.hash);
            
            zobrist_perft(position.make_move(ply), depth - 1);
        }
    }

    #[test]
    fn test_incremental_hash_updates() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // Position 1 - Starting Position
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        zobrist_perft(position, 5);

        // Position 2
        let position = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap().position;
        zobrist_perft(position, 4);

        // Position 3
        let position = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap().position;
        zobrist_perft(position, 5);

        // Position 4
        let position = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap().position;
        zobrist_perft(position, 5);

        // Position 5
        let position = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap().position;
        zobrist_perft(position, 4);
    }

    #[test]
    fn test_get_hash() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        // starting position
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(0x463b96181691fc9c, get_hash(&position));

        // position after e2e4
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap().position;
        //assert_eq!(0x823c9b50fd114196, get_hash(position));

        // position after e2e4 d7d5
        let position = Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2").unwrap().position;
        //assert_eq!(0x0756b94461c50fb0, get_hash(position));

        // position after e2e4 d7d5 e4e5
        let position = Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2").unwrap().position;
        assert_eq!(0x662fafb965db29d4, get_hash(&position));

        // position after e2e4 d7d5 e4e5 f7f5
        let position = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3").unwrap().position;
        assert_eq!(0x22a48b5a8e47ff78, get_hash(&position));

        // position after e2e4 d7d5 e4e5 f7f5 e1e2
        let position = Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR b kq - 0 3").unwrap().position;
        assert_eq!(0x652a607ca3f242c1, get_hash(&position));

        // position after e2e4 d7d5 e4e5 f7f5 e1e2 e8f7
        let position = Board::from_fen("rnbq1bnr/ppp1pkpp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR w - - 0 4").unwrap().position;
        assert_eq!(0x00fdd303c946bdd9, get_hash(&position));

        // ---------------------------------------------------------------------------------------------------------------------------------------

        // position after a2a4 b7b5 h2h4 b5b4 c2c4
        let position = Board::from_fen("rnbqkbnr/p1pppppp/8/8/PpP4P/8/1P1PPPP1/RNBQKBNR b KQkq c3 0 3").unwrap().position;
        assert_eq!(0x3c8123ea7b067637, get_hash(&position));

        // position after a2a4 b7b5 h2h4 b5b4 c2c4 b4c3 a1a3
        let position = Board::from_fen("rnbqkbnr/p1pppppp/8/8/P6P/R1p5/1P1PPPP1/1NBQKBNR b Kkq - 0 4").unwrap().position;
        assert_eq!(0x5c3f9b829b279560, get_hash(&position));
    }
}