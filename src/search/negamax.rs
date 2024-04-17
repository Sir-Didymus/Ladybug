use std::time::Duration;
use crate::board::position::Position;
use crate::{evaluation, move_gen};
use crate::evaluation::{NEGATIVE_INFINITY, POSITIVE_INFINITY};
use crate::search::{MAX_PLY, Search};

impl Search {
    /// Search the given position with iterative deepening.
    pub fn iterative_search(&mut self, position: Position, max_depth: u64, time_limit: Duration) {
        // reset the stop flag to allow searching
        self.stop = false;

        // start the total time
        self.total_time = Some(std::time::Instant::now());

        // initialize the best move to the first legal one, in case the search stops prematurely
        let mut best_move = move_gen::generate_moves(position).get(0);

        // start at depth 1 and increment the depth until the max depth is reached or the time runs out
        for depth in 1..=max_depth {
            // set the start time for this iteration
            let iteration_time = std::time::Instant::now();
            
            // search to the current depth and save the score
            let score = self.negamax(position, depth, 0, NEGATIVE_INFINITY, POSITIVE_INFINITY, time_limit);

            if self.stop {
                // if the stop flag is set, break out of iterative deepening immediately
                break;
            }

            // calculate nodes per second
            let mut nps: u128 = 0;
            let iteration_time_elapsed = iteration_time.elapsed().as_millis();
            if iteration_time_elapsed > 0 {
                nps = (self.search_info.node_count / iteration_time_elapsed) * 1000;
            }
            else {
                nps = self.search_info.node_count;
            }

            // send the information for the current iteration
            let mut output = format!("info depth {depth} score cp {score} nodes {nodes} time {iteration_time_elapsed} nps {nps} pv", nodes = self.search_info.node_count);
            for ply_num in 0..self.search_info.pv_length[0] {
                output += format!(" {}", self.search_info.pv_table[0][ply_num as usize]).as_str();
            }
            self.send_output(output);

            // set the best move to the result of this iteration
            best_move = self.search_info.pv_table[0][0];

            // clear the search info for this iteration
            self.search_info.clear_iteration();
        }

        // send the best move to the main thread
        self.send_output(format!("bestmove {}", best_move));

        // reset the total time
        self.total_time = None;

        // clear all search info
        self.search_info.clear_all();
    }

    /// A basic implementation of the [negamax](https://www.chessprogramming.org/Negamax) algorithm with alpha beta pruning.
    ///
    /// Instead of implementing two routines for the maximizing and minimizing players, this method
    /// negates the scores for each recursive call, making minimax easier to implement.
    pub fn negamax(&mut self, position: Position, depth: u64, ply_index: u64, mut alpha: i32, beta: i32, time_limit: Duration) -> i32 {
        // check if the max ply number is reached
        if ply_index as usize >= MAX_PLY {
            // the maximum number of plies is reached - return static evaluation to avoid overflows
            return evaluation::evaluate(position);
        }

        // check if the time limit is reached
        if let Some(instant) = self.total_time {
            if instant.elapsed() > time_limit {
                // the time limit is reached - break out of recursion immediately
                self.stop = true;
                return 0;
            }
        }

        // set the pv length
        self.search_info.pv_length[ply_index as usize] = ply_index as u8;

        // generate all legal moves for the current position
        let mut move_list = move_gen::generate_moves(position);

        // sort the  move list
        move_list.sort(&mut self.search_info, ply_index);

        // if there are no legal moves, check for mate or stalemate
        if move_list.is_empty() {
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

        // if depth 0 is reached, start the quiescence search
        if depth == 0 {
            return self.quiescence_search(position, ply_index, alpha, beta, time_limit);
        }

        // iterate over all possible moves and call negamax recursively for the arising positions
        for i in 0..move_list.len() {
            let ply = move_list.get(i);
            
            // the score of the new position
            let score = -self.negamax(position.make_move(ply), depth - 1, ply_index + 1, -beta, -alpha, time_limit);

            // fail-hard beta cutoff
            if score >= beta {
                // move fails high - the opponent won't allow this move because it's too good

                // check if move is a quiet move
                if ply.captured_piece.is_none() {
                    // store the killer moves
                    self.search_info.killer_moves[1][ply_index as usize] = self.search_info.killer_moves[0][ply_index as usize];
                    self.search_info.killer_moves[0][ply_index as usize] = ply;
                }
                return beta;
            }
            
            // found a better move
            if score > alpha {
                // update alpha to the better score
                alpha = score;
                
                // check if move is a quiet move
                if ply.captured_piece.is_none() {
                    // store history move bonus
                    // moves closer to the root get a bigger bonus
                    self.search_info.history_moves[ply.piece.to_index() as usize][ply.target.index as usize] = depth as i32;
                }

                // update the pv table
                self.search_info.pv_table[ply_index as usize][ply_index as usize] = ply;
                for next_ply_index in (ply_index + 1) as u8..self.search_info.pv_length[ply_index as usize + 1] {
                    self.search_info.pv_table[ply_index as usize][next_ply_index as usize] = self.search_info.pv_table[ply_index as usize + 1][next_ply_index as usize];
                }
                self.search_info.pv_length[ply_index as usize] = self.search_info.pv_length[ply_index as usize + 1];
            }
            
            // move fails low
            // if score < alpha, it means we have already found a better move
        }
        alpha
    }
}