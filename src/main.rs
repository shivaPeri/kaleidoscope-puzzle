use clap::Parser;
use kaleidoscope_puzzle::game::{self, BitRepresentation};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    // let x = game::Kaleidoscope::new(Path::new("boards/boards.json"), "classic");
    // let x = game::Kaleidoscope::new(Path::new("boards/boards.json"), "sun-shower");
    // let x = game::BitRepresentation::new(Path::new("boards/boards_2.json"), "australian-emu");
    let x = game::BitRepresentation::new(Path::new("boards/boards_1.json"), "classic");

    // let args = Args::parse();
    // let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), &args.name);
    // x.print_ref();

    // let mut solver = game::solver::KaleidoscopeSolver::new(x);
    let solver = game::solver::BacktrackingSolver<BitRepresentation>::new(x);
    let solved = solver.solve([17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    print(solved)

    // backtracking strategy
    // let order = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    // let mut backtracking = game::solver::BacktrackingSolver::new(order);
    // let mut backtracking = game::solver::BacktrackingSolver2::new(order);
    // solver.solve(&mut backtracking);
    // solver.print();
}
