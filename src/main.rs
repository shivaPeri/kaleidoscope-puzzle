use game::{Piece, PieceConfig, Strategy};
use termion::{color::{self, Color}, style};

use std::io;
use std::path::Path;

mod game;

pub type Move = [usize; 3];     // row, col, config_idx

#[derive(Clone)]
pub struct Kaleidoscope {
    pub refboard: [[u8; 8]; 8],
	pub board: [[[u8; 5]; 8]; 8],       // color, piece_idx, config_idx, row, col
    pub pieces: Vec<Piece>,
    pub used: [Option<[usize; 2]>; 18],     // pos_y, pos_x, config_idx
}

impl Kaleidoscope {
	// type Pos = [usize; 2];          // Position of a cell
	// type Val = [u8; 3];             // color, piece_idx, configuration_idx

    // place piece on board (ie. make a move)
	fn set(&mut self, piece_idx: usize, m: Move) {

        let pos = [m[1], m[0]];
        let piece = &self.pieces[piece_idx].configs[m[2]];
        let dim_1 = piece[0] as usize;
        let dim_2 = piece[1] as usize;

        for y in 0..dim_1 {
            for x in 0..dim_2 {
                let color = piece[y * dim_2 + x];
                if color != 0 {     // only place non-empty cells
                    self.board[pos[1] + y][pos[0] + x] = [color, piece_idx as u8, m[2] as u8, m[0] as u8, m[1] as u8];
                }
            }
        }
        self.used[piece_idx] = Some(pos);
	}

    // get current piece placed on a given position of the board
	fn get(&self, pos: [usize; 2]) -> [u8; 5] {
		self.board[pos[1]][pos[0]]
	}

	fn print(&self) {
		println!("----------------");
		for x in 0..8 {
		    for y in 0..8 {
                let val = self.board[x][y][1];  // piece_idx
                match self.refboard[x][y] {
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

    fn print_ref(&self) {
		println!("----------------");
        for x in 0..8 {
		    for y in 0..8 {
                match self.refboard[x][y] {
                    1 => print!("{}□ ", color::Fg(color::White)),
                    2 => print!("{}■ ", color::Fg(color::Red)),
                    3 => print!("{}■ ", color::Fg(color::Yellow)),
                    4 => print!("{}■ ", color::Fg(color::Blue)),
                    _ => print!("{}■ ", color::Fg(color::Reset)),
                };
            }
            println!("");
		}
	}

	fn is_solved(&self) -> bool {

        // check if all cells are filled
        for y in 0..8 {
            for x in 0..8 {
                if self.board[y][x][0] == 0 { return false; }
            }
        }

        // check if all pieces are used
        for i in 0..18 {
            if self.used[i] == None { return false; }
        }
		return true;
	}

    // Create a new game board from a string.
    pub fn from_str(board_str: &str) -> Self {

        let mut ref_board = [[0; 8]; 8];
        for (i, c) in board_str.chars().enumerate() {
            ref_board[i / 8][i % 8] = c.to_digit(10).unwrap() as u8;
        }

        return Self { 
            board: [[[0; 5]; 8]; 8],
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

    // Given a piece, returns a vector of possible placements and configurations.
	pub fn possible(&self, piece_idx: usize) -> Vec<Move> {

        let piece = &self.pieces[piece_idx];

		let mut res = vec![];
        if self.used[piece_idx] != None {
            return res;
        }

		'next_pos: for pos in 0..64 {     // for each position
            
            let pos_x = pos % 8;
            let pos_y = pos / 8;
            if self.board[pos_x][pos_y][0] != 0 { continue 'next_pos; }

            'next_config: for config_idx in 0..piece.configs.len() {     // for each config
                let config = &piece.configs[config_idx];
                let dim_1 = config[0] as usize;
                let dim_2 = config[1] as usize;

                for x in 0..dim_1 {
                    for y in 0..dim_2 {

                        let board_x = pos_x + x;
                        let board_y = pos_y + y;
                        // check if piece fits in the board
                        if board_x >= 8 || board_y >= 8 {
                            continue 'next_config;
                        }

                        // check if piece color matches the board
                        let color = piece.get_piece_color(config_idx, x, y);
                        if color != 0 {                                     // non-empty piece color
                            if self.board[board_x][board_y][0] != 0 {       // non-empty board color
                                continue 'next_config;
                            }
                            if color != self.refboard[board_x][board_y] {   // mismatched board color
                                continue 'next_config;
                            }
                        }
                    }
                }
                res.push([pos_y, pos_x, config_idx]);
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
    // let game_str = game::load_game_str(Path::new("boards/boards.json"), "classic");
	let x = Kaleidoscope::from_str(&game_str);
	x.print_ref();

    let test_idx = 17;
    let test = x.possible(test_idx);

    for thing in test.iter(){
        println!("{} {}", thing[0], thing[1]);
        x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    }
}