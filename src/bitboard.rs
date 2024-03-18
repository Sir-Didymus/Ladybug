/// A bitboard representing the state of the board for one type of piece of one color.
pub struct Bitboard {
    pub value: u64,
}

impl Bitboard {
    /// Constructs a new Bitboard from an u64.
    pub fn new(value: u64) -> Bitboard {
        Bitboard {value}
    }
}