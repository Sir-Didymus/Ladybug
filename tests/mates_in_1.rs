mod common;

#[test]
fn puzzle_1() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "8/8/1Q6/8/7B/2R4N/5K1P/k7 w - - 11 70", 1);
    common::assert_result(&receiver, 1, "bestmove c3a3");
}

// This puzzle is ridiculous.
#[test]
#[ignore]
fn puzzle_2() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "8/8/pppppppK/NBBR1NRp/nbbrqnrP/PPPPPPPk/8/Q7 w - - 0 1", 1);
    common::assert_result(&receiver, 1, "bestmove a1h1");
}

#[test]
fn puzzle_3() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "2k5/1p3R2/p2Bp3/P3P3/4bP2/2P3n1/4B2r/6K1 b - - 1 1", 1);
    common::assert_result(&receiver, 1, "bestmove h2g2");
}

#[test]
fn puzzle_4() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "8/R5p1/5p1p/4r1k1/6P1/5KP1/8/8 w - - 1 2", 1);
    common::assert_result(&receiver, 1, "bestmove a7g7");
}

#[test]
fn puzzle_5() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "2r3k1/1Q4p1/4p2p/8/p4P2/1n5P/1B3KP1/1q6 w - - 0 2", 1);
    common::assert_result(&receiver, 1, "bestmove b7g7");
}

#[test]
fn puzzle_6() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "5b1k/5p2/3ppN1p/2n3p1/2P3P1/2n1P2P/2Q2P1B/4qBK1 w - - 1 2", 1);
    common::assert_result(&receiver, 1, "bestmove c2h7");
}

#[test]
fn puzzle_7() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "8/5p1p/2p2kp1/p1P5/B2p1P2/P5Pb/1P2RK1P/3r4 b - - 1", 1);
    common::assert_result(&receiver, 1, "bestmove d1f1");
}

#[test]
fn puzzle_8() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "8/7r/8/k1B5/2K5/8/8/1R6 w - - 1 2", 1);
    common::assert_result(&receiver, 1, "bestmove b1a1");
}

#[test]
fn puzzle_9() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "8/2p5/p1k5/1pP1K3/1P1Qp3/P6q/5P2/8 w - - 0 2", 1);
    common::assert_result(&receiver, 1, "bestmove d4d5");
}

#[test]
fn puzzle_10() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "4N1k1/5p2/6p1/p5Q1/2p4P/3n2P1/5PbK/q7 b - - 1 1", 1);
    common::assert_result(&receiver, 1, "bestmove a1h1");
}

#[test]
fn puzzle_11() {
    let (sender, receiver) =  common::setup();

    common::go_position(&sender, "bNB1K3/krN5/rq6/pp1ppppp/PPpBPPPn/2PPb3/2QR4/n1R5 w - - 9 62", 1);
    common::assert_result(&receiver, 1, "bestmove b8c6");
}