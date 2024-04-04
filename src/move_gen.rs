//! The move_gen module is responsible for the move generation logic.
use crate::lookup::lookup_table::LookupTable;

pub mod ply;
mod pawn_moves;

/// This is the move generator. It can generate all legal moves (ply) for a given position.
/// For the move generator to work, it has to receive an initialized lookup table.
pub struct MoveGenerator {
    lookup: LookupTable,
}

impl MoveGenerator {
    /// Constructs a new move generator.
    pub fn new(lookup_table: LookupTable) -> Self {
        MoveGenerator { lookup: lookup_table }
    }
}

#[cfg(test)]
mod tests {}
