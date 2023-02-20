

use std::path::Path;
use crate::game::Strategy;
use termion::{color::{self}};

mod game;

fn main() {

    // let x = game::Kaleidoscope::new(Path::new("boards/boards.json"), "classic");
	// let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "australian-emu");
    let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "riverbanks");
	x.print_ref();

    let strategy: Strategy = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut solver = game::solver::Solver::new(x, strategy);

    if solver.solve() {
        println!("\n{}Solved in {} moves", color::Fg(color::Reset), solver.moves);
        solver.game.print();
    } else {
        println!("{}No solution found.", color::Fg(color::Reset));
    }
}