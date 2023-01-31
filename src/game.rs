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

pub enum Orientation {
    Orig,
    Rot90,
    Rot180,
    Rot270,
    Flip,
    Rot90Flip,
    Rot180Flip,
    Rot270Flip
}

pub struct Move {
    pub piece: usize,               // index of piece in pieces vector
    pub row: usize,                 // x coord of top left corner
    pub col: usize,                 // y coord of top left corner
    pub orientation: Orientation    // orientation of piece
}

#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

pub type Board = Vec<Color>;
pub type Piece = Vec<u8>;

pub const BOARD_SIZE: usize = 8;

// makes vector of all the pieces
// first three vals are dimensions of piece, then colors
pub fn load_pieces() -> Vec<Piece> {
    
    // Monominoes
    let mono_1: Vec<u8> = vec![1,1,2, 2,1];
    let mono_2: Vec<u8> = vec![1,1,2, 1,4];
    
    // Dominoes
    let domi: Vec<u8> = vec![1,2,2, 2,1,1,4];
    
    // Trominoes
    let trom_1: Vec<u8> = vec![1,3,2, 2,3,1,1,2,4];
    let trom_2: Vec<u8> = vec![1,3,2, 1,1,2,3,1,1];
    let trom_3: Vec<u8> = vec![2,2,2, 2,4,0,0,1,1,2,3];
    let trom_4: Vec<u8> = vec![2,2,2, 1,1,0,0,2,3,1,1];
    
    // Tetrominoes
    let tetr_1: Vec<u8> = vec![1,4,2, 2,4,1,1,2,3,1,1];
    let tetr_2: Vec<u8> = vec![2,2,2, 2,1,1,4,1,3,2,1];
    let tetr_3: Vec<u8> = vec![2,3,2, 1,1,2,3,0,0,0,0,1,1,2,4];
    let tetr_4: Vec<u8> = vec![2,3,2, 0,0,2,1,1,3,2,1,1,4,0,0];
    let tetr_5: Vec<u8> = vec![2,3,2, 0,0,2,1,0,0,2,1,1,4,2,1];
    let tetr_6: Vec<u8> = vec![2,3,2, 0,0,1,3,0,0,1,3,2,1,1,4];
    let tetr_7: Vec<u8> = vec![2,3,2, 1,4,0,0,0,0,2,1,1,3,2,1];
    let tetr_8: Vec<u8> = vec![2,3,2, 2,3,0,0,0,0,1,1,2,4,1,1];
    let tetr_9: Vec<u8> = vec![2,3,2, 0,0,0,0,1,1,2,3,1,1,2,4];
    let tetr_10: Vec<u8> = vec![2,3,2, 0,0,0,0,2,1,1,4,2,1,1,3];
    
    // Octominoes
    let oct_1: Vec<u8> = vec![1,8,2, 2,1,1,3,2,1,1,4,2,1,1,3,2,1,1,4];
    
    let pieces: Vec<Piece> = vec![mono_1, mono_2, domi, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
    return pieces;
}

fn is_placeable(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) -> bool {

    let piece = &pieces[turn.piece];

    let mut d1 = piece[0];
    let mut d2 = piece[1];
    
    if (turn.orientation == Orientation::Rot90 || 
        turn.orientation == Orientation::Rot270 ||
        turn.orientation == Orientation::Rot90Flip ||
        turn.orientation == Orientation::Rot270Flip) {
        d1 = piece[1];
        d2 = piece[0];
    }

    if (turn.row + d1 as usize > BOARD_SIZE || turn.col + d2 as usize > BOARD_SIZE) {
        return false;
    }

    for i in 0..d1 {
        for j in 0..d2 {
            let index = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
            if board[index] == Color.EMPTY {
                return false;
            }
        }
    }

    return true;
}

// TODO: implement this function
// assume the piece is in bounds and placeable
pub fn place_piece(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) {

    if !is_placeable(board, pieces, turn) {
        return;
    }

    let piece = &pieces[turn.piece];
    let dims: &[u8] = &piece[..3];
    let piece_colors: &[u8] = &piece[3..];

    let mut d1;
    let mut d1_flip;
    let mut d2;
    let mut d2_flip;

    let mut d3;

    match turn.orientation {
        Orientation::Orig => {
            // do nothing
            d1 = dims[0];
            d1_flip = false;
            d2 = dims[1];
            d2_flip = false;
            d3 = 0;
        },
        Orientation::Rot90 => {
            // rotate 90 degrees
            d1 = dims[1];
            d1_flip = true;
            d2 = dims[0];
            d2_flip = false;
            d3 = 0;
        },
        Orientation::Rot180 => {
            // rotate 180 degrees
            d1 = dims[0];
            d1_flip = true;
            d2 = dims[1];
            d2_flip = true;
            d3 = 0;
        },
        Orientation::Rot270 => {
            // rotate 270 degrees
            d1 = dims[1];
            d1_flip = false;
            d2 = dims[0];
            d2_flip = true;
            d3 = 0;
        },
        Orientation::Flip => {
            // flip vertically
            d1 = dims[0];
            d1_flip = false;
            d2 = dims[1];
            d2_flip = true;
            d3 = 1;
        },
        Orientation::Rot90Flip => {
            // rotate 90 degrees, then flip vertically
            d1 = dims[1];
            d1_flip = true;
            d2 = dims[0];
            d2_flip = true;
            d3 = 1;
        },
        Orientation::Rot180Flip => {
            // rotate 180 degrees, then flip vertically
            d1 = dims[0];
            d1_flip = true;
            d2 = dims[1];
            d2_flip = false;
            d3 = 1;
        },
        Orientation::Rot270Flip => {
            // rotate 270 degrees, then flip vertically
            d1 = dims[1];
            d1_flip = false;
            d2 = dims[0];
            d2_flip = false;
            d3 = 1;
        },
    }

    // this is the actual piece placement
    // TODO: double check this
    for i in 0..d1 {
        for j in 0..d2 {
            let mut index = turn.row * BOARD_SIZE + turn.col;
            if d1_flip {
                index += (d1 - i - 1) * BOARD_SIZE;
            } else {
                index += i * BOARD_SIZE;
            }
            if d2_flip {
                index += d2 - j - 1;
            } else {
                index += j;
            }

            let piece_idx = i * d2 + j + d3;
            board[index] = piece_colors[piece_idx as usize];
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