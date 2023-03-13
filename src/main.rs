use clap::Parser;
use std::path::Path;
use kaleidoscope_puzzle::game::{self, PlayOrder};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {

    // let x = game::Kaleidoscope::new(Path::new("boards/boards.json"), "sun-shower");
	// let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "australian-emu");

    let args = Args::parse();
    let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), &args.name);
	x.print_ref();

    let mut solver = game::solver::KaleidoscopeSolver::new(x);
    
    // backtracking strategy
    let order: PlayOrder = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut backtracking = game::solver::BacktrackingSolver::new(order);
    solver.solve(&mut backtracking);
    solver.print();
    
    // TODO: fix beam search solver
}