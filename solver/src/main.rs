use std::fs;
use std::env;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    EMPTY = -1,
    BLACK = 0,
    RED = 1,
    YELLOW = 2,
    BLUE = 3
}


#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

#[derive(Default, Debug)]
struct Board (Vec<Color>);

// function to read and parse json given local file path
fn load_game(path: &Path, game: &str) {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let data: Data = serde_json::from_str(&file).expect("Unable to parse json");
    let game_str = data.0.get(game).unwrap();

    // take game_str and create a vector mapping each character to the corresponding parsed integer
    let mut board = Board::default();
    for c in game_str.chars() {
        let color = match c {
            '0' => Color::BLACK,
            '1' => Color::RED,
            '2' => Color::YELLOW,
            '3' => Color::BLUE,
            _ => Color::EMPTY
        };
        board.0.push(color);
    }

    println!("{:?}", board);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    load_game(Path::new("../boards/scraped-boards.json"), "australian-emu");
    Ok(())
}