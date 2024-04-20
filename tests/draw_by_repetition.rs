mod common;

#[test]
#[ignore]
fn puzzle_1() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "qq3rk1/ppp1p2p/3p2p1/8/8/3Q4/2Q3PK/8 w - - 0 1", 11);
    common::assert_result(&receiver, 11, "bestmove d3g6");
}

#[test]
#[ignore]
fn puzzle_2() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "3Q4/kp4p1/2p1r3/8/P2P4/2PP1PK1/4r2P/8 b - - 0 40", 9);
    common::assert_result(&receiver, 9, "bestmove e6g6");
}