use crate::bitboard::Bitboard;

mod bitboard;
mod square;
mod position;
mod file;
mod rank;
mod color;
mod lookup_table;
mod pawn_attacks;

fn main() {
    let bitboard = Bitboard::new(9223372054036742144);
    println!("{}",bitboard);
}
