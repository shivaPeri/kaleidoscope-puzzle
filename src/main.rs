

use std::path::Path;
use crate::game::Strategy;

mod game;


fn main() {

    // let x = game::Kaleidoscope::new(Path::new("boards/boards.json"), "classic");
	let mut x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "australian-emu");
	x.print_ref();

    // testing ****************************************
    // let test_idx = 17;
    // let test = x.possible(test_idx);
    // let thing = test[0];

    // // for thing in test.iter(){
    // //     println!("{} {}", thing[0], thing[1]);
    // //     x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    // // }

    // println!("");
    // println!("{}", test.len());
    // x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    // x.set(test_idx, test[0]);
    // x.print();

    // let test_idx = 16;
    // let test = x.possible(test_idx);
    // let thing = test[0];

    // // for thing in test.iter(){
    // //     println!("{} {}", thing[0], thing[1]);
    // //     x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    // // }

    // println!("");
    // println!("{}", test.len());
    // x.pieces[test_idx].print(thing[2], thing[1], thing[0]);
    // x.set(test_idx, test[0]);
    // x.print();

    // testing ****************************************

    let strategy: Strategy = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut solver = game::solver::Solver::new(x, strategy);

    if solver.solve() {
        println!("Solved!");
    } else {
        println!("No solution found.");
    }

    println!("{} moves", solver.moves);

}