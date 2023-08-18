use super::{Kaleidoscope, Solver};
use std::time::{self, Instant};

/*
Backtracking solver returns the first solution it finds
it searches the tree of possible moves in a depth-first manner
it can get stuck and explore millions of 'bad' possibliities before finding a solution
 */
pub struct BacktrackingSolver<T: Kaleidoscope> {
    pub game: T,
    pub moves: u128,
    pub time: Option<time::Duration>,
    pub available: [bool; 18],    // whether or not piece is available
    pub possible: [usize; 18],    // slice of index of last move we tried for a given piece
    pub cur_move: usize,          // index into strategy (piece ordering)
    pub next_moves: Vec<T::Move>, // vector of moves for strategy[cur_move]
}

impl<T> BacktrackingSolver<T> {
    pub fn new(game: T) -> Self {
        Self {
            game,
            moves: 0,
            time: None,
            available: [true; 18],
            possible: [0; 18],
            cur_move: 0,
            next_moves: Vec::new(),
        }
    }
}

impl<T: Kaleidoscope> Solver<T> for BacktrackingSolver<T> {
    type Strategy = usize;
    type Solution = usize;

    fn solve(&self, strategy: Self::Strategy) -> bool {
        let now = Instant::now();

        // TODO
        //     // intialization
        //     let first_piece_idx = self.piece_order[0];
        //     self.possible.push_back(game.possible(first_piece_idx));

        //     while !self.possible.is_empty() {
        //         let curr_move = self.possible.len() - 1;

        //         // if there are no available moves for the current piece
        //         if self.possible[curr_move].is_empty() {
        //             // undo the last move, if possible
        //             if curr_move != 0 {
        //                 game.remove(self.piece_order[curr_move - 1]);
        //             }

        //             // remove the empty list of moves from the list of possible moves
        //             self.possible.pop_back();
        //             continue;
        //         }

        //         // place the first possible current move on the board
        //         let move_ = self.possible[curr_move].pop_front().unwrap();
        //         game.set(move_);
        //         *moves += 1;

        //         // exit condition
        //         if curr_move == self.piece_order.len() - 1 && game.is_solved() {
        //             return true;
        //         }

        //         // get the next piece's possible moves
        //         let next_move = curr_move + 1;
        //         let next_piece_idx = self.piece_order[next_move];
        //         let next_moves = game.possible(next_piece_idx);
        //         self.possible.push_back(next_moves);
        //     }
        //     false
        // }

        self.time = Some(now.elapsed());
        self.game.solved()
    }

    fn print(&self) {
        if self.game.solved() {
            println!(
                "\nSolved. {} moves in {}s",
                self.moves,
                self.time.unwrap().as_secs_f64()
            );
            self.game.print();
        } else {
            println!(
                "\nNo solution found. {} moves in {}s",
                self.moves,
                self.time.unwrap().as_secs_f64()
            );
        }
    }
}
