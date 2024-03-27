/// Contains information on what (if any) castling rights a player has.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CastlingRights {
    NoRights,
    KingSide,
    QueenSide,
    Both,
}