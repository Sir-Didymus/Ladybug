use crate::board::position::Position;
use crate::move_gen::generates_moves;
use crate::move_gen::ply::Ply;
use crate::search::Search;

impl Search {
    /// This function performs a [Perft](https://www.chessprogramming.org/Perft) (Performance Test).
    /// A perft counts the number of leaf nodes for a fixed depth, and serves two purposes:
    /// - verify that the move generation is working correctly
    /// - measure the speed of the move generation
    pub fn perft(&self, position: Position, depth: u64) -> u64 {
        // used to measure the elapsed time
        let time = std::time::Instant::now();

        // the number of leaf nodes
        let mut node_count: u64 = 0;

        // generate all legal moves for the position
        let move_list: Vec<Ply> = generates_moves(position);

        // call the perft_driver function for all legal moves and add the results to node_count
        for ply in move_list {
            let node_count_inner = self.perft_driver(position.make_move(ply), depth - 1);
            node_count += node_count_inner;
            self.send_output(format!("{ply}: {node_count_inner}"));
        }

        self.send_output(format!("\nSearched {node_count} nodes in {:?}", time.elapsed()));

        node_count
    }

    /// This is the recursive perft driver function, which is required by the `perft` function.
    /// It is used to traverse the tree and count the number of leaf nodes.
    fn perft_driver(&self, position: Position, depth: u64) -> u64 {
        // if depth is zero, return a node count of 1 to break out of the recursion
        if depth == 0 {
            return 1;
        }

        // the number of leaf nodes
        let mut node_count: u64 = 0;

        // generate all legal moves for the position
        let move_list: Vec<Ply> = generates_moves(position);

        // call the perft_driver function recursively for all legal moves and add the results to node_count
        for ply in move_list {
            node_count += self.perft_driver(position.make_move(ply), depth - 1);
        }

        node_count
    }
}

#[cfg(test)]
mod tests {
    //! ----------------------------------------------------------------------------------------------------------------------------------------
    //! This perft test suite is used to verify the correctness of the move generation.
    //! Since perft tests in higher depths and without compiling in release mode can take forever, everything above depth 3 is ignored by default.
    //! To run all tests, use `cargo test --release -- --include-ignored`.
    //! ----------------------------------------------------------------------------------------------------------------------------------------

    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use crate::board::Board;
    use crate::ladybug::Message;
    use crate::lookup::LOOKUP_TABLE;
    use crate::lookup::lookup_table::LookupTable;
    use crate::search::{Search, SearchCommand};

    /// Helper function to initialize the lookup table.
    fn initialize_lookup_table() {
        let mut lookup = LookupTable::default();
        lookup.initialize_tables();
        let _ = LOOKUP_TABLE.set(lookup);
    }
    
    /// Creates a search instance and spawns a test thread that will take the search thread's output.
    /// We don't actually care for the output since we only look at the returned u64 value,
    /// but since the search tries to send the results to the main thread, the receiver must not be dropped.
    fn setup() -> Search {
        // create search_command_sender and search_command_receiver so that the test thread can send commands to the search thread
        let (search_command_sender, search_command_receiver): (Sender<SearchCommand>, Receiver<SearchCommand>) = mpsc::channel();
        
        // create a test_sender and test_receiver so that search thread can send output to the test thread
        let (test_sender, test_receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        // initialize the search
        let search = Search::new(search_command_receiver, test_sender);
        
        // spawn the test thread
        thread::spawn(move || {
            loop {
                let _search_output = test_receiver.recv();
                search_command_sender.send(SearchCommand::Stop).unwrap();
            };
        });
        
        search
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 1 - Starting Position (https://www.chessprogramming.org/Perft_Results#Initial_Position)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // starting position depth 1
    fn perft_position1_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(20, search.perft(position, 1));
    }

    #[test]
    // starting position depth 2
    fn perft_position1_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(400, search.perft(position, 2));
    }

    #[test]
    // starting position depth 3
    fn perft_position1_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(8_902, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // starting position depth 4
    fn perft_position1_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(197_281, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // starting position depth 5
    fn perft_position1_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap().position;
        assert_eq!(4_865_609, search.perft(position, 5));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 2 (https://www.chessprogramming.org/Perft_Results#Position_2)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 2 depth 1
    fn perft_position2_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap().position;
        assert_eq!(48, search.perft(position, 1));
    }

    #[test]
    // position 2 depth 2
    fn perft_position2_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap().position;
        assert_eq!(2039, search.perft(position, 2));
    }

    #[test]
    // position 2 depth 3
    fn perft_position2_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap().position;
        assert_eq!(97_862, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 2 depth 4
    fn perft_position2_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap().position;
        assert_eq!(4_085_603, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 2 depth 5
    fn perft_position2_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap().position;
        assert_eq!(193_690_690, search.perft(position, 5));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 3 (https://www.chessprogramming.org/Perft_Results#Position_3)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 3 depth 1
    fn perft_position3_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap().position;
        assert_eq!(14, search.perft(position, 1));
    }

    #[test]
    // position 3 depth 2
    fn perft_position3_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap().position;
        assert_eq!(191, search.perft(position, 2));
    }

    #[test]
    // position 3 depth 3
    fn perft_position3_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap().position;
        assert_eq!(2_812, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 3 depth 4
    fn perft_position3_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap().position;
        assert_eq!(43_238, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 3 depth 5
    fn perft_position3_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap().position;
        assert_eq!(674_624, search.perft(position, 5));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 4 (https://www.chessprogramming.org/Perft_Results#Position_4)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 4 depth 1
    fn perft_position4_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap().position;
        assert_eq!(6, search.perft(position, 1));
    }

    #[test]
    // position 4 depth 2
    fn perft_position4_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap().position;
        assert_eq!(264, search.perft(position, 2));
    }

    #[test]
    // position 4 depth 3
    fn perft_position4_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap().position;
        assert_eq!(9_467, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 4 depth 4
    fn perft_position4_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap().position;
        assert_eq!(422_333, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 4 depth 5
    fn perft_position4_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap().position;
        assert_eq!(15_833_292, search.perft(position, 5));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 5 (https://www.chessprogramming.org/Perft_Results#Position_5)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 5 depth 1
    fn perft_position5_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap().position;
        assert_eq!(44, search.perft(position, 1));
    }

    #[test]
    // position 5 depth 2
    fn perft_position5_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap().position;
        assert_eq!(1_486, search.perft(position, 2));
    }

    #[test]
    // position 5 depth 3
    fn perft_position5_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap().position;
        assert_eq!(62_379, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 5 depth 4
    fn perft_position5_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap().position;
        assert_eq!(2_103_487, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 5 depth 5
    fn perft_position5_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap().position;
        assert_eq!(89_941_194, search.perft(position, 5));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 6 (https://www.chessprogramming.org/Perft_Results#Position_6)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 6 depth 1
    fn perft_position6_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap().position;
        assert_eq!(46, search.perft(position, 1));
    }

    #[test]
    // position 6 depth 2
    fn perft_position6_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap().position;
        assert_eq!(2_079, search.perft(position, 2));
    }

    #[test]
    // position 6 depth 3
    fn perft_position6_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap().position;
        assert_eq!(89_890, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 6 depth 4
    fn perft_position6_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap().position;
        assert_eq!(3_894_594, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 6 depth 5
    fn perft_position6_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap().position;
        assert_eq!(164_075_551, search.perft(position, 5));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 7 - promotion
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 7 depth 1
    fn perft_position7_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap().position;
        assert_eq!(24, search.perft(position, 1));
    }

    #[test]
    // position 7 depth 2
    fn perft_position7_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap().position;
        assert_eq!(496, search.perft(position, 2));
    }

    #[test]
    // position 7 depth 3
    fn perft_position7_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap().position;
        assert_eq!(9_483, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 7 depth 4
    fn perft_position7_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap().position;
        assert_eq!(182_838, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 7 depth 5
    fn perft_position7_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap().position;
        assert_eq!(3_605_103, search.perft(position, 5));
    }

    #[test]
    #[ignore]
    // position 7 depth 6
    fn perft_position7_depth6() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("n1n5/PPPk4/8/8/8/8/4Kppp/5N1N b - - 0 1").unwrap().position;
        assert_eq!(71_179_139, search.perft(position, 6));
    }


    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 8 - avoid illegal en passant capture - white (https://www.talkchess.com/forum3/viewtopic.php?f=7&t=47318)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 8 depth 1
    fn perft_position8_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(8, search.perft(position, 1));
    }

    #[test]
    // position 8 depth 2
    fn perft_position8_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(104, search.perft(position, 2));
    }

    #[test]
    // position 8 depth 3
    fn perft_position8_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(736, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 8 depth 4
    fn perft_position8_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(9_287, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 8 depth 5
    fn perft_position8_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(62_297, search.perft(position, 5));
    }

    #[test]
    #[ignore]
    // position 8 depth 6
    fn perft_position8_depth6() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(824_064, search.perft(position, 6));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 9 - avoid illegal en passant capture - black (https://www.talkchess.com/forum3/viewtopic.php?f=7&t=47318)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 9 depth 1
    fn perft_position9_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1").unwrap().position;
        assert_eq!(8, search.perft(position, 1));
    }

    #[test]
    // position 9 depth 2
    fn perft_position9_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1").unwrap().position;
        assert_eq!(104, search.perft(position, 2));
    }

    #[test]
    // position 9 depth 3
    fn perft_position9_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1").unwrap().position;
        assert_eq!(736, search.perft(position, 3));
    }

    #[test]
    // position 9 depth 4
    fn perft_position9_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1").unwrap().position;
        assert_eq!(9_287, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 9 depth 5
    fn perft_position9_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1").unwrap().position;
        assert_eq!(62_297, search.perft(position, 5));
    }

    #[test]
    #[ignore]
    // position 9 depth 6
    fn perft_position9_depth6() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1").unwrap().position;
        assert_eq!(824_064, search.perft(position, 6));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 10 - en passant capture checks opponent - white (https://www.talkchess.com/forum3/viewtopic.php?f=7&t=47318)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 10 depth 1
    fn perft_position10_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(15, search.perft(position, 1));
    }

    #[test]
    // position 10 depth 2
    fn perft_position10_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(126, search.perft(position, 2));
    }

    #[test]
    // position 10 depth 3
    fn perft_position10_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(1_928, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 10 depth 4
    fn perft_position10_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(13_931, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 10 depth 5
    fn perft_position10_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(20_6379, search.perft(position, 5));
    }

    #[test]
    #[ignore]
    // position 10 depth 6
    fn perft_position10_depth6() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1").unwrap().position;
        assert_eq!(1_440_467, search.perft(position, 6));
    }

    // ----------------------------------------------------------------------------------------------------------------------------------------
    // Position 11 - en passant capture checks opponent - black (https://www.talkchess.com/forum3/viewtopic.php?f=7&t=47318)
    // ----------------------------------------------------------------------------------------------------------------------------------------
    #[test]
    // position 11 depth 1
    fn perft_position11_depth1() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap().position;
        assert_eq!(15, search.perft(position, 1));
    }

    #[test]
    // position 11 depth 2
    fn perft_position11_depth2() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap().position;
        assert_eq!(126, search.perft(position, 2));
    }

    #[test]
    // position 11 depth 3
    fn perft_position11_depth3() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap().position;
        assert_eq!(1_928, search.perft(position, 3));
    }

    #[test]
    #[ignore]
    // position 11 depth 4
    fn perft_position11_depth4() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap().position;
        assert_eq!(13_931, search.perft(position, 4));
    }

    #[test]
    #[ignore]
    // position 11 depth 5
    fn perft_position11_depth5() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap().position;
        assert_eq!(20_6379, search.perft(position, 5));
    }

    #[test]
    #[ignore]
    // position 11 depth 6
    fn perft_position11_depth6() {
        initialize_lookup_table();
        let search = setup();
        
        let position = Board::from_fen("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1").unwrap().position;
        assert_eq!(1_440_467, search.perft(position, 6));
    }
}