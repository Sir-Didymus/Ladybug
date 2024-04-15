use std::time::Duration;
use crate::board::position::Position;
use crate::{evaluation, move_gen};
use crate::search::{Search};

impl Search {
    pub fn quiescence_search(&mut self, position: Position, mut alpha: i32, beta: i32, time_limit: Duration) -> i32 {
        // check if the time limit is reached
        if let Some(instant) = self.total_time {
            if instant.elapsed() > time_limit {
                // the time limit is reached - break out of recursion immediately
                self.stop = true;
                return 0;
            }
        }

        // increment the number of nodes searched
        self.node_count += 1;
        
        // Establish the lower bound of the score with the static evaluation
        let standing_pat = evaluation::evaluate(position);
        
        // fail-hard beta cutoff
        if standing_pat >= beta {
            // move fails high - the opponent won't allow this move because it's too good
            return beta;
        }

        // found a better move
        if standing_pat > alpha {
            // update alpha to the better score
            alpha = standing_pat;
        }
        
        // generate all legal capture moves for the current position
        let capture_list = move_gen::generate_moves(position).get_captures();

        // iterate over all capture moves and call the quiescence search recursively for the arising positions
        for i in 0..capture_list.len() {
            let ply = capture_list.get(i);

            // the score of the new position
            let score = -self.quiescence_search(position.make_move(ply), -beta, -alpha, time_limit);

            // fail-hard beta cutoff
            if score >= beta {
                // move fails high - the opponent won't allow this move because it's too good
                return beta;
            }

            // found a better move
            if score > alpha {
                // update alpha to the better score
                alpha = score;
            }

            // move fails low
            // if score < alpha, it means we have already found a better move
        }
        alpha
    }
}