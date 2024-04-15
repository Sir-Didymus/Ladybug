//! The move_gen module is responsible for the move generation logic.

use crate::board::position::Position;
use crate::move_gen::leaper_moves::generate_leaper_moves;
use crate::move_gen::move_list::MoveList;
use crate::move_gen::pawn_moves::generate_pawn_moves;
use crate::move_gen::slider_moves::generate_slider_moves;

pub mod ply;
pub mod move_list;
mod pawn_moves;
mod slider_moves;
mod leaper_moves;

/// Generates all legal moves for the given position.
pub fn generate_moves(position: Position) -> MoveList {
    let mut move_list = MoveList::default();
    generate_pawn_moves(position, &mut move_list);
    generate_leaper_moves(position, &mut move_list);
    generate_slider_moves(position, &mut move_list);
    move_list.sort();
    move_list
}