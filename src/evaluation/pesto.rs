use crate::board::color::{Color, NUM_COLORS};
use crate::board::piece::{NUM_PIECES, Piece};
use crate::board::position::Position;
use crate::evaluation::{psqt};

/// Used to calculate the game phase.
const GAMEPHASE_INC: [i32; 6] = [0, 1, 1, 2, 4, 0];

/// Performs a tapered evaluation based on piece square tables only.
pub fn evaluate(position: Position) -> i32 {
    let mut game_phase = 0;
    let mut mg_score = 0;
    let mut eg_score = 0;

    // calculate the middlegame and endgame scores
    for color_index in 0..NUM_COLORS {
        for piece_index in 0..NUM_PIECES {
            let active_bits = position.pieces[color_index as usize][piece_index as usize].get_active_bits();
            for square in active_bits {
                if Color::from_index(color_index) == position.color_to_move {
                    mg_score += psqt::get_mg_value(Piece::from_index(piece_index), square, Color::from_index(color_index));
                    eg_score += psqt::get_eg_value(Piece::from_index(piece_index), square, Color::from_index(color_index));
                } else {
                    mg_score -= psqt::get_mg_value(Piece::from_index(piece_index), square, Color::from_index(color_index));
                    eg_score -= psqt::get_eg_value(Piece::from_index(piece_index), square, Color::from_index(color_index));
                }
                // increment the game phase based on the piece type
                game_phase += GAMEPHASE_INC[piece_index as usize];
            }
        }
    }

    let mut mg_phase = game_phase;

    // in case of early promotion
    if mg_phase > 24 {
        mg_phase = 24;
    }

    let eg_phase = 24 - mg_phase;

    (mg_score * mg_phase + eg_score * eg_phase) / 24
}