
use super::{Piece, Kaleidoscope, Move, Strategy};
use std::collections::VecDeque;

pub struct Solver {
    pub game: Kaleidoscope,
    pub strategy: Strategy,
    pub moves: u128,
    possible: VecDeque<VecDeque<Move>>,   // possible moves for each piece
    debug: bool
}

impl Solver {
    pub fn new(game: Kaleidoscope, strategy: Strategy) -> Self {
        
        let first_piece_idx = strategy[0];
        let mut possible = VecDeque::new();
        possible.push_back(game.possible(first_piece_idx));

        Self {
            game,
            strategy,
            moves: 0,
            possible,
            debug: false
        }
    }

    pub fn solve(&mut self) -> bool {

        // NOTE TO SELF: the bug is with removing one too many pieces too early
        while self.possible.len() > 0 {

            let curr_move = self.possible.len() - 1;
            let curr_piece_idx = self.strategy[curr_move];
            
            // if there are no available moves for the current piece
            if self.possible[curr_move].len() == 0 {

                // undo the last move, if possible
                if curr_move != 0 {
                    self.game.remove(self.strategy[curr_move - 1]);
                }

                // remove the empty list of moves from the list of possible moves
                self.possible.pop_back();
                continue;
            }

            // place the first possible current move on the board
            let move_ = self.possible[curr_move].pop_front().unwrap();
            self.game.set(curr_piece_idx, move_);

            // exit condition
            if curr_move == self.strategy.len() - 1 && self.game.is_solved() {
                return true;
            }

            // get the next piece's possible moves
            let next_move = curr_move + 1;
            let next_piece_idx = self.strategy[next_move];
            let next_moves = self.game.possible(next_piece_idx);
            self.possible.push_back(next_moves);

            if self.debug {
                self.game.print();
                println!("{} {} {} {:?}", self.moves, curr_move, curr_piece_idx, self.possible);
            }

            self.moves += 1;

            // if self.possible.len() >= 8 && 
            //     self.possible[0].len() == 3 &&
            //     self.possible[1].len() == 2 &&
            //     self.possible[2].len() == 7 && 
            //     self.possible[3].len() == 11 &&
            //     self.possible[4].len() == 4 && 
            //     self.possible[5].len() == 3 &&
            //     self.possible[6].len() == 3 && 
            //     self.possible[7].len() == 2 {
            //     return false;
            // }

            // // if self.moves > 11750 {
            // //     return false;
            // // }
        }

        return false;
    }
}