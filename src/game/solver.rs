use super::{Kaleidoscope, Move, PlayOrder};
use std::collections::VecDeque;
use std::time::{self, Instant};

pub trait Strategy {
    fn solve(&mut self, game: &mut Kaleidoscope, moves: &mut u128) -> bool;
}

pub struct KaleidoscopeSolver {
    pub game: Kaleidoscope,
    pub moves: u128,
    pub time: Option<time::Duration>,
}

impl KaleidoscopeSolver {
    pub fn new(game: Kaleidoscope) -> Self {
        Self {
            game,
            moves: 0,
            time: None
        }
    }

    pub fn print(&self) {
        if self.game.is_solved() {
            println!("\nSolved. {} moves in {}s", self.moves, self.time.unwrap().as_secs_f64());
            self.game.print();
        } else {
            println!("\nNo solution found. {} moves in {}s", self.moves, self.time.unwrap().as_secs_f64());
        }
    }

    pub fn solve<T: Strategy>(&mut self, strategy: &mut T) -> bool {
        let now = Instant::now();
        let solved = strategy.solve(&mut self.game, &mut self.moves);
        self.time = Some(now.elapsed());
        solved
    }
}

/*
Backtracking solver returns the first solution it finds
it searches the tree of possible moves in a depth-first manner
it can get stuck and explore millions of 'bad' possibliities before finding a solution
 */
pub struct BacktrackingSolver {
    pub piece_order: PlayOrder,
    possible: VecDeque<VecDeque<Move>>,   // possible moves for each piece
}

impl BacktrackingSolver {
    pub fn new(piece_order: PlayOrder) -> Self {
        Self {
            piece_order,
            possible: VecDeque::new(),
        }
    }
}

impl Strategy for BacktrackingSolver {

    fn solve(&mut self, game: &mut Kaleidoscope, moves: &mut u128) -> bool {

        // intialization
        let first_piece_idx = self.piece_order[0];
        self.possible.push_back(game.possible(first_piece_idx));

        while !self.possible.is_empty() {

            let curr_move = self.possible.len() - 1;
            
            // if there are no available moves for the current piece
            if self.possible[curr_move].is_empty() {

                // undo the last move, if possible
                if curr_move != 0 {
                    game.remove(self.piece_order[curr_move - 1]);
                }

                // remove the empty list of moves from the list of possible moves
                self.possible.pop_back();
                continue;
            }

            // place the first possible current move on the board
            let move_ = self.possible[curr_move].pop_front().unwrap();
            game.set(move_);
            *moves += 1;

            // exit condition
            if curr_move == self.piece_order.len() - 1 && game.is_solved() {
                return true;
            }

            // get the next piece's possible moves
            let next_move = curr_move + 1;
            let next_piece_idx = self.piece_order[next_move];
            let next_moves = game.possible(next_piece_idx);
            self.possible.push_back(next_moves);
        }
        false
    }
}

/*
Backtracking solver2 returns the first solution it finds
it searches the tree of possible moves in a depth-first manner

The difference between this and BacktrackingSolver is that this
implementation goes cell by cell rather than piece by piece
 */
pub struct BacktrackingSolver2 {
    pub piece_order: PlayOrder,
    available: Vec<bool>,
    last_piece_idx: Option<usize>,
    possible: VecDeque<VecDeque<Move>>,   // possible moves for each piece
}

impl BacktrackingSolver2 {
    pub fn new(piece_order: PlayOrder) -> Self {
        Self {
            piece_order,
            available: vec![true; 18],
            last_piece_idx: None,
            possible: VecDeque::new(),
        }
    }
}

impl Strategy for BacktrackingSolver2 {

    fn solve(&mut self, game: &mut Kaleidoscope, moves: &mut u128) -> bool {

        // intialization
        self.possible.push_back(game.possible_at_cell(0, &self.piece_order, &self.available));
        // println!("{}", self.possible.len());

        while (!self.possible.is_empty()) {
            let curr_move = self.possible.len() - 1;
            print!("{:?}", self.possible);

            for pos in 0..64 {
                if game.board[pos].is_none() {  // find first empty cell

                    // if there are no available moves for the current piece
                    if self.possible[curr_move].is_empty() {

                        // undo the last move, if possible
                        if self.last_piece_idx.is_some() {
                            game.remove(self.last_piece_idx.unwrap());
                            self.available[self.last_piece_idx.unwrap()] = true;
                        }

                        // remove the empty list of moves from the list of possible moves
                        self.possible.pop_back();
                        continue;
                    }

                    // place the first possible current move on the board, mark it as used
                    let move_ = self.possible[curr_move].pop_front().unwrap();
                    game.set(move_);
                    self.available[move_.piece_idx] = false;
                    self.last_piece_idx = Some(move_.piece_idx);
                    *moves += 1;

                    // get the next piece's possible moves
                    let next_moves = game.possible_at_cell(pos, &self.piece_order, &self.available);
                    self.possible.push_back(next_moves);
                    break;
                }
            }
            // exit condition
            if curr_move == self.piece_order.len() - 1 && game.is_solved() {
                return true;
            }

        }
        false
    }
}

/*
Beam search solver returns the first solution it finds
it searches the tree of possible moves in a breadth-first manner
 */
#[derive(Default, Debug)]
pub struct BeamSearchSolver {
    beam_width: usize,                    // number of possible moves to consider at each level
    possible: VecDeque<VecDeque<Move>>,   // possible moves for each piece
}

impl BeamSearchSolver {
    pub fn new(beam_width: usize) -> Self {
        Self {
            beam_width,
            possible: VecDeque::new(),
        }
    }
}

// TODO: implement BeamSearchSolver
impl Strategy for BeamSearchSolver {
    fn solve(&mut self, game: &mut Kaleidoscope, moves: &mut u128) -> bool {
        unimplemented!()
    }
}