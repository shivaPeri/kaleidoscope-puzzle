use clap::Parser;
use std::path::Path;
use termion::color;
use kaleidoscope_puzzle::game::{self, Strategy};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {

    // let x = game::Kaleidoscope::new(Path::new("boards/boards.json"), "chaos");
	// let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), "australian-emu");

    let args = Args::parse();
    let x = game::Kaleidoscope::new(Path::new("boards/scraped-boards.json"), &args.name);
	x.print_ref();

    let strategy: Strategy = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut solver = game::solver::BacktrackingSolver::new(x, strategy);

    if solver.solve() {
        println!("\n{}Solved in {} moves ({}s)", color::Fg(color::Reset), solver.moves, solver.time.unwrap().as_secs_f64());
        solver.game.print();
    } else {
        println!("{}No solution found.", color::Fg(color::Reset));
    }
}