use super::{Kaleidoscope, Solver};
use std::time::{self, Instant};

/*
Backtracking solver returns the first solution it finds
it searches the tree of possible moves in a depth-first manner
it can get stuck and explore millions of 'bad' possibliities before finding a solution
 */
#[derive(Debug, Clone)]
pub struct BacktrackingSolver<T: Kaleidoscope> {
    pub game: T,
    pub num_moves: u128,
    pub time: Option<time::Duration>,
    pub possible: [usize; 18], // slice of index of last move we tried for a given piece
    pub cur_move: usize,       // index into strategy (piece ordering)
    pub solution: Vec<T::Move>, // list of moves in solution (for undo operation)
}

impl<T: Kaleidoscope> BacktrackingSolver<T> {
    pub fn new(game: T) -> Self {
        Self {
            game,
            num_moves: 0,
            time: None,
            possible: [0; 18],
            cur_move: 0,
            solution: Vec::new(),
        }
    }
}

impl<T: Kaleidoscope> Solver<T> for BacktrackingSolver<T> {
    type Strategy = [usize; 18];

    fn solve(&mut self, strategy: Self::Strategy) -> bool {
        let now = Instant::now();

        loop {
            if self.game.solved() {
                break;
            }

            println!("{:?}", self.possible);
            self.game.print();

            // intialization
            let piece_idx = strategy[self.cur_move];
            let move_idx = self.possible[self.cur_move];
            let possible_moves = self.game.possible(piece_idx, false);

            // if there are no moves available
            if possible_moves.len() == 0 || move_idx >= possible_moves.len() {
                match self.solution.len() {
                    // if there are no remaining moves
                    0 => break,
                    _ => {
                        self.cur_move -= 1;
                        let mv = self.solution.pop().unwrap();
                        self.game.undo(mv);
                    }
                }
            } else {
                // increment index of possible moves to avoid re-playing in later iteration
                self.possible[self.cur_move] += 1;
                let mv = &possible_moves[move_idx];
                self.game.play(mv);
                self.solution.push(self.game.clone_move(mv));
                self.cur_move += 1;
            }
            self.num_moves += 1;
        }

        self.time = Some(now.elapsed());
        self.game.solved()
    }

    fn print(&self) {
        if self.game.solved() {
            println!(
                "\nSolved. {} moves in {}s",
                self.num_moves,
                self.time.unwrap().as_secs_f64()
            );
            self.game.print();
        } else {
            println!(
                "\nNo solution found. {} moves in {}s",
                self.num_moves,
                self.time.unwrap().as_secs_f64()
            );
        }
    }
}

/*
More memory-intensive solver than BacktrackingSolver, but avoids recomputing possible moves at multiple steps
 */
#[derive(Debug, Clone)]
pub struct BacktrackingSolver2<T: Kaleidoscope> {
    pub game: T,
    pub num_moves: u128,
    pub time: Option<time::Duration>,
    pub possible: Vec<Vec<T::Move>>, // possible moves, cached
    pub cur_move: usize,             // index into strategy (piece ordering)
    pub solution: Vec<T::Move>,      // list of moves in solution (for undo operation)
}

impl<T: Kaleidoscope> BacktrackingSolver2<T> {
    pub fn new(game: T) -> Self {
        Self {
            game,
            num_moves: 0,
            time: None,
            possible: Vec::new(),
            cur_move: 0,
            solution: Vec::new(),
        }
    }
}

impl<T: Kaleidoscope> Solver<T> for BacktrackingSolver2<T> {
    type Strategy = [usize; 18];

    fn solve(&mut self, strategy: Self::Strategy) -> bool {
        // intialization
        let now = Instant::now();
        let first_piece_idx = strategy[0];
        self.possible
            .push(self.game.possible(first_piece_idx, true));

        while !self.possible.is_empty() {
            let curr_move = self.possible.len() - 1;

            // if there are no available moves for the current piece
            if self.possible[curr_move].is_empty() {
                // undo the last move, if possible
                if curr_move != 0 {
                    let mv = self.solution.pop().unwrap();
                    self.game.undo(mv);
                }

                // remove the empty list of moves from the list of possible moves
                self.possible.pop();
                continue;
            }

            // place the first possible current move on the board
            let move_ = self.possible[curr_move].pop().unwrap();
            self.game.play(&move_);
            self.solution.push(move_);
            self.num_moves += 1;

            // exit condition
            if curr_move == strategy.len() - 1 && self.game.solved() {
                self.time = Some(now.elapsed());
                return true;
            }

            // get the next piece's possible moves
            let next_move = curr_move + 1;
            let next_piece_idx = strategy[next_move];
            let next_moves = self.game.possible(next_piece_idx, true);
            self.possible.push(next_moves);
        }
        self.time = Some(now.elapsed());
        self.game.solved()
    }

    fn print(&self) {
        if self.game.solved() {
            println!(
                "\nSolved. {} moves in {}s",
                self.num_moves,
                self.time.unwrap().as_secs_f64()
            );
            self.game.print();
            // for mv in self.solution.iter() {
            //     self.game.print_move(&mv);
            // }
        } else {
            println!(
                "\nNo solution found. {} moves in {}s",
                self.num_moves,
                self.time.unwrap().as_secs_f64()
            );
        }
    }
}
