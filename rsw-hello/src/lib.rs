mod utils;

use std::str::FromStr;

use chess::{Board, ChessMove, MoveGen, Square};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn get_move(fen: String) -> String {
    let board = Board::from_str(&fen).unwrap();

    let m = find_min_max_best_move(board);

    let m_str = m.to_string();
    return m_str;
}

// minmax that will find the best move for the current player
fn find_min_max_best_move(board: Board) -> ChessMove {
    let current_color = board.side_to_move();
    let mut best_move = None;
    let mut best_score = match current_color {
        chess::Color::White => i32::MIN,
        chess::Color::Black => i32::MAX,
    };

    for m in MoveGen::new_legal(&board) {
        let m = m.to_owned();
        let mut board = board.make_move_new(m);
        let score = min_max(board, 3, false);
        match current_color {
            chess::Color::White => {
                if score > best_score {
                    best_score = score;
                    best_move = Some(m);
                }
            }
            chess::Color::Black => {
                if score < best_score {
                    best_score = score;
                    best_move = Some(m);
                }
            }
        }
    }

    return best_move.unwrap();
}

fn min_max(board: Board, depth: i32, is_maximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let current_color = board.side_to_move();
    let mut best_score = match current_color {
        chess::Color::White => i32::MIN,
        chess::Color::Black => i32::MAX,
    };

    for m in MoveGen::new_legal(&board) {
        let m = m.to_owned();
        let board = board.make_move_new(m);
        let score = min_max(board, depth - 1, !is_maximizing);
        match current_color {
            chess::Color::White => {
                if score > best_score {
                    best_score = score;
                }
            }
            chess::Color::Black => {
                if score < best_score {
                    best_score = score;
                }
            }
        }
    }

    return best_score;
}

fn evaluate(board: Board) -> i32 {
    let mut score = 0;
    for square in 0..64 {
        let piece = board.piece_on(unsafe { Square::new(square) });
        let color = board.color_on(unsafe { Square::new(square) });
        if piece.is_some() {
            let piece = piece.unwrap();
            let value = match piece {
                chess::Piece::Pawn => 1,
                chess::Piece::Knight => 3,
                chess::Piece::Bishop => 3,
                chess::Piece::Rook => 5,
                chess::Piece::Queen => 9,
                chess::Piece::King => 100,
            };
            score += match color.unwrap() {
                chess::Color::White => value,
                chess::Color::Black => -value,
            };
        }
    }
    return score;
}
