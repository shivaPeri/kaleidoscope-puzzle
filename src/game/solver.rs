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
    type Solution = usize;

    fn solve(&mut self, strategy: Self::Strategy) -> bool {
        let now = Instant::now();

        loop {
            if self.game.solved() {
                break;
            }

            self.game.print();

            // intialization
            let piece_idx = strategy[self.cur_move];
            let move_idx = self.possible[piece_idx];
            let possible_moves = self.game.possible(piece_idx);

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
                self.possible[piece_idx] += 1;
                let mv = &possible_moves[move_idx];
                self.game.play(mv);
                self.solution.push(self.game.clone_move(mv));
                self.num_moves += 1;
                self.cur_move += 1;
            }
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
