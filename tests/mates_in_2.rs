use ladybug::ladybug::Message;

mod common;

#[test]
fn puzzle_1() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/8/2p2K1p/2P2p1k/3R2p1/6P1/5P1P/8 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove f2f4"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/8/2p2K1p/2P2p1k/3R4/5pP1/7P/8 w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove d4h4"));
}

#[test]
fn puzzle_2() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen Q4rkr/1p3p1p/7P/R2Bp3/8/8/4KP1p/8 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove d5h1"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen Q4rkr/1p3p1p/7P/R7/4p3/8/4KP1p/7B w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove a5g5"));
}

#[test]
fn puzzle_3() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen rk2K3/NPR5/8/8/8/8/8/4Q3 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove e1b4"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen r3K3/NPk5/8/8/1Q6/8/8/8 w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove b7a8n"));
}

#[test]
fn puzzle_4() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 2kr3r/pp1nb1pp/2p2n2/4B3/2B5/2N2Q1P/P1q3P1/4RRK1 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove f3c6"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 2kr3r/p2nb1pp/2p2n2/4B3/2B5/2N4P/P1q3P1/4RRK1 w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove c4a6"));
}

#[test]
fn puzzle_5() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 6rk/6pp/8/8/8/2Q5/7P/6RK w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove g1g6"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 6rk/6p1/6Rp/8/8/2Q5/7P/7K w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove g6h6"));
}

#[test]
fn puzzle_6() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/4Q3/8/8/8/4K3/8/4k3 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove e7e4"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/8/8/8/4Q3/4K3/8/3k4 w - - 2 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove e4b1"));
}

#[test]
fn puzzle_7() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen R5rk/6qp/4K3/4B3/8/8/8/7Q w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove e5a1"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen R5rk/6q1/4K2p/8/8/8/8/B6Q w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove h1h6"));
}

#[test]
fn puzzle_8() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/1N2N3/2r5/3qp2R/QP2kp1K/5R2/6B1/6B1 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove a4a8"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen Q7/1N2N3/8/3qp2R/1P2kp1K/5R2/2r3B1/6B1 w - - 2 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove b7d6"));
}

#[test]
fn puzzle_9() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/5p2/8/8/1B3K2/6N1/5kpQ/8 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove h2h7"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 8/5p1Q/8/8/1B3K2/6N1/5k2/6q1 w - - 0 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove h7c2"));
}

#[test]
fn puzzle_10() {
    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 2R4N/2p1k1pp/1pBbp3/3n4/1p1P4/3Q4/1PP2PPP/2B2RK1 w - - 0 1".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove c1g5"));

    let (sender, receiver) =  common::setup();
    let _ = sender.send(Message::ConsoleMessage("position fen 2R4N/2p1k1pp/1pBbpn2/6B1/1p1P4/3Q4/1PP2PPP/5RK1 w - - 2 2".to_string()));
    let _ = sender.send(Message::ConsoleMessage("go depth 3".to_string()));

    let solution = receiver.recv().unwrap();
    println!("{}", solution.as_str());
    assert!(solution.contains("bestmove c8e8"));
}