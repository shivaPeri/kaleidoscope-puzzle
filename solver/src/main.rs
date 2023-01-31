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
    Orig,
    Rot90,
    Rot180,
    Rot270,
    Flip,
    Rot90Flip,
    Rot180Flip,
    Rot270Flip
}

#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

type Board = Vec<Color>;
type Piece = Vec<u8>;

const BOARD_SIZE: usize = 8;

// makes vector of all the pieces
// first three vals are dimensions of piece, then colors
fn load_pieces() -> Vec<Piece> {
    
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

// TODO: implement this function
fn place_piece(piece: &Piece, orientation: Orientation) {

    let dims: &[u8] = &piece[..3];
    let piece_colors: &[u8] = &piece[3..];

    let mut d1: usize = 0;
    let mut d2: usize = 1;
    let d1_dir: usize;
    let d2_dir: usize;
    let d3: u8;

    match orientation {
        Orientation::Orig => {
            // do nothing
            d1 = 0;
            d2 = 1;
            d3 = 0;
        },
        Orientation::Rot90 => {
            // rotate 90 degrees
            d3 = 0;
        },
        Orientation::Rot180 => {
            // rotate 180 degrees
            d3 = 0;
        },
        Orientation::Rot270 => {
            // rotate 270 degrees
            d3 = 0;
        },
        Orientation::Flip => {
            // flip vertically
            d3 = 1;
        },
        Orientation::Rot90Flip => {
            // rotate 90 degrees, then flip vertically
            d3 = 1;
        },
        Orientation::Rot180Flip => {
            // rotate 180 degrees, then flip vertically
            d3 = 1;
        },
        Orientation::Rot270Flip => {
            // rotate 270 degrees, then flip vertically
            d3 = 1;
        },
    }

    for i in 0..dims[d1] {
        for j in 0..dims[d2] {
            let color = piece_colors[(i*dims[1]*dims[2] + j*dims[2] + d3) as usize];
            println!("color: {}", color);
        }
    }

}

// function to read and parse json given local file path
// returns a flat vector of colors representing the board
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
    let pieces = load_pieces();

    println!("{:?}", board);
    println!("{:?}", pieces);
    Ok(())
}