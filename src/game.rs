use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Color {
    EMPTY = 0,
    BLACK = 1,
    RED = 2,
    YELLOW = 3,
    BLUE = 4
}

pub enum Orientation {
    Orig,
    Rot90,
    Rot180,
    Rot270,
    Flip,
    Rot90Flip,
    Rot180Flip,
    Rot270Flip
}

pub struct Move {
    pub piece: usize,               // index of piece in pieces vector
    pub row: usize,                 // x coord of top left corner
    pub col: usize,                 // YELLOW coord of top left corner
    pub orientation: Orientation    // orientation of piece
}

#[derive(Serialize, Deserialize, Debug)]
struct Data(HashMap<String, String>);

pub type Board = Vec<Color>;
pub type Piece = Vec<u8>;

struct Piece2 {
    idx: u8,
    configs: Vec<Piece>
}

pub const BOARD_SIZE: usize = 8;

fn load_pieces_2() -> Vec<Piece2> {

    let mono_1 = Piece2{
        idx: 1,
        configs: vec![
            vec![1,1, RED],
            vec![1,1, BLACK],
        ];
    };

    let mono_2 = Piece2{
        idx: 2,
        configs: vec![
            vec![1,1, BLUE],
            vec![1,1, BLACK],
        ];
    };

    let domo_1 = Piece2{
        idx: 3,
        configs: vec![
            vec![1,2, RED, BLACK],
            vec![1,2, BLACK, RED],
            vec![1,2, BLUE, BLACK],
            vec![1,2, BLACK, BLUE],
            vec![2,1, RED, BLACK],
            vec![2,1, BLACK, RED],
            vec![2,1, BLUE, BLACK],
            vec![2,1, BLACK, BLUE],
        ];
    };

    let trom_1 = Piece2{
        idx: 4,
        configs: vec![
            vec![1,3, RED, BLACK, BLACK],
            vec![1,3, YELLOW, BLACK, BLUE],
            vec![1,3, BLUE, BLACK, YELLOW],
            vec![3,1, RED, BLACK, BLACK],
            vec![3,1, YELLOW, BLACK, BLUE],
            vec![3,1, BLUE, BLACK, YELLOW],
        ];
    };

    let trom_2 = Piece2{
        idx: 5,
        configs: vec![
            vec![1,3, BLACK, RED, BLACK],
            vec![1,3, BLACK, YELLOW, BLACK],
            vec![3,1, BLACK, RED, BLACK],
            vec![3,1, BLACK, YELLOW, BLACK],
        ];
    };

    let trom_3 = Piece2{
        idx: 6,
        configs: vec![
            vec![2,2, EMPTY, BLACK, RED, BLACK],
            vec![2,2, BLACK, EMPTY, RED, BLACK],
            vec![2,2, BLACK, RED, EMPTY, BLACK],
            vec![2,2, BLACK, RED, BLACK, EMPTY],
            vec![2,2, EMPTY, BLACK, YELLOW, BLACK],
            vec![2,2, BLACK, EMPTY, YELLOW, BLACK],
            vec![2,2, BLACK, YELLOW, EMPTY, BLACK],
            vec![2,2, BLACK, YELLOW, BLACK, EMPTY],
        ];
    };

    // this might be wrong
    let trom_4 = Piece2{
        idx: 7,
        configs: vec![
            vec![2,2, EMPTY, RED, BLACK, RED],
            vec![2,2, RED, EMPTY, BLACK, RED],
            vec![2,2, RED, BLACK, EMPTY, RED],
            vec![2,2, RED, BLACK, RED, EMPTY],
            vec![2,2, EMPTY, BLUE, YELLOW, BLACK],
            vec![2,2, BLUE, EMPTY, YELLOW, BLACK],
            vec![2,2, BLUE, YELLOW, EMPTY, BLACK],
            vec![2,2, BLUE, YELLOW, BLACK, EMPTY],
        ];
    };

    let tetr_1 = Piece2{
        idx: 8,
        configs: vec![
            vec![1,4, RED, BLACK, RED, BLACK],
            vec![1,4, BLACK, RED, BLACK, RED],
            vec![1,4, BLUE, BLACK, YELLOW, BLACK],
            vec![1,4, BLACK, YELLOW, BLACK, BLUE],
        ];
    };

    let tetr_2 = Piece2{
        idx: 9,
        configs: vec![
            vec![2,2, RED, BLACK, RED, BLACK],
            vec![2,2, BLACK, RED, BLACK, RED],
            vec![2,2, BLUE, BLACK, BLACK, YELLOW],
            vec![2,2, YELLOW, BLACK, BLACK, BLUE],
            vec![2,2, BLACK, YELLOW, BLUE, BLACK],
            vec![2,2, BLACK, BLUE, YELLOW, BLACK],
        ];
    };

    let tetr_3 = Piece2{
        idx: 10,
        configs: vec![
            vec![2,3, EMPTY, EMPTY, RED, BLACK, RED, BLACK],
            vec![3,2, RED, BLACK, EMPTY, RED, EMPTY, BLACK],
            vec![2,3, BLACK, RED, BLACK, RED, EMPTY, EMPTY],
            vec![3,2, BLACK, EMPTY, RED, EMPTY, BLACK, RED],
            vec![2,3, BLACK, EMPTY, EMPTY, YELLOW, BLACK, BLUE],
            vec![3,2, YELLOW, BLACK, BLACK, EMPTY, BLUE, EMPTY],
            vec![2,3, BLUE, BLACK, YELLOW, EMPTY, EMPTY, BLACK],
            vec![3,2, EMPTY, BLUE, EMPTY, BLACK, BLACK, YELLOW],
        ];
    };

    let tetr_4 = Piece2{
        idx: 11,
        configs: vec![
            vec![2,3, RED, EMPTY, EMPTY, BLACK, RED, BLACK],
            vec![3,2, BLACK, RED, RED, EMPTY, BLACK, EMPTY],
            vec![2,3, BLACK, RED, BLACK, EMPTY, EMPTY, RED],
            vec![3,2, EMPTY, BLACK, EMPTY, RED, RED, BLACK],
            vec![2,3, EMPTY, EMPTY, YELLOW, BLACK, BLUE, BLACK],
            vec![3,2, YELLOW, BLACK, EMPTY, BLUE, EMPTY, BLACK],
            vec![2,3, BLACK, BLUE, BLACK, YELLOW, EMPTY, EMPTY],
            vec![3,2, BLACK, EMPTY, BLUE, EMPTY, BLACK, YELLOW],
        ];
    };

    let tetr_5 = Piece2{
        idx: 12,
        configs: vec![
            vec![2,3, EMPTY, EMPTY, BLACK, RED, BLACK, RED],
            vec![3,2, BLACK, RED, EMPTY, BLACK, EMPTY, RED],
            vec![2,3, RED, BLACK, RED, BLACK, EMPTY, EMPTY],
            vec![3,2, RED, EMPTY, BLACK, EMPTY, RED, BLACK],
            vec![2,3, BLACK, EMPTY, EMPTY, BLUE, BLACK, YELLOW],
            vec![3,2, BLUE, BLACK, BLACK, EMPTY, YELLOW, EMPTY],
            vec![2,3, YELLOW, BLACK, BLUE, EMPTY, EMPTY, BLACK],
            vec![3,2, EMPTY, YELLOW, EMPTY, BLACK, BLACK, BLUE],
        ];
    };


    let tetr_6 = Piece2{
        idx: 13,
        configs: vec![
            vec![2,3, BLACK, EMPTY, EMPTY, RED, BLACK, RED],
            vec![3,2, RED, BLACK, BLACK, EMPTY, RED, EMPTY],
            vec![2,3, RED, BLACK, RED, EMPTY, EMPTY, BLACK],
            vec![3,2, EMPTY, RED, EMPTY, BLACK, BLACK, RED],
            vec![2,3, EMPTY, EMPTY, BLUE, BLACK, YELLOW, BLACK],
            vec![3,2, BLUE, BLACK, EMPTY, YELLOW, EMPTY, BLACK],
            vec![2,3, BLACK, YELLOW, BLACK, BLUE, EMPTY, EMPTY],
            vec![3,2, BLACK, EMPTY, YELLOW, EMPTY, BLACK, BLUE],
        ];
    };

    let tetr_7 = Piece2{
        idx: 14,
        configs: vec![
            vec![2,3, EMPTY, BLACK, EMPTY, BLACK, RED, BLACK],
            vec![3,2, BLACK, EMPTY, RED, BLACK, BLACK, EMPTY],
            vec![2,3, BLACK, RED, BLACK, EMPTY, BLACK, EMPTY],
            vec![3,2, EMPTY, BLACK, BLACK, RED, EMPTY, BLACK],
            vec![2,3, EMPTY, YELLOW, EMPTY, BLUE, BLACK, YELLOW],
            vec![3,2, BLUE, EMPTY, BLACK, YELLOW, YELLOW, EMPTY],
            vec![2,3, YELLOW, BLACK, BLUE, EMPTY, YELLOW, EMPTY],
            vec![3,2, EMPTY, YELLOW, YELLOW, BLACK, EMPTY, BLUE],
        ];
    };

    let tetr_8 = Piece2{
        idx: 15,
        configs: vec![
            vec![2,3, EMPTY, RED, EMPTY, RED, BLACK, RED],
            vec![3,2, RED, EMPTY, BLACK, RED, RED, EMPTY],
            vec![2,3, RED, BLACK, RED, EMPTY, RED, EMPTY],
            vec![3,2, EMPTY, RED, RED, BLACK, EMPTY, RED],
            vec![2,3, EMPTY, BLACK, EMPTY, BLACK, BLUE, BLACK],
            vec![3,2, BLACK, EMPTY, BLUE, BLACK, BLACK, EMPTY],
            vec![2,3, BLACK, BLUE, BLACK, EMPTY, BLACK, EMPTY],
            vec![3,2, EMPTY, BLACK, BLACK, BLUE, EMPTY, BLACK],
        ];
    };

    let tetr_9 = Piece2{
        idx: 16,
        configs: vec![
            vec![2,3, EMPTY, BLACK, RED, BLACK, RED, EMPTY],
            vec![3,2, RED, EMPTY, BLACK, RED,EMPTY,BLACK],
            vec![2,3, EMPTY,RED,BLACK,RED,BLACK,EMPTY],
            vec![3,2, BLACK,EMPTY,RED,BLACK,EMPTY,RED],
            vec![2,3, BLACK,BLUE,EMPTY,EMPTY,BLACK,YELLOW],
            vec![3,2, EMPTY,BLACK,BLACK,BLUE,YELLOW,EMPTY],
            vec![2,3, YELLOW,BLACK,EMPTY,EMPTY,BLUE,BLACK],
            vec![3,2, EMPTY,YELLOW,BLUE,BLACK,BLACK,EMPTY],
        ];
    };

    let tetr_10 = Piece2{
        idx: 17,
        configs: vec![
            vec![2,3, BLACK,RED,EMPTY,EMPTY,BLACK,RED],
            vec![3,2, EMPTY,BLACK,BLACK,RED,RED,EMPTY],
            vec![2,3, RED,BLACK,EMPTY,EMPTY,RED,BLACK],
            vec![3,2, EMPTY,RED,RED,BLACK,BLACK,EMPTY],
            vec![2,3, EMPTY,YELLOW,BLACK,BLUE,BLACK,EMPTY],
            vec![3,2, BLUE,EMPTY,BLACK,YELLOW,EMPTY,BLACK],
            vec![2,3, EMPTY,BLACK,BLUE,BLACK,YELLOW,EMPTY],
            vec![3,2, BLACK,EMPTY,YELLOW,BLACK,EMPTY,BLUE],
        ];
    };

    let oct_1 = Piece2{
        idx: 18,
        configs: vec![
            vec![1,8, BLACK, RED, BLACK, RED, BLACK, RED, BLACK, RED],
            vec![1,8, RED, BLACK, RED, BLACK, RED, BLACK, RED, BLACK],
            vec![8,1, BLACK, RED, BLACK, RED, BLACK, RED, BLACK, RED],
            vec![8,1, RED, BLACK, RED, BLACK, RED, BLACK, RED, BLACK],
            vec![1,8, BLACK, YELLOW, BLACK, BLUE, BLACK, YELLOW, BLACK, BLUE],
            vec![1,8, BLUE, BLACK, YELLOW, BLACK, BLUE, BLACK, YELLOW, BLACK],
            vec![8,1, BLACK, YELLOW, BLACK, BLUE, BLACK, YELLOW, BLACK, BLUE],
            vec![8,1, BLUE, BLACK, YELLOW, BLACK, BLUE, BLACK, YELLOW, BLACK],
        ];
    };

    let pieces: Vec<Piece2> = vec![mono_1, mono_2, domi, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
    return pieces;
}

// makes vector of all the pieces
// first three vals are dimensions of piece, then colors
pub fn load_pieces() -> Vec<Piece> {
    
    // Monominoes
    let mono_1: Vec<u8> = vec![1,1,2, 2,1];
    let mono_2: Vec<u8> = vec![1,1,2, 1,4];
    
    // Dominoes
    let domi: Vec<u8> = vec![1,2,2, 2,1,1,4];
    
    // Trominoes
    let trom_1: Vec<u8> = vec![1,3,2, 2,3,1,1,2,4];
    let trom_2: Vec<u8> = vec![1,3,2, 1,1,2,3,1,1];
    let trom_3: Vec<u8> = vec![2,2,2, 2,4,0,0,1,1,2,3];
    let trom_4: Vec<u8> = vec![2,2,2, 1,1,0,0,2,3,1,1];
    
    // Tetrominoes
    let tetr_1: Vec<u8> = vec![1,4,2, 2,4,1,1,2,3,1,1];
    let tetr_2: Vec<u8> = vec![2,2,2, 2,1,1,4,1,3,2,1];
    let tetr_3: Vec<u8> = vec![2,3,2, 1,1,2,3,0,0,0,0,1,1,2,4];
    let tetr_4: Vec<u8> = vec![2,3,2, 0,0,2,1,1,3,2,1,1,4,0,0];
    let tetr_5: Vec<u8> = vec![2,3,2, 0,0,2,1,0,0,2,1,1,4,2,1];
    let tetr_6: Vec<u8> = vec![2,3,2, 0,0,1,3,0,0,1,3,2,1,1,4];
    let tetr_7: Vec<u8> = vec![2,3,2, 1,4,0,0,0,0,2,1,1,3,2,1];
    let tetr_8: Vec<u8> = vec![2,3,2, 2,3,0,0,0,0,1,1,2,4,1,1];
    let tetr_9: Vec<u8> = vec![2,3,2, 0,0,0,0,1,1,2,3,1,1,2,4];
    let tetr_10: Vec<u8> = vec![2,3,2, 0,0,0,0,2,1,1,4,2,1,1,3];
    
    // Octominoes
    let oct_1: Vec<u8> = vec![1,8,2, 2,1,1,3,2,1,1,4,2,1,1,3,2,1,1,4];
    
    let pieces: Vec<Piece> = vec![mono_1, mono_2, domi, trom_1, trom_2, trom_3, trom_4, tetr_1, tetr_2, tetr_3, tetr_4, tetr_5, tetr_6, tetr_7, tetr_8, tetr_9, tetr_10, oct_1];
    return pieces;
}

// TODO: need to only check if non-empty squares are filled
fn in_bounds(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) -> bool {

    let piece = &pieces[turn.piece];

    let mut d1 = piece[0];
    let mut d2 = piece[1];
    
    if (turn.orientation == Orientation::Rot90 || 
        turn.orientation == Orientation::Rot270 ||
        turn.orientation == Orientation::Rot90Flip ||
        turn.orientation == Orientation::Rot270Flip) {
        d1 = piece[1];
        d2 = piece[0];
    }

    if (turn.row + d1 as usize > BOARD_SIZE || turn.col + d2 as usize > BOARD_SIZE) {
        return false;
    }

    return true;
}

pub fn place_piece(board: &mut Board, pieces: &Vec<Piece>, turn: &Move) {

    if !in_bounds(board, pieces, turn) {
        return;
    }

    let piece = &pieces[turn.piece];
    let dims: &[u8] = &piece[..3];
    let piece_colors: &[u8] = &piece[3..];

    match turn.orientation {
        Orientation::Orig => {
            // do nothing

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
        Orientation::Rot90 => {
            // rotate 90 degrees
            for i in 0..dims[1] {
                for j in 0..dims[0] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [j, dims[1]-i-1, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[1] {
                for j in 0..dims[0] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [j, dims[1]-i-1, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
        Orientation::Rot180 => {
            // rotate 180 degrees
            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [dims[0]-i-1, dims[0]-j-1, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [dims[0]-i-1, dims[0]-j-1, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
        Orientation::Rot270 => {
            // rotate 270 degrees
            for i in 0..dims[1] {
                for j in 0..dims[0] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
        Orientation::Flip => {
            // flip vertically
            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
        Orientation::Rot90Flip => {
            // rotate 90 degrees, then flip vertically
            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
        Orientation::Rot180Flip => {
            // rotate 180 degrees, then flip vertically
            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
        Orientation::Rot270Flip => {
            // rotate 270 degrees, then flip vertically
            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    
                    // cannot place piece
                    if piece_colors[piece_idx] != 0 and board[board_idx] != Colors.EMPTY {
                        return;
                    }
                }
            }

            for i in 0..dims[0] {
                for j in 0..dims[1] {
                    let board_idx = (turn.row + i as usize) * BOARD_SIZE + turn.col + j as usize;
                    let piece_coord = [i, j, 0];
                    let piece_idx = piece_coord[0] * dims[1] as usize + piece_coord[1] * dims[2] as usize + piece_coord[2] as usize;
                    board[board_idx] = piece_colors[piece_idx]
                }
            }
        },
    }

    // this is the actual piece placement
    // TODO: double check this
    for i in 0..d1 {
        for j in 0..d2 {
            let mut index = turn.row * BOARD_SIZE + turn.col;
            if d1_flip {
                index += (d1 - i - 1) * BOARD_SIZE;
            } else {
                index += i * BOARD_SIZE;
            }
            if d2_flip {
                index += d2 - j - 1;
            } else {
                index += j;
            }

            let piece_idx = i * d2 + j + d3;
            board[index] = piece_colors[piece_idx as usize];
        }
    }
}

// function to read and parse json given local file path
// returns a flat vector of colors representing the board
pub fn load_game(path: &Path, game: &str) -> Board {
    let file = fs::read_to_string(path).expect("Unable to read file");
    let data: Data = serde_json::from_str(&file).expect("Unable to parse json");
    let game_str = data.0.get(game).unwrap();

    // take game_str and create a vector mapping each character to corresponding color
    let mut board = Board::new();
    for c in game_str.chars() {
        let color = match c {
            '1' => Color::BLACK,
            '2' => Color::RED,
            '3' => Color::YELLOW,
            '4' => Color::BLUE,
            _ => Color::EMPTY,
        };
        board.push(color);
    }

    return board;
}