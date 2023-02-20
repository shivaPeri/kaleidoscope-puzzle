
use super::{Piece, Kaleidoscope, Move, Strategy};
use std::collections::VecDeque;

pub struct Solver {
    pub game: Kaleidoscope,
    pub strategy: Strategy,
    pub moves: u128,
    possible: VecDeque<VecDeque<Move>>,   // possible moves for each piece
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
            possible
        }
    }

    pub fn solve(&mut self) -> bool {

        
        while self.possible.len() > 0 && self.possible[0].len() > 0 {
            // println!("{} {:?}", self.moves, self.possible);

            let curr_move = self.possible.len() - 1;
            let curr_piece_idx = self.strategy[curr_move];
            self.game.remove(curr_piece_idx);
            let next_move = self.possible.len();

            if self.possible[curr_move].len() == 0 {
                self.game.remove(curr_piece_idx);
                self.possible.pop_back();
                continue;
            }


            let move_ = self.possible[curr_move].pop_front().unwrap();
            // println!("{:?} {:?}", move_, self.possible);
            self.game.set(curr_piece_idx, move_);
            self.game.print();

            if self.game.is_solved() {
                return true;
            }

            let next_piece_idx = self.strategy[next_move];
            let next_moves = self.game.possible(next_piece_idx);

            if next_moves.len() == 0 {
                self.game.remove(curr_piece_idx);
            } else {
                self.possible.push_back(next_moves);
            }

            self.moves += 1
        }

        return false;
    }
}