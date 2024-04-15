use arrayvec::ArrayVec;
use crate::move_gen::ply::Ply;

/// The move can hold up to 255 ply, encoded as unsigned 32-bit integers.
pub struct MoveList {
    /// The array of encoded moves.
    moves: ArrayVec<u32, 255>,
}

impl Default for MoveList {
    /// Constructs a new move list.
    fn default() -> Self{
        MoveList {
            moves: ArrayVec::new(),
        }
    }
}

impl MoveList {
    /// Adds a ply to the move list.
    pub fn push(&mut self, ply: Ply) {
        self.moves.push(ply.encode());
    }

    /// Returns the ply with the given index.
    pub fn get(&self, index: u8) -> Ply {
        Ply::decode(self.moves[index as usize])
    }
    
    
    /// Returns the length of the move list.
    pub fn len(&self) -> u8 {
        self.moves.len() as u8
    }
    
    /// Returns true if the move list ist empty.
    pub fn is_empty(&self) -> bool {
        self.moves.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::board::piece::Piece;
    use crate::board::square;
    use crate::move_gen::move_list::MoveList;
    use crate::move_gen::ply::Ply;

    #[test]
    fn test_move_list() {
        let ply1 = Ply {source: square::A1, target: square::A2, piece: Piece::Rook, captured_piece: None, promotion_piece: None};
        let ply2 = Ply {source: square::H8, target: square::A8, piece: Piece::Rook, captured_piece: Some(Piece::Rook), promotion_piece: None};
        let ply3 = Ply {source: square::E4, target: square::D5, piece: Piece::Pawn, captured_piece: Some(Piece::Pawn), promotion_piece: None};
        let ply4 = Ply {source: square::G7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)};
        let ply5 = Ply {source: square::H3, target: square::C8, piece: Piece::Bishop, captured_piece: Some(Piece::Rook), promotion_piece: None};
        
        let mut move_list = MoveList::default();
        assert_eq!(0, move_list.len());
        assert!(move_list.is_empty());
        
        move_list.push(ply1);
        assert_eq!(1, move_list.len());
        assert_eq!(ply1, move_list.get(0));
        assert!(!move_list.is_empty());
        
        move_list.push(ply1);
        move_list.push(ply2);
        move_list.push(ply3);
        move_list.push(ply4);
        move_list.push(ply5);
        
        assert_eq!(6, move_list.len());
        assert_eq!(ply1, move_list.get(0));
        assert_eq!(ply1, move_list.get(1));
        assert_eq!(ply2, move_list.get(2));
        assert_eq!(ply3, move_list.get(3));
        assert_eq!(ply4, move_list.get(4));
        assert_eq!(ply5, move_list.get(5));
        
        let mut move_list = MoveList::default();
        for _i in 0..255 {
            move_list.push(Ply {source: square::G7, target: square::H8, piece: Piece::Pawn, captured_piece: Some(Piece::Queen), promotion_piece: Some(Piece::Knight)});
        }
        assert!(!move_list.is_empty());
        assert_eq!(255, move_list.len());
    }
}