use super::{Kaleidoscope, Move};
use std::collections::VecDeque;

pub struct Generator {
    pub game: Kaleidoscope,
    pub moves: u128,
    possible: VecDeque<VecDeque<Move>>,   // possible moves for each piece
    debug: bool
}