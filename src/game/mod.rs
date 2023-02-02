mod solver;
mod generator;

use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Color {
    EMPTY = 0,
    BLACK = 1,
    RED = 2,
    YELLOW = 3,
    BLUE = 4
}

pub struct Move {
    piece: usize,               // index of piece in pieces vector
    config: usize,              // index of config in piece vector
    row: usize,                 // row of top left corner
    col: usize,                 // col of top left corner
}

#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

pub type Board = Vec<u8>;
pub type PieceConfig = Vec<u8>;
pub type Piece = Vec<PieceConfig>;

#[derive(Default, Serialize, Deserialize, Debug)]

const BOARD_SIZE: usize = 8;

pub fn load_pieces() -> Vec<Piece> {

    let mono_1: Piece = vec![
        vec![1,1, 2],
        vec![1,1, 1],
    ];

    let mono_2: Piece = vec![
        vec![1,1, 4],
        vec![1,1, 1],
    ];

    let domo_1: Piece = vec![
        vec![1,2, 2,1],
        vec![1,2, 1,2],
        vec![1,2, 4,1],
        vec![1,2, 1,4],
        vec![2,1, 2,1],
        vec![2,1, 1,2],
        vec![2,1, 4,1],
        vec![2,1, 1,4],
    ];

    let trom_1: Piece = vec![
        vec![1,3, 2,1,1],
        vec![1,3, 3,1,4],
        vec![1,3, 4,1,3],
        vec![3,1, 2,1,1],
        vec![3,1, 3,1,4],
        vec![3,1, 4,1,3],
    ];

    let trom_2: Piece = vec![
        vec![1,3, 1,2,1],
        vec![1,3, 1,3,1],
        vec![3,1, 1,2,1],
        vec![3,1, 1,3,1],
    ];

    let trom_3: Piece = vec![
        vec![2,2, 0,1,2,1],
        vec![2,2, 1,0,2,1],
        vec![2,2, 1,2,0,1],
        vec![2,2, 1,2,1,0],
        vec![2,2, 0,1,3,1],
        vec![2,2, 1,0,3,1],
        vec![2,2, 1,3,0,1],
        vec![2,2, 1,3,1,0],
    ];

    let trom_4: Piece = vec![
        vec![2,2, 0,2,1,2],
        vec![2,2, 2,0,1,2],
        vec![2,2, 2,1,0,2],
        vec![2,2, 2,1,2,0],
        vec![2,2, 0,4,3,1],
        vec![2,2, 4,0,3,1],
        vec![2,2, 4,3,0,1],
        vec![2,2, 4,3,1,0],
    ];

    let tetr_1: Piece = vec![
        vec![1,4, 2,1,2,1],
        vec![1,4, 1,2,1,2],
        vec![1,4, 4,1,3,1],
        vec![1,4, 1,3,1,4],
    ];

    let tetr_2: Piece = vec![
        vec![2,2, 2,1,2,1],
        vec![2,2, 1,2,1,2],
        vec![2,2, 4,1,1,3],
        vec![2,2, 3,1,1,4],
        vec![2,2, 1,3,4,1],
        vec![2,2, 1,4,3,1],
    ];

    let tetr_3: Piece = vec![
        vec![2,3, 0,0,2,1,2,1],
        vec![3,2, 2,1,0,2,0,1],
        vec![2,3, 1,2,1,2,0,0],
        vec![3,2, 1,0,2,0,1,2],
        vec![2,3, 1,0,0,3,1,4],
        vec![3,2, 3,1,1,0,4,0],
        vec![2,3, 4,1,3,0,0,1],
        vec![3,2, 0,4,0,1,1,3],
    ];

    let tetr_4: Piece = vec![
        vec![2,3, 2,0,0,1,2,1],
        vec![3,2, 1,2,2,0,1,0],
        vec![2,3, 1,2,1,0,0,2],
        vec![3,2, 0,1,0,2,2,1],
        vec![2,3, 0,0,3,1,4,1],
        vec![3,2, 3,1,0,4,0,1],
        vec![2,3, 1,4,1,3,0,0],
        vec![3,2, 1,0,4,0,1,3],
    ];

    let tetr_5: Piece = vec![
        vec![2,3, 0,0,1,2,1,2],
        vec![3,2, 1,2,0,1,0,2],
        vec![2,3, 2,1,2,1,0,0],
        vec![3,2, 2,0,1,0,2,1],
        vec![2,3, 1,0,0,4,1,3],
        vec![3,2, 4,1,1,0,3,0],
        vec![2,3, 3,1,4,0,0,1],
        vec![3,2, 0,3,0,1,1,4],
    ];


    let tetr_6: Piece = vec![
        vec![2,3, 1,0,0,2,1,2],
        vec![3,2, 2,1,1,0,2,0],
        vec![2,3, 2,1,2,0,0,1],
        vec![3,2, 0,2,0,1,1,2],
        vec![2,3, 0,0,4,1,3,1],
        vec![3,2, 4,1,0,3,0,1],
        vec![2,3, 1,3,1,4,0,0],
        vec![3,2, 1,0,3,0,1,4],
    ];

    let tetr_7: Piece = vec![
        vec![2,3, 0,1,0,1,2,1],
        vec![3,2, 1,0,2,1,1,0],
        vec![2,3, 1,2,1,0,1,0],
        vec![3,2, 0,1,1,2,0,1],
        vec![2,3, 0,3,0,4,1,3],
        vec![3,2, 4,0,1,3,3,0],
        vec![2,3, 3,1,4,0,3,0],
        vec![3,2, 0,3,3,1,0,4],
    ];

    let tetr_8: Piece = vec![
        vec![2,3, 0,2,0,2,1,2],
        vec![3,2, 2,0,1,2,2,0],
        vec![2,3, 2,1,2,0,2,0],
        vec![3,2, 0,2,2,1,0,2],
        vec![2,3, 0,1,0,1,4,1],
        vec![3,2, 1,0,4,1,1,0],
        vec![2,3, 1,4,1,0,1,0],
        vec![3,2, 0,1,1,4,0,1],
    ];

    let tetr_9: Piece = vec![
        vec![2,3, 0,1,2,1,2,0],
        vec![3,2, 2,0,1,2,0,1],
        vec![2,3, 0,2,1,2,1,0],
        vec![3,2, 1,0,2,1,0,2],
        vec![2,3, 1,4,0,0,1,3],
        vec![3,2, 0,1,1,4,3,0],
        vec![2,3, 3,1,0,0,4,1],
        vec![3,2, 0,3,4,1,1,0],
    ];

    let tetr_10: Piece = vec![
        vec![2,3, 1,2,0,0,1,2],
        vec![3,2, 0,1,1,2,2,0],
        vec![2,3, 2,1,0,0,2,1],
        vec![3,2, 0,2,2,1,1,0],
        vec![2,3, 0,3,1,4,1,0],
        vec![3,2, 4,0,1,3,0,1],
        vec![2,3, 0,1,4,1,3,0],
        vec![3,2, 1,0,3,1,0,4],
    ];

    let oct_1: Piece = vec![
        vec![1,8, 1,2,1,2,1,2,1,2],
        vec![1,8, 2,1,2,1,2,1,2,1],
        vec![8,1, 1,2,1,2,1,2,1,2],
        vec![8,1, 2,1,2,1,2,1,2,1],
        vec![1,8, 1,3,1,4,1,3,1,4],
        vec![1,8, 4,1,3,1,4,1,3,1],
        vec![8,1, 1,3,1,4,1,3,1,4],
        vec![8,1, 4,1,3,1,4,1,3,1],
    ];

    let pieces: Vec<Piece> = vec![mono_1, mono_2, domo_1, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
    return pieces;
}

fn is_placeable(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) -> bool {

    let piece = &pieces[turn.piece].configs[turn.config];
    let dims: &[u8] = &piece[..2];
    let piece_colors: &[u8] = &piece[2..];
    
    if (turn.row + dims[0] as usize > BOARD_SIZE || turn.col + dims[1] as usize > BOARD_SIZE) {
        return false;
    }

    for i in 0..dims[0] {
        for j in 0..dims[1] {
            let board_idx: usize = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
            let piece_idx: usize = (i * dims[1]) as usize + j as usize;

            // non-empty piece color and non-empty board color => collision
            if piece_colors[piece_idx] != 0 && board[board_idx] != 0 {
                return false;
            }
        }
    }

    return true;
}

pub fn place_piece(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) {

    if !is_placeable(board, pieces, turn) { return; }

    let piece = &pieces[turn.piece].configs[turn.config];
    let dims: &[u8] = &piece[..2];
    let piece_colors: &[u8] = &piece[2..];

    for i in 0..dims[0] {
        for j in 0..dims[1] {
            let board_idx: usize = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
            let piece_idx: usize = (i * dims[1]) as usize + j as usize;

            // place non-empty piece color on board
            if piece_colors[piece_idx] != 0 {
                board[board_idx] = piece_colors[piece_idx];
            }
        }
    }

}

// function to read and parse json given local file path
// returns a flat vector of colors representing the board
pub fn load_game(path: &Path, game: &str) -> Board {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let data: Data = serde_json::from_str(&file).expect("Unable to parse json");
    let game_str = data.0.get(game).unwrap();

    // take game_str and map each character to parsed integer as u8
    let mut board = game_str.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    return board;
}

// function that prints board color codes to terminal
pub fn print_board(board: &Board) {
    for i in 0..BOARD_SIZE {
        for j in 0..BOARD_SIZE {
            let idx: usize = i * BOARD_SIZE + j;
            print!("{} ", board[idx]);
        }
        println!();
    }
}