/*

Code Modified from https://github.com/advancedresearch/quickbacktrack/blob/master/examples/sudoku.rs

*/

use termion::{color::{self, Color}, style};

use std::io;
use std::path::Path;
use quickbacktrack::{combine, BackTrackSolver, MultiBackTrackSolver, Puzzle, SolveSettings};

mod game;

#[derive(Clone)]
pub struct Kaleidoscope {
    pub refboard: [[u8; 8]; 8],
	pub board: [[[usize; 4]; 8]; 8],
    pub pieces: Vec<Vec<Vec<u8>>>,
    pub used: [Option<[usize; 2]>; 18],
}

impl Puzzle for Kaleidoscope {
	type Pos = [usize; 2];          // Position of a cell
	type Val = [usize; 4];          // piece_idx, configuration_idx, row, col

	fn solve_simple<F: FnMut(&mut Self, Self::Pos, Self::Val)>(&mut self, mut f: F) {
		loop {
			let mut found_any = false;
			for y in 0..8 {
				for x in 0..8 {
					if self.board[y][x][0] != 0 { continue; }
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

    // place piece on board (ie. make a move)
	fn set(&mut self, pos: [usize; 2], val: [usize; 4]) {

        let piece_idx = val[1];
        let config = val[2];
        let piece = &self.pieces[piece_idx][config];
        let dim_1 = piece[0] as usize;
        let dim_2 = piece[1] as usize;

        for y in 0..dim_1 {
            for x in 0..dim_2 {
                let color = piece[y * dim_2 + x];
                if color != 0 {     // only place non-empty cells
                    self.board[pos[1] + y][pos[0] + x] = [val[1], val[2], y, x];
                }
            }
        }
;
        self.used[piece_idx] = Some(pos);
	}

    // get current piece placed on a given position of the board
	fn get(&self, pos: [usize; 2]) -> [usize; 4] {
		self.board[pos[1]][pos[0]]
	}

    // TODO: debug this
	fn remove(&mut self, other: &Kaleidoscope) {
		for y in 0..8 {
			for x in 0..8 {
				if other.board[y][x][0] != 0 {
					self.board[y][x][0] = 0;
				}
			}
		}
	}

	fn print(&self) {
		println!("----------------");
		for y in 0..8 {
			for x in 0..8 {
                let val = self.board[y][x][0];
                match self.refboard[y][x] {
                    1 => print!("{}{} ", color::Fg(color::White), val),
                    2 => print!("{}{} ", color::Fg(color::Red), val),
                    3 => print!("{}{} ", color::Fg(color::Yellow), val),
                    4 => print!("{}{} ", color::Fg(color::Blue), val),
                    _ => print!("{}{} ", color::Fg(color::Reset), val),
                };
            }
            println!("");
		}
	}

	fn is_solved(&self) -> bool {
        for i in 0..18 {
            if self.used[i] == None { return false; }
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
            board: [[[0; 4]; 8]; 8],
            refboard: ref_board,
            pieces: game::load_pieces(),
            used: [None; 18],
        };
    }

    // Finds the first empty cell in the board.
	pub fn find_empty(&self) -> Option<[usize; 2]> {
		for y in 0..8 {
			for x in 0..8 {
				if self.board[y][x][0] == 0 {
					return Some([x, y]);
				}
			}
		}
		return None;
	}

    // // Finds the empty cell with the least possible values.
	// pub fn find_min_empty(&self) -> Option<[usize; 2]> {
	// 	let mut min = None;
	// 	let mut min_pos = None;
	// 	for y in 0..8 {
	// 		for x in 0..8 {
	// 			if self.board[y][x] == 0 {
	// 				let possible = self.possible([x, y]);
	// 				if possible.len() == 0 {return None};
	// 				if min.is_none() || min.unwrap() > possible.len() {
	// 					min = Some(possible.len());
	// 					min_pos = Some([x, y]);
	// 				}
	// 			}
	// 		}
	// 	}
	// 	return min_pos;
	// }

    // Given a an empty cell position, returns a vector of possible values.
	pub fn possible(&self, pos: [usize; 2]) -> Vec<[usize; 4]> {

		let mut res = vec![];
		if self.board[pos[0]][pos[1]][0] != 0 {
			return res;
		}
		'next_piece: for idx in 0..18 {     // for each piece
            if self.used[idx] == None {
                continue 'next_piece;
            }
            'next_config: for config in 0..self.pieces[idx].len() {     // for each config
                let piece = &self.pieces[idx][config];
                let dim_1 = piece[0] as usize;
                let dim_2 = piece[1] as usize;
                for y in 0..dim_1 {
                    for x in 0..dim_2 {
                        let color = piece[y * dim_2 + x];
                        if color != 0 {     // only place non-empty cells
                            if self.board[pos[0] + y][pos[1] + x][0] != 0 {
                                continue 'next_config;
                            }
                        }
                    }
                }
                res.push([idx, config, 0, 0]);
            }
		}
		return res;
	}
}

fn main() {

    // testing termion
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Green", color::Fg(color::Green));
    // println!("{}Just plain italic", style::Italic);

    println!("{}Red", color::Fg(color::Red));

    let game_str = game::load_game_str(Path::new("boards/scraped-boards.json"), "australian-emu");
	let x = Kaleidoscope::from_str(&game_str);
	x.print();

	let settings = SolveSettings::new()
		.solve_simple(true)
		.debug(true)
		.difference(true)
		.sleep_ms(500);

    let solver = BackTrackSolver::new(x, settings);
	// let solution = solver.solve(Sudoku::find_min_empty, Sudoku::possible);

	// let solution = solution.expect("Expected solution");

	// println!("Difference:");
	// solution.puzzle.print();
	// println!("Non-trivial moves: {}", solution.iterations);
	// println!("Strategy: {}", solution.strategy.unwrap_or(0))

}