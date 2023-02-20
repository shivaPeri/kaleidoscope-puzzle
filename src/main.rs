

use std::path::Path;
use crate::game::Strategy;
use termion::{color::{self}};

mod game;


fn main() {

    let mut x = game::Kaleidoscope::new(Path::new("boards/boards.json"), "classic");
	// let mut x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "australian-emu");
    // let mut x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "fancy-pinwheel");
	x.print_ref();

    // testing ****************************************
    // for australian emu board

    // let test = [[17,0], [16,5], [15,1],  [14,0], [13,0], [12,1], [11,0]];
    
    // for i in test.iter(){
    //     let test_idx = i[0];
    //     let test = x.possible(test_idx);
    //     let thing = test[i[1]];
    //     println!("");
    //     println!("{}", test.len());
    //     x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    //     x.set(test_idx, thing);
    //     x.print();
    // }

    // let test_idx = 10;
    // let test = x.possible(test_idx);

    // for thing in test.iter(){
    //     println!("{} {}", thing[0], thing[1]);
    //     x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    // }

    // testing ****************************************

    let strategy: Strategy = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut solver = game::solver::Solver::new(x, strategy);

    if solver.solve() {
        println!("\n{}Solved!", color::Fg(color::Reset));
        solver.game.print();
    } else {
        println!("{}No solution found.", color::Fg(color::Reset));
    }

    // println!("{} moves", solver.moves);

}