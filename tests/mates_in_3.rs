mod common;

#[test]
fn puzzle_1() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "2r3k1/p4p2/3Rp2p/1p2P1pK/8/1P4P1/P3Q2P/1q6 b - - 0 1", 5);
    common::assert_result(&receiver, 5, "bestmove b1g6");

    common::go_position(&sender, "2r3k1/p4p2/3Rp1qp/1p2P1p1/6K1/1P4P1/P3Q2P/8 b - - 2 2", 5);
    common::assert_result(&receiver, 5, "bestmove g6f5");

    common::go_position(&sender, "2r3k1/p4p2/3Rp2p/1p2PqpK/8/1P4P1/P3Q2P/8 b - - 4 3", 5);
    common::assert_result(&receiver, 5, "bestmove f5h3");
}