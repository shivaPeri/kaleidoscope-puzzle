use game::{Piece, PieceConfig, Strategy};
use termion::{color::{self, Color}, style};

use std::io;
use std::path::Path;

mod game;


fn main() {

    // testing termion
    println!("{}Red", color::Fg(color::Red));
    println!("{}Blue", color::Fg(color::Blue));
    println!("{}Green", color::Fg(color::Green));
    // println!("{}Just plain italic", style::Italic);

    println!("{}Red", color::Fg(color::Red));

    // let game_str = game::load_game_str(Path::new("boards/boards.json"), "classic");
	let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "australian-emu");
	x.print_ref();

    let test_idx = 10;
    let test = x.possible(test_idx);

    for thing in test.iter(){
        println!("{} {}", thing[0], thing[1]);
        x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    }

    println!("");
    println!("{}", test.len());
}