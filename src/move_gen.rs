//! The move_gen module is responsible for the move generation logic.

use crate::board::bitboard::Bitboard;
use crate::board::color::Color;
use crate::board::piece::Piece;
use crate::board::position::Position;
use crate::lookup::lookup_table::LookupTable;

pub mod ply;
mod pawn_moves;

/// This is the move generator. It can generate all legal moves (ply) for a given position.
/// For the move generator to work, it has to receive an initialized lookup table.
struct MoveGenerator {
    lookup: LookupTable,
}

impl MoveGenerator {
    /// Constructs a new move generator.
    fn new(lookup_table: LookupTable) -> Self {
        MoveGenerator {lookup: lookup_table}
    }
    
    /// Returns the attack bitboard for a given type of piece of the given color for the given position.
    /// 
    /// For example `get_attack_bb(position, Piece::Bishop, Color::White)` will return a bitboard with all squares
    /// set that are attacked by White's bishops.
    fn get_attack_bb(&self, position: Position, piece: Piece, color: Color) -> Bitboard {
        // the result attack_bb
        let mut attack_bb = Bitboard::new(0);
        // the bitboard for the given piece of the given color
        let piece_bb = position.pieces[color.to_index() as usize][piece.to_index() as usize];
        // get squares with pieces on them
        let active_squares = piece_bb.get_active_bits();
        // get blocker bitboard (needed for slider pieces)
        let blockers = position.get_occupancies();

        // loop over active squares and `or` the result attack_bb with the attack bitboard of the piece on each square
        for square in active_squares {
            match piece {
                Piece::Pawn => attack_bb.value |= self.lookup.get_pawn_attacks(square, color).value,
                Piece::Knight => attack_bb.value |= self.lookup.get_knight_attacks(square).value,
                Piece::Bishop => attack_bb.value |= self.lookup.get_bishop_attacks(square, blockers).value,
                Piece::Rook => attack_bb.value |= self.lookup.get_rook_attacks(square, blockers).value,
                Piece::Queen => attack_bb.value |= self.lookup.get_queen_attacks(square, blockers).value,
                Piece::King => attack_bb.value |= self.lookup.get_king_attacks(square).value,
            };
        }
        
        attack_bb
    }
}

#[cfg(test)]
mod tests {
    use crate::board::color::Color;
    use crate::board::fen::parse_fen;
    use crate::board::piece::Piece;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::MoveGenerator;

    #[test]
    fn test_get_attack_bb() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let generator = MoveGenerator::new(lookup);
        
        // -----------------------------------------------------------------------------------------
        // position 1 (starting position)
        // -----------------------------------------------------------------------------------------
        
        let position = parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        let pawn_attack_bb = generator.get_attack_bb(position, Piece::Pawn, Color::Black);
        let knight_attack_bb = generator.get_attack_bb(position, Piece::Knight, Color::Black);
        let bishop_attack_bb = generator.get_attack_bb(position, Piece::Bishop, Color::Black);
        let rook_attack_bb = generator.get_attack_bb(position, Piece::Rook, Color::Black);
        let queen_attack_bb = generator.get_attack_bb(position, Piece::Queen, Color::Black);
        let king_attack_bb = generator.get_attack_bb(position, Piece::King, Color::Black);
        
        assert_eq!(0xff0000000000, pawn_attack_bb.value);
        assert_eq!(6936818859638784, knight_attack_bb.value);
        assert_eq!(0x5a000000000000, bishop_attack_bb.value);
        assert_eq!(0x4281000000000000, rook_attack_bb.value);
        assert_eq!(0x141c000000000000, queen_attack_bb.value);
        assert_eq!(0x2838000000000000, king_attack_bb.value);

        // -----------------------------------------------------------------------------------------
        // position 2
        // -----------------------------------------------------------------------------------------

        let position = parse_fen("6k1/2p2pp1/7p/N7/8/1Pn3P1/4r1qP/R6K w - - 0 29").unwrap().position;
        let pawn_attack_bb = generator.get_attack_bb(position, Piece::Pawn, Color::White);
        let knight_attack_bb = generator.get_attack_bb(position, Piece::Knight, Color::White);
        let bishop_attack_bb = generator.get_attack_bb(position, Piece::Bishop, Color::White);
        let rook_attack_bb = generator.get_attack_bb(position, Piece::Rook, Color::White);
        let queen_attack_bb = generator.get_attack_bb(position, Piece::Queen, Color::White);
        let king_attack_bb = generator.get_attack_bb(position, Piece::King, Color::White);

        assert_eq!(0xa5400000, pawn_attack_bb.value);
        assert_eq!(0x2040004020000, knight_attack_bb.value);
        assert_eq!(0, bishop_attack_bb.value);
        assert_eq!(0x1010101fe, rook_attack_bb.value);
        assert_eq!(0, queen_attack_bb.value);
        assert_eq!(0xc040, king_attack_bb.value);

        // -----------------------------------------------------------------------------------------
        // position 3
        // -----------------------------------------------------------------------------------------

        let position = parse_fen("r7/pb3k2/1p4p1/3N1p2/2P2Np1/3BR3/PP4PP/6K1 b - - 1 28").unwrap().position;
        let pawn_attack_bb = generator.get_attack_bb(position, Piece::Pawn, Color::White);
        let knight_attack_bb = generator.get_attack_bb(position, Piece::Knight, Color::White);
        let bishop_attack_bb = generator.get_attack_bb(position, Piece::Bishop, Color::White);
        let rook_attack_bb = generator.get_attack_bb(position, Piece::Rook, Color::White);
        let queen_attack_bb = generator.get_attack_bb(position, Piece::Queen, Color::White);
        let king_attack_bb = generator.get_attack_bb(position, Piece::King, Color::White);
        
        assert_eq!(0xa00e70000, pawn_attack_bb.value);
        assert_eq!(0x147288229c5000, knight_attack_bb.value);
        assert_eq!(0x2014001422, bishop_attack_bb.value);
        assert_eq!(0x1010101010e81010, rook_attack_bb.value);
        assert_eq!(0, queen_attack_bb.value);
        assert_eq!(0xe0a0, king_attack_bb.value);
    }
}
