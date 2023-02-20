pub mod solver;
pub mod generator;

use std::{fs, str::FromStr};
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use termion::{color::{self}};
use std::collections::VecDeque;

// for Reference

// EMPTY = 0,
// BLACK = 1,
// RED = 2,
// YELLOW = 3,
// BLUE = 4

#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

// ordering of 18 pieces to place
pub type Strategy = [usize; 18];
pub type PieceConfig = Vec<u8>;

#[derive(Default, Debug, Clone)]
pub struct Piece {
    pub idx: usize,
    pub configs: Vec<PieceConfig>,
}

impl Piece {

    pub fn new(idx: usize, configs: Vec<PieceConfig>) -> Self {
        Piece {
            idx,
            configs,
        }
    }

    pub fn get_piece_color(&self, config_idx: usize, y: usize, x: usize) -> u8 {
        let piece = &self.configs[config_idx];
        let dims = &piece[..2];
        let piece_colors: &[u8] = &piece[2..];
        let idx: usize = y * dims[1] as usize + x;
        return piece_colors[idx];
    }

    pub fn print(&self, config_idx: usize, row: usize, col: usize) {

        println!("----------------");
        let piece = &self.configs[config_idx];
        let width = *&piece[0] as usize;
        let height = *&piece[1] as usize;

        for i in 0..8 {
            for j in 0..8 {

                if i >= row && i < row + width && j >= col && j < col + height {
                    let color = self.get_piece_color(config_idx, i-row, j-col);
                    match color {
                        1 => print!("{}■ ", color::Fg(color::White)),
                        2 => print!("{}■ ", color::Fg(color::Red)),
                        3 => print!("{}■ ", color::Fg(color::Yellow)),
                        4 => print!("{}■ ", color::Fg(color::Blue)),
                        _ => print!("{}□ ", color::Fg(color::Reset)),
                    };
                } else {
                    print!("{}□ ", color::Fg(color::White));
                }
            }
            println!("");
        }
    }
}


pub type Move = [usize; 3];     // row, col, config_idx

#[derive(Clone)]
pub struct Kaleidoscope {
    pub refboard: [[u8; 8]; 8],
	pub board: [[Option<u8>; 8]; 8],   // piece_idx
    pub pieces: Vec<Piece>,
}

impl Kaleidoscope {

    // Create a new game board from a string.
    pub fn new(path: &Path, name: &str) -> Self {

        let file = fs::read_to_string(path).expect("Unable to read file");
        let data: Data = serde_json::from_str(&file).expect("Unable to parse json");
        let board_str = String::from_str(data.0.get(name).unwrap()).unwrap();

        let mut ref_board = [[0; 8]; 8];
        for (i, c) in board_str.chars().enumerate() {
            ref_board[i / 8][i % 8] = c.to_digit(10).unwrap() as u8;
        }

        Self { 
            board: [[None; 8]; 8],
            refboard: ref_board,
            pieces: Self::load_pieces(),
        }
    }

    fn load_piece_configs() -> Vec<Vec<PieceConfig>> {

        let mono_1: Vec<PieceConfig> = vec![
            vec![1,1, 2],
            vec![1,1, 1],
        ];
    
        let mono_2: Vec<PieceConfig> = vec![
            vec![1,1, 4],
            vec![1,1, 1],
        ];
    
        let domo_1: Vec<PieceConfig> = vec![
            vec![1,2, 2,1],
            vec![1,2, 1,2],
            vec![1,2, 4,1],
            vec![1,2, 1,4],
            vec![2,1, 2,1],
            vec![2,1, 1,2],
            vec![2,1, 4,1],
            vec![2,1, 1,4],
        ];
    
        let trom_1: Vec<PieceConfig> = vec![
            vec![1,3, 2,1,2],
            vec![1,3, 3,1,4],
            vec![1,3, 4,1,3],
            vec![3,1, 2,1,2],
            vec![3,1, 3,1,4],
            vec![3,1, 4,1,3],
        ];
    
        let trom_2: Vec<PieceConfig> = vec![
            vec![1,3, 1,2,1],
            vec![1,3, 1,3,1],
            vec![3,1, 1,2,1],
            vec![3,1, 1,3,1],
        ];
    
        let trom_3: Vec<PieceConfig> = vec![
            vec![2,2, 0,1,1,2],
            vec![2,2, 1,0,2,1],
            vec![2,2, 1,2,0,1],
            vec![2,2, 2,1,1,0],
            vec![2,2, 0,1,1,3],
            vec![2,2, 1,0,3,1],
            vec![2,2, 1,3,0,1],
            vec![2,2, 3,1,1,0],
        ];
    
        let trom_4: Vec<PieceConfig> = vec![
            vec![2,2, 0,2,2,1],
            vec![2,2, 2,0,1,2],
            vec![2,2, 2,1,0,2],
            vec![2,2, 1,2,2,0],
            vec![2,2, 0,4,3,1],
            vec![2,2, 3,0,1,4],
            vec![2,2, 4,1,0,3],
            vec![2,2, 1,3,4,0],
        ];
    
        let tetr_1: Vec<PieceConfig> = vec![
            vec![1,4, 2,1,2,1],
            vec![1,4, 1,2,1,2],
            vec![1,4, 4,1,3,1],
            vec![1,4, 1,3,1,4],
            vec![4,1, 2,1,2,1],
            vec![4,1, 1,2,1,2],
            vec![4,1, 4,1,3,1],
            vec![4,1, 1,3,1,4],
        ];
    
        let tetr_2: Vec<PieceConfig> = vec![
            vec![2,2, 2,1,1,2],
            vec![2,2, 1,2,2,1],
            vec![2,2, 4,1,1,3],
            vec![2,2, 3,1,1,4],
            vec![2,2, 1,3,4,1],
            vec![2,2, 1,4,3,1],
        ];
    
        let tetr_3: Vec<PieceConfig> = vec![
            vec![2,3, 0,0,2,1,2,1],
            vec![3,2, 2,1,0,2,0,1],
            vec![2,3, 1,2,1,2,0,0],
            vec![3,2, 1,0,2,0,1,2],
            vec![2,3, 1,0,0,3,1,4],
            vec![3,2, 3,1,1,0,4,0],
            vec![2,3, 4,1,3,0,0,1],
            vec![3,2, 0,4,0,1,1,3],
        ];
    
        let tetr_4: Vec<PieceConfig> = vec![
            vec![2,3, 2,0,0,1,2,1],
            vec![3,2, 1,2,2,0,1,0],
            vec![2,3, 1,2,1,0,0,2],
            vec![3,2, 0,1,0,2,2,1],
            vec![2,3, 0,0,3,1,4,1],
            vec![3,2, 3,1,0,4,0,1],
            vec![2,3, 1,4,1,3,0,0],
            vec![3,2, 1,0,4,0,1,3],
        ];
    
        let tetr_5: Vec<PieceConfig> = vec![
            vec![2,3, 0,0,1,2,1,2],
            vec![3,2, 1,2,0,1,0,2],
            vec![2,3, 2,1,2,1,0,0],
            vec![3,2, 2,0,1,0,2,1],
            vec![2,3, 1,0,0,4,1,3],
            vec![3,2, 4,1,1,0,3,0],
            vec![2,3, 3,1,4,0,0,1],
            vec![3,2, 0,3,0,1,1,4],
        ];
    
    
        let tetr_6: Vec<PieceConfig> = vec![
            vec![2,3, 1,0,0,2,1,2],
            vec![3,2, 2,1,1,0,2,0],
            vec![2,3, 2,1,2,0,0,1],
            vec![3,2, 0,2,0,1,1,2],
            vec![2,3, 0,0,4,1,3,1],
            vec![3,2, 4,1,0,3,0,1],
            vec![2,3, 1,3,1,4,0,0],
            vec![3,2, 1,0,3,0,1,4],
        ];
    
        let tetr_7: Vec<PieceConfig> = vec![
            vec![2,3, 0,1,0,1,2,1],
            vec![3,2, 1,0,2,1,1,0],
            vec![2,3, 1,2,1,0,1,0],
            vec![3,2, 0,1,1,2,0,1],
            vec![2,3, 0,3,0,4,1,3],
            vec![3,2, 4,0,1,3,3,0],
            vec![2,3, 3,1,4,0,3,0],
            vec![3,2, 0,3,3,1,0,4],
        ];
    
        let tetr_8: Vec<PieceConfig> = vec![
            vec![2,3, 0,2,0,2,1,2],
            vec![3,2, 2,0,1,2,2,0],
            vec![2,3, 2,1,2,0,2,0],
            vec![3,2, 0,2,2,1,0,2],
            vec![2,3, 0,1,0,1,4,1],
            vec![3,2, 1,0,4,1,1,0],
            vec![2,3, 1,4,1,0,1,0],
            vec![3,2, 0,1,1,4,0,1],
        ];
    
        let tetr_9: Vec<PieceConfig> = vec![
            vec![2,3, 0,1,2,1,2,0],
            vec![3,2, 2,0,1,2,0,1],
            vec![2,3, 0,2,1,2,1,0],
            vec![3,2, 1,0,2,1,0,2],
            vec![2,3, 1,4,0,0,1,3],
            vec![3,2, 0,1,1,4,3,0],
            vec![2,3, 3,1,0,0,4,1],
            vec![3,2, 0,3,4,1,1,0],
        ];
    
        let tetr_10: Vec<PieceConfig> = vec![
            vec![2,3, 1,2,0,0,1,2],
            vec![3,2, 0,1,1,2,2,0],
            vec![2,3, 2,1,0,0,2,1],
            vec![3,2, 0,2,2,1,1,0],
            vec![2,3, 0,3,1,4,1,0],
            vec![3,2, 4,0,1,3,0,1],
            vec![2,3, 0,1,4,1,3,0],
            vec![3,2, 1,0,3,1,0,4],
        ];
    
        let oct_1: Vec<PieceConfig> = vec![
            vec![1,8, 1,2,1,2,1,2,1,2],
            vec![1,8, 2,1,2,1,2,1,2,1],
            vec![8,1, 1,2,1,2,1,2,1,2],
            vec![8,1, 2,1,2,1,2,1,2,1],
            vec![1,8, 1,3,1,4,1,3,1,4],
            vec![1,8, 4,1,3,1,4,1,3,1],
            vec![8,1, 1,3,1,4,1,3,1,4],
            vec![8,1, 4,1,3,1,4,1,3,1],
        ];
    
        let pieces: Vec<Vec<PieceConfig>> = vec![mono_1, mono_2, domo_1, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
        return pieces;
    }
    
    pub fn load_pieces() -> Vec<Piece> {
        let piece_configs = Self::load_piece_configs();
    
        let mut pieces: Vec<Piece> = Vec::new();
        for (i, configs) in piece_configs.iter().enumerate() {
            pieces.push(Piece::new(i, configs.clone()));
        }
    
        return pieces;
    }

    // place piece on board (ie. make a move)
	pub fn set(&mut self, piece_idx: usize, m: Move) {

        let pos = [m[1], m[0]];
        let config_idx = m[2];

        let piece = &self.pieces[piece_idx].configs[config_idx];
        let dim_1 = piece[0] as usize;
        let dim_2 = piece[1] as usize;

        for x in 0..dim_1 {
            for y in 0..dim_2 {
                let color = self.pieces[piece_idx].get_piece_color(config_idx, x, y);
                if color != 0 {     // only place non-empty cells
                    self.board[pos[0] + x][pos[1] + y] = Some(piece_idx as u8);
                }
            }
        }
	}

    // remove piece on board (ie. undo a move)
	pub fn remove(&mut self, piece_idx: usize) {

        for x in 0..8 {
            for y in 0..8 {
                match self.board[x][y] {
                    Some(piece) => {
                        if piece == piece_idx as u8 {
                            self.board[x][y] = None;
                        }
                    },
                    None => continue,
                }
            }
        }
	}

	pub fn print(&self) {
		println!("----------------");
		for x in 0..8 {
		    for y in 0..8 {
                let val = match self.board[x][y] {
                    Some(piece) => piece as i8,
                    None => -1,
                };
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

    pub fn print_ref(&self) {
		println!("----------------");
        for x in 0..8 {
		    for y in 0..8 {
                match self.refboard[x][y] {
                    1 => print!("{}■ ", color::Fg(color::White)),
                    2 => print!("{}■ ", color::Fg(color::Red)),
                    3 => print!("{}■ ", color::Fg(color::Yellow)),
                    4 => print!("{}■ ", color::Fg(color::Blue)),
                    _ => print!("{}■ ", color::Fg(color::Reset)),
                };
            }
            println!("");
		}
	}

	pub fn is_solved(&self) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if self.board[y][x] == None { return false; }
            }
        }
		return true;
	}

    // Given a piece, returns a vector of possible placements and configurations.
	pub fn possible(&self, piece_idx: usize) -> VecDeque<Move> {

        let piece = &self.pieces[piece_idx];

		let mut res = VecDeque::new();

		for pos in 0..64 {     // for each position
            
            let pos_x = pos % 8;
            let pos_y = pos / 8;

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
                            if self.board[board_x][board_y] != None {        // non-empty board color
                                continue 'next_config;
                            }
                            if color != self.refboard[board_x][board_y] {   // mismatched board color
                                continue 'next_config;
                            }
                        }
                    }
                }
                res.push_back([pos_y, pos_x, config_idx]);
            }
        }
        return res;
    }
}
