mod solver;
mod generator;

use std::{fs, str::FromStr};
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use termion::{color::{self}};

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

    pub fn get_piece_color(&self, config_idx: usize, x: usize, y: usize) -> u8 {
        let piece = &self.configs[config_idx];
        let dims = &piece[..2];
        let piece_colors: &[u8] = &piece[2..];
        let idx: usize = x * dims[1] as usize + y;
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
                        _ => print!("{}■ ", color::Fg(color::Reset)),
                    };
                } else {
                    print!("{}□ ", color::Fg(color::White));
                }
            }
            println!("");
        }
    }
}

// function to read and parse json given local file path
// returns a flat vector of colors representing the board
pub fn load_game_str(path: &Path, game: &str) -> String {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let data: Data = serde_json::from_str(&file).expect("Unable to parse json");
    return String::from_str(data.0.get(game).unwrap()).unwrap();
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
        vec![1,3, 2,1,1],
        vec![1,3, 3,1,4],
        vec![1,3, 4,1,3],
        vec![3,1, 2,1,1],
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
        vec![2,2, 0,1,2,1],
        vec![2,2, 1,0,2,1],
        vec![2,2, 1,2,0,1],
        vec![2,2, 1,2,1,0],
        vec![2,2, 0,1,3,1],
        vec![2,2, 1,0,3,1],
        vec![2,2, 1,3,0,1],
        vec![2,2, 1,3,1,0],
    ];

    let trom_4: Vec<PieceConfig> = vec![
        vec![2,2, 0,2,1,2],
        vec![2,2, 2,0,1,2],
        vec![2,2, 2,1,0,2],
        vec![2,2, 2,1,2,0],
        vec![2,2, 0,4,3,1],
        vec![2,2, 4,0,3,1],
        vec![2,2, 4,3,0,1],
        vec![2,2, 4,3,1,0],
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
        vec![2,2, 2,1,2,1],
        vec![2,2, 1,2,1,2],
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
    let piece_configs = load_piece_configs();

    let mut pieces: Vec<Piece> = Vec::new();
    for (i, configs) in piece_configs.iter().enumerate() {
        pieces.push(Piece::new(i, configs.clone()));
    }

    return pieces;
}