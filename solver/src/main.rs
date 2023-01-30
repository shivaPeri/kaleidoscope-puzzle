use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use ndarray::{ArrayBase, OwnedRepr, Dim};
use ndarray::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    EMPTY = 0,
    BLACK = 1,
    RED = 2,
    YELLOW = 3,
    BLUE = 4
}


#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

type Board = Vec<Color>;
type Piece = ArrayBase<OwnedRepr<Color>, Dim<[usize; 3]>>;

const BOARD_SIZE: usize = 8;

// creates a vector of all initial pieces
fn make_pieces() -> Vec<Piece> {

    // Monominoes
    let mono_1 = array![[[Color::RED, Color::BLACK]]];
    let mono_2 = array![[[Color::BLACK, Color::BLUE]]];

    // Dominoes
    let domi = array![[[Color::RED, Color::BLACK], [Color::BLACK, Color::BLUE]]];
    
    // Trominoes
    let trom_1 = array![[[Color::RED, Color::YELLOW], [Color::BLACK, Color::BLACK], [Color::RED, Color::BLUE]]];
    let trom_2 = array![[[Color::BLACK, Color::BLACK], [Color::RED, Color::YELLOW], [Color::BLACK, Color::BLACK]]];
    let trom_3 = array![[[Color::RED, Color::BLUE], [Color::EMPTY, Color::EMPTY]], [[Color::BLACK, Color::BLACK], [Color::RED, Color::YELLOW]]];
    let trom_4 = array![[[Color::BLACK, Color::BLACK], [Color::EMPTY, Color::EMPTY]], [[Color::RED, Color::YELLOW], [Color::BLACK, Color::BLACK]]];
    
    // Tetrominoes
    let tetr_1 = array![[[Color::RED, Color::BLUE], [Color::BLACK, Color::BLACK], [Color::RED, Color::YELLOW], [Color::BLACK, Color::BLACK]]];
    let tetr_2 = array![[[Color::RED, Color::BLACK], [Color::BLACK, Color::BLUE]], [[Color::BLACK, Color::YELLOW], [Color::RED, Color::BLACK]]];
    let tetr_3 = array![[[Color::BLACK, Color::BLACK], [Color::RED, Color::YELLOW], [Color::EMPTY, Color::EMPTY]], [[Color::EMPTY, Color::EMPTY], [Color::BLACK, Color::BLACK], [Color::RED, Color::BLUE]]];
    let tetr_4 = array![[[Color::EMPTY, Color::EMPTY], [Color::RED, Color::BLACK], [Color::BLACK, Color::YELLOW]], [[Color::RED, Color::BLACK], [Color::BLACK, Color::BLUE], [Color::EMPTY, Color::EMPTY]]];
    let tetr_5 = array![[[Color::EMPTY, Color::EMPTY], [Color::RED, Color::BLACK], [Color::EMPTY, Color::EMPTY]], [[Color::RED, Color::BLACK], [Color::BLACK, Color::BLUE], [Color::RED, Color::BLACK]]];
    let tetr_6 = array![[[Color::EMPTY, Color::EMPTY], [Color::BLACK, Color::YELLOW], [Color::EMPTY, Color::EMPTY]], [[Color::BLACK, Color::YELLOW], [Color::RED, Color::BLACK], [Color::BLACK, Color::BLUE]]];
    let tetr_7 = array![[[Color::BLACK, Color::BLUE], [Color::EMPTY, Color::EMPTY], [Color::EMPTY, Color::EMPTY]], [[Color::RED, Color::BLACK], [Color::BLACK, Color::YELLOW], [Color::RED, Color::BLACK]]];
    let tetr_8 = array![[[Color::RED, Color::YELLOW], [Color::EMPTY, Color::EMPTY], [Color::EMPTY, Color::EMPTY]], [[Color::BLACK, Color::BLACK], [Color::RED, Color::BLUE], [Color::BLACK, Color::BLACK]]];
    let tetr_9 = array![[[Color::EMPTY, Color::EMPTY], [Color::EMPTY, Color::EMPTY], [Color::BLACK, Color::BLACK]], [[Color::RED, Color::YELLOW], [Color::BLACK, Color::BLACK], [Color::RED, Color::BLUE]]];
    let tetr_10 = array![[[Color::EMPTY, Color::EMPTY], [Color::EMPTY, Color::EMPTY], [Color::RED, Color::BLACK]], [[Color::BLACK, Color::BLUE], [Color::RED, Color::BLACK], [Color::BLACK, Color::YELLOW]]];

    // Octominoes
    let oct_1 = array![[[Color::RED, Color::BLACK], [Color::BLACK, Color::YELLOW], [Color::RED, Color::BLACK], [Color::BLACK, Color::BLUE], [Color::RED, Color::BLACK], [Color::BLACK, Color::YELLOW], [Color::RED, Color::BLACK], [Color::BLACK, Color::BLUE]]];

    let pieces = vec![mono_1, mono_2, domi, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
    return pieces;
}

// function to read and parse json given local file path
fn load_game(path: &Path, game: &str) {
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

    println!("{:?}", board);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    load_game(Path::new("../boards/scraped-boards.json"), "australian-emu");
    Ok(())
}