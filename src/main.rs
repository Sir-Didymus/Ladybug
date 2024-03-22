use crate::bitboard::Bitboard;

mod bitboard;
mod square;
mod position;
mod file;
mod rank;
mod color;

fn main() {
    let bitboard = Bitboard::new(9223372054036742144);
    println!("{}",bitboard);
}
