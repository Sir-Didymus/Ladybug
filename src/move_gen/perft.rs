use crate::board::position::Position;
use crate::move_gen::pawn_moves::generate_pawn_moves;
use crate::move_gen::ply::Ply;

/// This function performs a [Perft](https://www.chessprogramming.org/Perft) (Performance Test).
/// A perft counts the number of leaf nodes for a fixed depth, and serves two purposes:
/// - verify that the move generation is working correctly
/// - measure the speed of the move generation
pub fn perft(position: Position, depth: u64) -> u64 {
    // used to measure the elapsed time
    let time = std::time::Instant::now();
    
    // the number of leaf nodes
    let mut node_count: u64 = 0;

    // generate all legal moves for the position
    let move_list: Vec<Ply> = generate_pawn_moves(position);

    // call the perft_driver function for all legal moves and add the results to node_count
    for ply in move_list {
        let node_count_inner = perft_driver(position.make_move(ply), depth  - 1);
        node_count += node_count_inner;
        println!("{ply}: {node_count_inner}");
    }
    
    println!("\nSearched {node_count} nodes in {:?}", time.elapsed());

    node_count
}

/// This is the recursive perft driver function, which is required by the `perft` function.
/// It is used to traverse the tree and count the number of leaf nodes.
fn perft_driver(position: Position, depth: u64) -> u64 {
    // if depth is zero, return a node count of 1 to break out of the recursion
    if depth == 0 {
        return 1
    }

    // the number of leaf nodes
    let mut node_count: u64 = 0;

    // generate all legal moves for the position
    let move_list: Vec<Ply> = generate_pawn_moves(position);

    // call the perft_driver function recursively for all legal moves and add the results to node_count
    for ply in move_list {
        node_count += perft_driver(position.make_move(ply), depth - 1);
    }

    node_count
}

#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::move_gen::perft::perft;

    #[test]
    fn test_perft() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);

        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        perft(position, 4);
    }
}