pub mod solver;
pub mod generator;

use std::path::Path;
use std::{fs, str::FromStr};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use termion::color;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceColor {
    Empty,
    Black,
    Red,
    Yellow,
    Blue,
}

impl PieceColor {
    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Self::Black,
            2 => Self::Red,
            3 => Self::Yellow,
            4 => Self::Blue,
            _ => Self::Empty,
        }
    }
}

impl std::fmt::Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Black => write!(f, "{}■ ", color::Fg(color::White)),
            Self::Red => write!(f, "{}■ ", color::Fg(color::Red)),
            Self::Yellow => write!(f, "{}■ ", color::Fg(color::Yellow)),
            Self::Blue => write!(f, "{}■ ", color::Fg(color::Blue)),
            _ => write!(f, "{}□ ", color::Fg(color::Reset)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

// ordering of 18 pieces to place
pub type PlayOrder = [usize; 18];

#[derive(Debug, Clone, PartialEq)]
pub struct PieceConfig {
    pub width: usize,
    pub height: usize,
    pub config: [Option<u8>; 8],
}

impl PieceConfig {
    pub fn new(width: usize, height: usize, config: &[u8]) -> Self {
        let mut conf = [None; 8];
        for (c_entry, &value) in conf.iter_mut().zip(config) {
            *c_entry = Some(value);
        }
        Self {
            width,
            height,
            config: conf,
        }
    }
}

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
        let idx: usize = y * piece.height as usize + x;
        piece.config[idx].unwrap()
    }

    pub fn print(&self, config_idx: usize, row: usize, col: usize) {

        println!("----------------");
        let piece = &self.configs[config_idx];
        let width = usize::from(piece.width);
        let height = usize::from(piece.height);

        for i in 0..8 {
            for j in 0..8 {
                if i >= row && i < row + width && j >= col && j < col + height {
                    let color = self.get_piece_color(config_idx, i-row, j-col);
                    print!("{}", PieceColor::from_u8(color));
                } else {
                    print!("{}", PieceColor::from_u8(0));
                }
            }
            println!();
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
    pub row: usize,
    pub col: usize,
    pub config_idx: usize,
}
#[derive(Clone)]
pub struct Kaleidoscope {
    pub refboard: [u8; 64],
	pub board: [Option<u8>; 64],   // piece_idx
    pub pieces: Vec<Piece>,
}

impl Kaleidoscope {

    // Create a new game board from a string.
    pub fn new(path: &Path, name: &str) -> Self {

        let file = fs::read_to_string(path).expect("Unable to read file");
        let data: Data = serde_json::from_str(&file).expect("Unable to parse json");
        let board_str = String::from_str(data.0.get(name).unwrap()).unwrap();
        let refboard: Vec<u8> = board_str.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
        let refboard: [u8; 64] = refboard.try_into().unwrap();

        Self { 
            board: [None; 64],
            refboard,
            pieces: Self::load_pieces(),
        }
    }

    fn load_piece_configs() -> Vec<Vec<PieceConfig>> {

        let mono_1: Vec<PieceConfig> = vec![
            PieceConfig::new(1,1, &[2]),
            PieceConfig::new(1,1, &[1]),
        ];
    
        let mono_2: Vec<PieceConfig> = vec![
            PieceConfig::new(1,1, &[4]),
            PieceConfig::new(1,1, &[1]),
        ];
    
        let domo_1: Vec<PieceConfig> = vec![
            PieceConfig::new(1,2, &[2,1]),
            PieceConfig::new(1,2, &[1,2]),
            PieceConfig::new(1,2, &[4,1]),
            PieceConfig::new(1,2, &[1,4]),
            PieceConfig::new(2,1, &[2,1]),
            PieceConfig::new(2,1, &[1,2]),
            PieceConfig::new(2,1, &[4,1]),
            PieceConfig::new(2,1, &[1,4]),
        ];
    
        let trom_1: Vec<PieceConfig> = vec![
            PieceConfig::new(1,3, &[2,1,2]),
            PieceConfig::new(1,3, &[3,1,4]),
            PieceConfig::new(1,3, &[4,1,3]),
            PieceConfig::new(3,1, &[2,1,2]),
            PieceConfig::new(3,1, &[3,1,4]),
            PieceConfig::new(3,1, &[4,1,3]),
        ];
    
        let trom_2: Vec<PieceConfig> = vec![
            PieceConfig::new(1,3, &[1,2,1]),
            PieceConfig::new(1,3, &[1,3,1]),
            PieceConfig::new(3,1, &[1,2,1]),
            PieceConfig::new(3,1, &[1,3,1]),
        ];
    
        let trom_3: Vec<PieceConfig> = vec![
            PieceConfig::new(2,2, &[0,1,1,2]),
            PieceConfig::new(2,2, &[1,0,2,1]),
            PieceConfig::new(2,2, &[1,2,0,1]),
            PieceConfig::new(2,2, &[2,1,1,0]),
            PieceConfig::new(2,2, &[0,1,1,3]),
            PieceConfig::new(2,2, &[1,0,3,1]),
            PieceConfig::new(2,2, &[1,3,0,1]),
            PieceConfig::new(2,2, &[3,1,1,0]),
        ];
    
        let trom_4: Vec<PieceConfig> = vec![
            PieceConfig::new(2,2, &[0,2,2,1]),
            PieceConfig::new(2,2, &[2,0,1,2]),
            PieceConfig::new(2,2, &[2,1,0,2]),
            PieceConfig::new(2,2, &[1,2,2,0]),
            PieceConfig::new(2,2, &[0,4,3,1]),
            PieceConfig::new(2,2, &[3,0,1,4]),
            PieceConfig::new(2,2, &[4,1,0,3]),
            PieceConfig::new(2,2, &[1,3,4,0]),
        ];
    
        let tetr_1: Vec<PieceConfig> = vec![
            PieceConfig::new(1,4, &[2,1,2,1]),
            PieceConfig::new(1,4, &[1,2,1,2]),
            PieceConfig::new(1,4, &[4,1,3,1]),
            PieceConfig::new(1,4, &[1,3,1,4]),
            PieceConfig::new(4,1, &[2,1,2,1]),
            PieceConfig::new(4,1, &[1,2,1,2]),
            PieceConfig::new(4,1, &[4,1,3,1]),
            PieceConfig::new(4,1, &[1,3,1,4]),
        ];
    
        let tetr_2: Vec<PieceConfig> = vec![
            PieceConfig::new(2,2, &[2,1,1,2]),
            PieceConfig::new(2,2, &[1,2,2,1]),
            PieceConfig::new(2,2, &[4,1,1,3]),
            PieceConfig::new(2,2, &[3,1,1,4]),
            PieceConfig::new(2,2, &[1,3,4,1]),
            PieceConfig::new(2,2, &[1,4,3,1]),
        ];
    
        let tetr_3: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[0,0,2,1,2,1]),
            PieceConfig::new(3,2, &[2,1,0,2,0,1]),
            PieceConfig::new(2,3, &[1,2,1,2,0,0]),
            PieceConfig::new(3,2, &[1,0,2,0,1,2]),
            PieceConfig::new(2,3, &[1,0,0,3,1,4]),
            PieceConfig::new(3,2, &[3,1,1,0,4,0]),
            PieceConfig::new(2,3, &[4,1,3,0,0,1]),
            PieceConfig::new(3,2, &[0,4,0,1,1,3]),
        ];
    
        let tetr_4: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[2,0,0,1,2,1]),
            PieceConfig::new(3,2, &[1,2,2,0,1,0]),
            PieceConfig::new(2,3, &[1,2,1,0,0,2]),
            PieceConfig::new(3,2, &[0,1,0,2,2,1]),
            PieceConfig::new(2,3, &[0,0,3,1,4,1]),
            PieceConfig::new(3,2, &[3,1,0,4,0,1]),
            PieceConfig::new(2,3, &[1,4,1,3,0,0]),
            PieceConfig::new(3,2, &[1,0,4,0,1,3]),
        ];
    
        let tetr_5: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[0,0,1,2,1,2]),
            PieceConfig::new(3,2, &[1,2,0,1,0,2]),
            PieceConfig::new(2,3, &[2,1,2,1,0,0]),
            PieceConfig::new(3,2, &[2,0,1,0,2,1]),
            PieceConfig::new(2,3, &[1,0,0,4,1,3]),
            PieceConfig::new(3,2, &[4,1,1,0,3,0]),
            PieceConfig::new(2,3, &[3,1,4,0,0,1]),
            PieceConfig::new(3,2, &[0,3,0,1,1,4]),
        ];
    
    
        let tetr_6: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[1,0,0,2,1,2]),
            PieceConfig::new(3,2, &[2,1,1,0,2,0]),
            PieceConfig::new(2,3, &[2,1,2,0,0,1]),
            PieceConfig::new(3,2, &[0,2,0,1,1,2]),
            PieceConfig::new(2,3, &[0,0,4,1,3,1]),
            PieceConfig::new(3,2, &[4,1,0,3,0,1]),
            PieceConfig::new(2,3, &[1,3,1,4,0,0]),
            PieceConfig::new(3,2, &[1,0,3,0,1,4]),
        ];
    
        let tetr_7: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[0,1,0,1,2,1]),
            PieceConfig::new(3,2, &[1,0,2,1,1,0]),
            PieceConfig::new(2,3, &[1,2,1,0,1,0]),
            PieceConfig::new(3,2, &[0,1,1,2,0,1]),
            PieceConfig::new(2,3, &[0,3,0,4,1,3]),
            PieceConfig::new(3,2, &[4,0,1,3,3,0]),
            PieceConfig::new(2,3, &[3,1,4,0,3,0]),
            PieceConfig::new(3,2, &[0,3,3,1,0,4]),
        ];
    
        let tetr_8: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[0,2,0,2,1,2]),
            PieceConfig::new(3,2, &[2,0,1,2,2,0]),
            PieceConfig::new(2,3, &[2,1,2,0,2,0]),
            PieceConfig::new(3,2, &[0,2,2,1,0,2]),
            PieceConfig::new(2,3, &[0,1,0,1,4,1]),
            PieceConfig::new(3,2, &[1,0,4,1,1,0]),
            PieceConfig::new(2,3, &[1,4,1,0,1,0]),
            PieceConfig::new(3,2, &[0,1,1,4,0,1]),
        ];
    
        let tetr_9: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[0,1,2,1,2,0]),
            PieceConfig::new(3,2, &[2,0,1,2,0,1]),
            PieceConfig::new(2,3, &[0,2,1,2,1,0]),
            PieceConfig::new(3,2, &[1,0,2,1,0,2]),
            PieceConfig::new(2,3, &[1,4,0,0,1,3]),
            PieceConfig::new(3,2, &[0,1,1,4,3,0]),
            PieceConfig::new(2,3, &[3,1,0,0,4,1]),
            PieceConfig::new(3,2, &[0,3,4,1,1,0]),
        ];
    
        let tetr_10: Vec<PieceConfig> = vec![
            PieceConfig::new(2,3, &[1,2,0,0,1,2]),
            PieceConfig::new(3,2, &[0,1,1,2,2,0]),
            PieceConfig::new(2,3, &[2,1,0,0,2,1]),
            PieceConfig::new(3,2, &[0,2,2,1,1,0]),
            PieceConfig::new(2,3, &[0,3,1,4,1,0]),
            PieceConfig::new(3,2, &[4,0,1,3,0,1]),
            PieceConfig::new(2,3, &[0,1,4,1,3,0]),
            PieceConfig::new(3,2, &[1,0,3,1,0,4]),
        ];
    
        let oct_1: Vec<PieceConfig> = vec![
            PieceConfig::new(1,8, &[1,2,1,2,1,2,1,2]),
            PieceConfig::new(1,8, &[2,1,2,1,2,1,2,1]),
            PieceConfig::new(8,1, &[1,2,1,2,1,2,1,2]),
            PieceConfig::new(8,1, &[2,1,2,1,2,1,2,1]),
            PieceConfig::new(1,8, &[1,3,1,4,1,3,1,4]),
            PieceConfig::new(1,8, &[4,1,3,1,4,1,3,1]),
            PieceConfig::new(8,1, &[1,3,1,4,1,3,1,4]),
            PieceConfig::new(8,1, &[4,1,3,1,4,1,3,1]),
        ];
    
        let pieces: Vec<Vec<PieceConfig>> = vec![mono_1, mono_2, domo_1, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
        pieces
    }
    
    pub fn load_pieces() -> Vec<Piece> {
        let piece_configs = Self::load_piece_configs();
    
        let mut pieces: Vec<Piece> = Vec::new();
        for (i, configs) in piece_configs.iter().enumerate() {
            pieces.push(Piece::new(i, configs.clone()));
        }
    
        pieces
    }

    // maps 2d coordinates to 1d index
    fn board_idx(&self, x: usize, y: usize) -> usize {
        return x * 8 + y;
    }

    // place piece on board (ie. make a move)
	pub fn set(&mut self, piece_idx: usize, m: Move) {

        let Move {
            row,
            col,
            config_idx,
        } = m;

        let piece = &self.pieces[piece_idx].configs[config_idx];
        let dim_1 = usize::from(piece.width);
        let dim_2 = usize::from(piece.height);

        for x in 0..dim_1 {
            for y in 0..dim_2 {
                let color = self.pieces[piece_idx].get_piece_color(config_idx, x, y);
                if color != 0 {     // only place non-empty cells
                    let idx = self.board_idx(col + x, row + y);
                    self.board[idx] = Some(u8::try_from(piece_idx).unwrap());
                }
            }
        }
	}

    // remove piece on board (ie. undo a move)
	pub fn remove(&mut self, piece_idx: usize) {

        for x in 0..8 {
            for y in 0..8 {
                let idx = self.board_idx(x, y);
                match self.board[idx] {
                    Some(piece) => {
                        if piece == piece_idx as u8 {
                            self.board[idx] = None;
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
                let idx = self.board_idx(x, y);
                let val = match self.board[idx] {
                    Some(piece) => String::from_utf8(vec![(17 - piece) + 65]).unwrap(),
                    None => " ".to_string(),
                };
                match self.refboard[idx] {
                    1 => print!("{}{} ", color::Fg(color::White), val),
                    2 => print!("{}{} ", color::Fg(color::Red), val),
                    3 => print!("{}{} ", color::Fg(color::Yellow), val),
                    4 => print!("{}{} ", color::Fg(color::Blue), val),
                    _ => print!("{}{} ", color::Fg(color::Reset), val),
                };
            }
            println!();
		}
	}

    pub fn print_ref(&self) {
		println!("----------------");
        for x in 0..8 {
		    for y in 0..8 {
                let idx = self.board_idx(x, y);
                match self.refboard[idx] {
                    1 => print!("{}□ ", color::Fg(color::White)),
                    2 => print!("{}■ ", color::Fg(color::Red)),
                    3 => print!("{}■ ", color::Fg(color::Yellow)),
                    4 => print!("{}■ ", color::Fg(color::Blue)),
                    _ => print!("{}■ ", color::Fg(color::Reset)),
                };
            }
            println!();
		}
	}

	pub fn is_solved(&self) -> bool {
        self.board.iter().all(Option::is_some)
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
                let dim_1 = usize::from(config.width);
                let dim_2 = usize::from(config.height);

                for x in 0..dim_1 {
                    for y in 0..dim_2 {

                        let board_x = pos_x + x;
                        let board_y = pos_y + y;
                        // check if piece fits in the board
                        if board_x >= 8 || board_y >= 8 {
                            continue 'next_config;
                        }

                        // check if piece color matches the board
                        let idx = self.board_idx(board_x, board_y);
                        let color = piece.get_piece_color(config_idx, x, y);
                        if color != 0 {                                     // non-empty piece color
                            if self.board[idx].is_some() {        // non-empty board color
                                continue 'next_config;
                            }
                            if color != self.refboard[idx] {   // mismatched board color
                                continue 'next_config;
                            }
                        }
                    }
                }
                res.push_back(Move {
                    row: pos_y,
                    col: pos_x,
                    config_idx,
                });
            }
        }
        res
    }
}
