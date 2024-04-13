use std::time::Duration;
use crate::board::position::Position;
use crate::{evaluation, move_gen};
use crate::search::Search;

impl Search {
    /// Search the given position with iterative deepening.
    pub fn iterative_search(&mut self, position: Position, max_depth: u64, time_limit: Duration) {
        // initialize the best move with a random one, in case the search is stopped immediately
        let mut prev_best_move = move_gen::generates_moves(position)[0];

        // start the timer
        self.instant = Some(std::time::Instant::now());

        for depth in 1..=max_depth {
            self.negamax(position, depth, 0, time_limit);

            match self.best_move {
                None => {
                    // the search was stopped before it could complete
                    self.send_output(format!("info string cancelled search at depth {depth}"));
                    break
                }
                Some(best_move) => {
                    // send the information for the current iteration
                    let mut output = format!("info depth {depth} pv");
                    for ply_num in 0..self.pv_length[0] {
                        output += format!(" {}", self.pv_table[0][ply_num as usize]).as_str();
                    }
                    self.send_output(output);
                    
                    // set the best move of the previous iteration to the new best move
                    prev_best_move = best_move;
                    self.best_move = None;
                }
            }
        }

        // reset the timer
        self.instant = None;

        self.send_output(format!("bestmove {}", prev_best_move));
    }

    /// A basic implementation of the [negamax](https://www.chessprogramming.org/Negamax) algorithm.
    ///
    /// Instead of implementing two routines for the maximizing and minimizing players, this method
    /// negates the scores for each recursive call, making minimax easier to implement.
    pub fn negamax(&mut self, position: Position, depth: u64, ply_index: u64, time_limit: Duration) -> i32 {
        // initialize the pv length
        self.pv_length[ply_index as usize] = ply_index as u8;

        // check if the time limit has expired
        if let Some(instant) = self.instant {
            if instant.elapsed() > time_limit {
                // reset the best move of this search to let the caller know that the search was cancelled prematurely
                self.best_move = None;
                return 0;
            }
        }

        // the maximum score that can be reached in this position
        let mut max_score = evaluation::NEGATIVE_INFINITY;

        // generate all legal moves for the current position
        let moves = move_gen::generates_moves(position);

        // if there are no legal moves, check for mate or stalemate
        if moves.is_empty() {
            return if position.is_in_check(position.color_to_move) {
                // In case of checkmate, return a large negative number.
                // By adding a large number (larger than the worth of a queen) for each ply in the search tree, 
                // and thus decreasing the penalty for getting checkmated, the engine is incentivised to sacrifice material in order to delay checkmate.
                // It will also prefer shorter mates when being on the winning side.
                evaluation::NEGATIVE_INFINITY + (ply_index as i32 * 5000)
            } else {
                0
            };
        }

        // if depth 0 is reached, break out of the recursion by returning the static evaluation of the position
        if depth == 0 {
            return evaluation::evaluate(position);
        }

        // make all moves and call negamax recursively for the arising positions
        for ply in moves {
            // the score of the position arising after playing the move
            let score = -self.negamax(position.make_move(ply), depth - 1, ply_index + 1, time_limit);
            // check if the score of the position is better than the current max score
            if score > max_score {
                // update the max score
                max_score = score;
                
                // --------------------
                // update the pv table
                // --------------------
                self.pv_table[ply_index as usize][ply_index as usize] = ply;
                for next_ply_index in (ply_index + 1) as u8..self.pv_length[ply_index as usize + 1] {
                    self.pv_table[ply_index as usize][next_ply_index as usize] = self.pv_table[ply_index as usize + 1][next_ply_index as usize];
                }
                self.pv_length[ply_index as usize] = self.pv_length[ply_index as usize + 1];

                // we're at the root node - update the best move
                if ply_index == 0 {
                    self.best_move = Some(ply);
                }
            }
        }
        max_score
    }
}