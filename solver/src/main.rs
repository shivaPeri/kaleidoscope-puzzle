use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    EMPTY = 0,
    BLACK = 1,
    RED = 2,
    YELLOW = 3,
    BLUE = 4
}

enum Orientation {
    ORIG,
    ROT90,
    ROT180,
    ROT270,
    FLIP,
    FLIP_ROT90,
    FLIP_ROT180,
    FLIP_ROT270
}

#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

type Board = Vec<Color>;
type Piece = Vec<u8>;

const BOARD_SIZE: usize = 8;

fn make_pieces() -> Vec<Piece> {
    
    // first three vals are dimensions of piece, then colors

    // Monominoes
    let mono_1: Vec<u8> = vec![1,1,2, 2,1];
    let mono_2: Vec<u8> = vec![1,1,2, 1,4];

    // Dominoes
    let domi: Vec<u8> = vec![1,2,2, 2,1,1,4];

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

    let pieces = vec![mono_1, mono_2, domi, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
    return pieces;
}

fn place_piece(piece: &Piece, orientation: Orientation) {
    // TODO

    let dim_1 = piece[0];
    let dim_2 = piece[1];
    let dim_3 = piece[2];

    match orientation {
        Orientation::ORIG => {
            // do nothing
        },
        Orientation::ROT90 => {
            // rotate 90 degrees
        },
        Orientation::ROT180 => {
            // rotate 180 degrees
        },
        Orientation::ROT270 => {
            // rotate 270 degrees
        },
        Orientation::FLIP => {
            // flip horizontally
        },
        Orientation::FLIP_ROT90 => {
            // flip horizontally, then rotate 90 degrees
        },
        Orientation::FLIP_ROT180 => {
            // flip horizontally, then rotate 180 degrees
        },
        Orientation::FLIP_ROT270 => {
            // flip horizontally, then rotate 270 degrees
        },
    }

}

// function to read and parse json given local file path
fn load_game(path: &Path, game: &str) -> Board {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let board = load_game(Path::new("../boards/scraped-boards.json"), "australian-emu");
    let pieces = make_pieces();

    println!("{:?}", board);
    println!("{:?}", pieces);
    Ok(())
}