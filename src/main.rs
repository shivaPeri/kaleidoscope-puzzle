use std::path::Path;

mod game;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let board = game::load_game(Path::new("boards/scraped-boards.json"), "australian-emu");
    let pieces = game::load_pieces();

    game::print_board(&board);

    // println!("{:?}", pieces);
    Ok(())
}