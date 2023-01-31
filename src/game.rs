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

pub type Board = Vec<Color>;
pub type Piece = Vec<u8>;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Piece2 {
    idx: u8,
    configs: Vec<Piece>
}

pub const BOARD_SIZE: usize = 8;

pub fn load_pieces() -> Vec<Piece2> {

    let mono_1 = Piece2{
        idx: 1,
        configs: vec![
            vec![1,1, 2],
            vec![1,1, 1],
        ],
    };

    let mono_2 = Piece2{
        idx: 2,
        configs: vec![
            vec![1,1, 4],
            vec![1,1, 1],
        ],
    };

    let domo_1 = Piece2{
        idx: 3,
        configs: vec![
            vec![1,2, 2,1],
            vec![1,2, 1,2],
            vec![1,2, 4,1],
            vec![1,2, 1,4],
            vec![2,1, 2,1],
            vec![2,1, 1,2],
            vec![2,1, 4,1],
            vec![2,1, 1,4],
        ],
    };

    let trom_1 = Piece2{
        idx: 4,
        configs: vec![
            vec![1,3, 2,1,1],
            vec![1,3, 3,1,4],
            vec![1,3, 4,1,3],
            vec![3,1, 2,1,1],
            vec![3,1, 3,1,4],
            vec![3,1, 4,1,3],
        ],
    };

    let trom_2 = Piece2{
        idx: 5,
        configs: vec![
            vec![1,3, 1,2,1],
            vec![1,3, 1,3,1],
            vec![3,1, 1,2,1],
            vec![3,1, 1,3,1],
        ],
    };

    let trom_3 = Piece2{
        idx: 6,
        configs: vec![
            vec![2,2, 0,1,2,1],
            vec![2,2, 1,0,2,1],
            vec![2,2, 1,2,0,1],
            vec![2,2, 1,2,1,0],
            vec![2,2, 0,1,3,1],
            vec![2,2, 1,0,3,1],
            vec![2,2, 1,3,0,1],
            vec![2,2, 1,3,1,0],
        ],
    };

    // this might be wrong
    let trom_4 = Piece2{
        idx: 7,
        configs: vec![
            vec![2,2, 0,2,1,2],
            vec![2,2, 2,0,1,2],
            vec![2,2, 2,1,0,2],
            vec![2,2, 2,1,2,0],
            vec![2,2, 0,4,3,1],
            vec![2,2, 4,0,3,1],
            vec![2,2, 4,3,0,1],
            vec![2,2, 4,3,1,0],
        ],
    };

    let tetr_1 = Piece2{
        idx: 8,
        configs: vec![
            vec![1,4, 2,1,2,1],
            vec![1,4, 1,2,1,2],
            vec![1,4, 4,1,3,1],
            vec![1,4, 1,3,1,4],
        ],
    };

    let tetr_2 = Piece2{
        idx: 9,
        configs: vec![
            vec![2,2, 2,1,2,1],
            vec![2,2, 1,2,1,2],
            vec![2,2, 4,1,1,3],
            vec![2,2, 3,1,1,4],
            vec![2,2, 1,3,4,1],
            vec![2,2, 1,4,3,1],
        ],
    };

    let tetr_3 = Piece2{
        idx: 10,
        configs: vec![
            vec![2,3, 0,0,2,1,2,1],
            vec![3,2, 2,1,0,2,0,1],
            vec![2,3, 1,2,1,2,0,0],
            vec![3,2, 1,0,2,0,1,2],
            vec![2,3, 1,0,0,3,1,4],
            vec![3,2, 3,1,1,0,4,0],
            vec![2,3, 4,1,3,0,0,1],
            vec![3,2, 0,4,0,1,1,3],
        ],
    };

    let tetr_4 = Piece2{
        idx: 11,
        configs: vec![
            vec![2,3, 2,0,0,1,2,1],
            vec![3,2, 1,2,2,0,1,0],
            vec![2,3, 1,2,1,0,0,2],
            vec![3,2, 0,1,0,2,2,1],
            vec![2,3, 0,0,3,1,4,1],
            vec![3,2, 3,1,0,4,0,1],
            vec![2,3, 1,4,1,3,0,0],
            vec![3,2, 1,0,4,0,1,3],
        ],
    };

    let tetr_5 = Piece2{
        idx: 12,
        configs: vec![
            vec![2,3, 0,0,1,2,1,2],
            vec![3,2, 1,2,0,1,0,2],
            vec![2,3, 2,1,2,1,0,0],
            vec![3,2, 2,0,1,0,2,1],
            vec![2,3, 1,0,0,4,1,3],
            vec![3,2, 4,1,1,0,3,0],
            vec![2,3, 3,1,4,0,0,1],
            vec![3,2, 0,3,0,1,1,4],
        ],
    };


    let tetr_6 = Piece2{
        idx: 13,
        configs: vec![
            vec![2,3, 1,0,0,2,1,2],
            vec![3,2, 2,1,1,0,2,0],
            vec![2,3, 2,1,2,0,0,1],
            vec![3,2, 0,2,0,1,1,2],
            vec![2,3, 0,0,4,1,3,1],
            vec![3,2, 4,1,0,3,0,1],
            vec![2,3, 1,3,1,4,0,0],
            vec![3,2, 1,0,3,0,1,4],
        ],
    };

    let tetr_7 = Piece2{
        idx: 14,
        configs: vec![
            vec![2,3, 0,1,0,1,2,1],
            vec![3,2, 1,0,2,1,1,0],
            vec![2,3, 1,2,1,0,1,0],
            vec![3,2, 0,1,1,2,0,1],
            vec![2,3, 0,3,0,4,1,3],
            vec![3,2, 4,0,1,3,3,0],
            vec![2,3, 3,1,4,0,3,0],
            vec![3,2, 0,3,3,1,0,4],
        ],
    };

    let tetr_8 = Piece2{
        idx: 15,
        configs: vec![
            vec![2,3, 0,2,0,2,1,2],
            vec![3,2, 2,0,1,2,2,0],
            vec![2,3, 2,1,2,0,2,0],
            vec![3,2, 0,2,2,1,0,2],
            vec![2,3, 0,1,0,1,4,1],
            vec![3,2, 1,0,4,1,1,0],
            vec![2,3, 1,4,1,0,1,0],
            vec![3,2, 0,1,1,4,0,1],
        ],
    };

    let tetr_9 = Piece2{
        idx: 16,
        configs: vec![
            vec![2,3, 0,1,2,1,2,0],
            vec![3,2, 2,0,1,2,0,1],
            vec![2,3, 0,2,1,2,1,0],
            vec![3,2, 1,0,2,1,0,2],
            vec![2,3, 1,4,0,0,1,3],
            vec![3,2, 0,1,1,4,3,0],
            vec![2,3, 3,1,0,0,4,1],
            vec![3,2, 0,3,4,1,1,0],
        ],
    };

    let tetr_10 = Piece2{
        idx: 17,
        configs: vec![
            vec![2,3, 1,2,0,0,1,2],
            vec![3,2, 0,1,1,2,2,0],
            vec![2,3, 2,1,0,0,2,1],
            vec![3,2, 0,2,2,1,1,0],
            vec![2,3, 0,3,1,4,1,0],
            vec![3,2, 4,0,1,3,0,1],
            vec![2,3, 0,1,4,1,3,0],
            vec![3,2, 1,0,3,1,0,4],
        ],
    };

    let oct_1 = Piece2{
        idx: 18,
        configs: vec![
            vec![1,8, 1,2,1,2,1,2,1,2],
            vec![1,8, 2,1,2,1,2,1,2,1],
            vec![8,1, 1,2,1,2,1,2,1,2],
            vec![8,1, 2,1,2,1,2,1,2,1],
            vec![1,8, 1,3,1,4,1,3,1,4],
            vec![1,8, 4,1,3,1,4,1,3,1],
            vec![8,1, 1,3,1,4,1,3,1,4],
            vec![8,1, 4,1,3,1,4,1,3,1],
        ],
    };

    let pieces: Vec<Piece2> = vec![mono_1, mono_2, domo_1, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
    return pieces;
}

// TODO: need to only check if non-0 squares are filled
fn in_bounds(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) -> bool {

    let piece = &pieces[turn.piece].configs[turn.config];

    let mut d1 = piece[0];
    let mut d2 = piece[1];
    
    if (turn.row + d1 as usize > BOARD_SIZE || turn.col + d2 as usize > BOARD_SIZE) {
        return false;
    }

    // TODO: review this later
    for i in 0..d1 {
        for j in 0..d2 {
            let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
            let piece_coord = [i, j, 0];
            let piece_idx = piece_coord[0] * d1 as usize + piece_coord[1] * d2 as usize + piece_coord[2] * 1;
            let piece_color = piece[2 + piece_idx];

            if piece_color != 0 && board[board_idx] != 0 {
                return false;
            }
        }
    }

    return true;
}

pub fn place_piece(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) {

    if !in_bounds(board, pieces, turn) { return; }

    let piece = &pieces[turn.piece].configs[turn.config];
    let dims: &[u8] = &piece[..2];
    let piece_colors: &[u8] = &piece[2..];

    // TODO: review this later
    for i in 0..dims[0] {
        for j in 0..dims[1] {
            let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
            let piece_coord = [i, j, 0];
            let piece_idx = piece_coord[0] * dims[0] as usize + piece_coord[1] * dims[1] as usize + piece_coord[2] * 1;
            let piece_color = piece_colors[piece_idx];

            if piece_color != 0 {
                board[board_idx] = piece_color;
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

    // take game_str and create a vector mapping each character to corresponding color
    let mut board = Board::new();
    for c in game_str.chars() {
        let color = match c {
            '1' => Color::BLACK,
            '2' => Color::RED,
            '3' => Color::YELLOW,
            '4' => Color::BLUE,
            _ => Color::EMPTY,
        };
        board.push(color);
    }

    return board;
}