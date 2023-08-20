use clap::Parser;
use kaleidoscope_puzzle::game::Kaleidoscope;
use kaleidoscope_puzzle::game::Solver;
use kaleidoscope_puzzle::game::{self, BitRepresentation, VectorRepresentation};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "classic")]
    board: String,
    #[arg(short, long, default_value = "boards/boards_1.json")]
    file: String,
    #[arg(short, long, default_value_t = 0)]
    repr: u8,
}

fn main() {
    // let x = game::BitRepresentation::new(Path::new("boards/boards_2.json"), "australian-emu");
    // let x = game::VectorRepresentation::new(Path::new("boards/boards_1.json"), "classic");

    let args = Args::parse();
    let strat = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

    match &args.repr {
        0 => {
            let x = game::BitRepresentation::new(Path::new(&args.file), &args.board);
            x.print_ref();
            let mut solver = game::solver::BacktrackingSolver::<BitRepresentation>::new(x);
            let solved = solver.solve(strat);
            print!("{}", solved);
            solver.print();
        }
        1 => {
            let x = game::VectorRepresentation::new(Path::new(&args.file), &args.board);
            x.print_ref();
            let mut solver = game::solver::BacktrackingSolver2::<VectorRepresentation>::new(x);
            let solved = solver.solve(strat);
            print!("{}", solved);
            solver.print();
        }
        _ => panic!("bad arg"),
    }
}
