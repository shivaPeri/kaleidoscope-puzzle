
use super::{Piece, Kaleidoscope, Move, Strategy};

// pub fn solve(board: &Board, pieces: &Vec<Piece>) {
//     let mut used = vec![false; pieces.len()];
    
// }

struct Solver {
    game: Kaleidoscope,
    strategy: Strategy,
    moves: u128,
}

impl Solver {
    pub fn new(game: Kaleidoscope, strategy: Strategy) -> Self {
        Self {
            game,
            strategy,
            moves: 0,
        }
    }

    pub fn solve(&mut self) -> bool {
        let mut piece_idx = 0;
        let mut moves: Vec<Move> = vec![];
        let mut used = vec![false; self.game.pieces.len()];
        let mut board = self.game.board.clone();
        let mut refboard = self.game.refboard.clone();

        // loop {
        //     if piece_idx == self.game.pieces.len() {
        //         return true;
        //     }

        //     let piece = &self.game.pieces[piece_idx];
        //     let possible = self.game.possible(piece_idx, &board, &refboard);

        //     if possible.len() == 0 {
        //         if piece_idx == 0 {
        //             return false;
        //         }
        //         piece_idx -= 1;
        //         let last_move = moves.pop().unwrap();
        //         used[last_move[2] as usize] = false;
        //         self.game.unplace_piece(&mut board, &mut refboard, &last_move);
        //         continue;
        //     }

        //     let mut move_idx = 0;
        //     let mut best_move = possible[0];
        //     let mut best_score = 0;
        //     for (i, m) in possible.iter().enumerate() {
        //         let score = self.strategy.score(&self.game, &board, &refboard, m);
        //         if score > best_score {
        //             best_score = score;
        //             best_move = *m;
        //             move_idx = i;
        //         }
        //     }

        //     self.game.place_piece(&mut board, &mut refboard, &best_move);
        //     moves.push(best_move);
        //     used[best_move[2] as usize] = true;
        //     piece_idx += 1;
        //     self.moves += 1;
        // }

        return true;
    }
}