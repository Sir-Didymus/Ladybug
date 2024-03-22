use crate::bitboard::Bitboard;

/// This array contains the pre-calculated attack tables for both the white and black pawns.
/// The table allows the move generator to quickly get a bitmap with all the squares a given pawn attacks.
pub const PAWN_ATTACKS: [[Bitboard; 64]; 2] = gen_pawn_attacks();

/// This function will generate the pre-calculated pawn attack tables.
pub const fn gen_pawn_attacks() -> [[Bitboard; 64]; 2] {
    [[Bitboard{value: 0};64]; 2]
}