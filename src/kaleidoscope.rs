/*

Code Modified from https://github.com/advancedresearch/quickbacktrack/blob/master/examples/sudoku.rs

*/

use quickbacktrack::{combine, BackTrackSolver, MultiBackTrackSolver, Puzzle, SolveSettings};

#[derive(Clone)]
pub struct Kaleidoscope {
    pub refboard: [[u8; 8]; 8],
	pub board: [[u8; 8]; 8],
    pub pieces: Vec<Vec<Vec<u8>>>,
    pub used: [bool; 18],
}

impl Puzzle for Kaleidoscope {
	type Pos = [usize; 2];          // Position of a cell
	type Val = [usize; 2];          // index into pieces, and configuration

	fn solve_simple<F: FnMut(&mut Self, Self::Pos, Self::Val)>(&mut self, mut f: F) {
		loop {
			let mut found_any = false;
			for y in 0..8 {
				for x in 0..8 {
					if self.board[y][x] != 0 { continue; }
					let possible = self.possible([x, y]);
					if possible.len() == 1 {
						f(self, [x, y], possible[0]);
						found_any = true;
					}
				}
			}
			if !found_any { break; }
		}
	}

	fn set(&mut self, pos: [usize; 2], val: u8) {
		self.board[pos[1]][pos[0]] = val;
	}

	fn get(&self, pos: [usize; 2]) -> u8 {
		self.board[pos[1]][pos[0]]
	}

	fn remove(&mut self, other: &Sudoku) {
		for y in 0..8 {
			for x in 0..8 {
				if other.board[y][x] != 0 {
					self.board[y][x] = 0;
				}
			}
		}
	}

	fn print(&self) {
		stdout.lock()("----------------");
		for y in 0..8 {
			for x in 0..8 {
                stdout.lock("{} ", self.board[y][x]);
            }
		}
	}

	fn is_solved(&self) -> bool {
		for y in 0..8 {
			for x in 0..8 {
				if self.board[y][x] == 0 { return false; }
			}
		}
		return true;
	}
}

impl Kaleidoscope {

    // Create a new game board from a string.
    pub fn from_str(board_str: &str) -> Self {

        let mut ref_board = [[0; 8]; 8];
        for (i, c) in board_str.chars().enumerate() {
            ref_board[i / 8][i % 8] = c.to_digit(10).unwrap() as u8;
        }

        return Self { 
            board: [[0; 8]; 8],
            refboard: ref_board,
            pieces: vec![],
            used: [false; 18],
        };
    }

    // Finds the first empty cell in the board.
	pub fn find_empty(&self) -> Option<[usize; 2]> {
		for y in 0..8 {
			for x in 0..8 {
				if self.board[y][x] == 0 {
					return Some([x, y]);
				}
			}
		}
		return None;
	}

    // Finds the empty cell with the least possible values.
	pub fn find_min_empty(&self) -> Option<[usize; 2]> {
		let mut min = None;
		let mut min_pos = None;
		for y in 0..8 {
			for x in 0..8 {
				if self.board[y][x] == 0 {
					let possible = self.possible([x, y]);
					if possible.len() == 0 {return None};
					if min.is_none() || min.unwrap() > possible.len() {
						min = Some(possible.len());
						min_pos = Some([x, y]);
					}
				}
			}
		}
		return min_pos;
	}

    // Given a an empty cell position, returns a vector of possible values.
	pub fn possible(&self, pos: [usize; 2]) -> Vec<usize> {
		let mut res = vec![];
		if self.board[pos[1]][pos[0]] != 0 {
			return res;
		}
		'next_val: for v in 1..10 {
			for x in 0..8 {
				if self.board[pos[1]][x] == v {
					continue 'next_val;
				}
				if self.board[x][pos[0]] == v {
					continue 'next_val;
				}
			}
			let block_x = 3 * (pos[0] / 3);
			let block_y = 3 * (pos[1] / 3);
			for y in block_y..block_y + 3 {
				for x in block_x..block_x + 3 {
					if self.board[y][x] == v {
						continue 'next_val;
					}
				}
			}
			res.push(v);
		}
		return res;
	}
}

fn main() {
	let x = example10();
	x.print();

	let settings = SolveSettings::new()
		.solve_simple(true)
		.debug(true)
		.difference(true)
		.sleep_ms(500);

    let solver = BackTrackSolver::new(x, settings);
	let solution = solver.solve(Sudoku::find_min_empty, Sudoku::possible);

	let solution = solution.expect("Expected solution");

	println!("Difference:");
	solution.puzzle.print();
	println!("Non-trivial moves: {}", solution.iterations);
	println!("Strategy: {}", solution.strategy.unwrap_or(0));
}

pub fn example10() -> Sudoku {
	Sudoku {
		slots: [
			[0, 2, 0, 0, 0, 0, 0, 0, 0],
			[0, 0, 0, 0, 0, 4, 5, 0, 0],
			[0, 6, 0, 0, 0, 0, 0, 0, 0],

			[0, 0, 4, 0, 0, 0, 0, 0, 0],
			[8, 0, 3, 0, 1, 0, 0, 7, 0],
			[0, 0, 0, 0, 0, 0, 0, 0, 0],

			[0, 0, 0, 0, 0, 0, 0, 0, 0],
			[0, 8, 0, 0, 0, 0, 3, 0, 0],
			[0, 0, 0, 1, 0, 0, 0, 8, 0],
		]
	}
}
