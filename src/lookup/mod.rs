//! This module is responsible for generating the lookup tables used by the move generator.
//! The submodule `lookup_table` contains a struct to store these tables, while the generation logic
//! is provided by functions in submodules such as `pawn_attacks` or `knight_attacks`.

pub mod lookup_table;
pub mod pawn_attacks;

