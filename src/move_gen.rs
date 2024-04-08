//! The move_gen module is responsible for the move generation logic.

use crate::board::position::Position;
use crate::move_gen::leaper_moves::generate_leaper_moves;
use crate::move_gen::pawn_moves::generate_pawn_moves;
use crate::move_gen::ply::Ply;
use crate::move_gen::slider_moves::generate_slider_moves;

pub mod ply;
pub mod perft;
mod pawn_moves;
mod slider_moves;
mod leaper_moves;

/// Generates all legal moves for the given position.
pub fn generates_moves(position: Position) -> Vec<Ply> {
    let mut move_list: Vec<Ply> = Vec::new();
    move_list.append(&mut generate_pawn_moves(position));
    move_list.append(&mut generate_slider_moves(position));
    move_list.append(&mut generate_leaper_moves(position));
    move_list
}