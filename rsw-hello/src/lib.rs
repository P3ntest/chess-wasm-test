mod utils;

use std::str::FromStr;

use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};
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
pub fn get_eval(fen: String) -> i32 {
    let board = Board::from_str(&fen).unwrap();
    return evaluate(board);
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
        let board = board.make_move_new(m);
        let is_maximizing = match current_color {
            chess::Color::White => true,
            chess::Color::Black => false,
        };
        let score = min_max(board, 3, i32::MIN, i32::MAX, is_maximizing);
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

fn min_max(board: Board, depth: i32, alpha: i32, beta: i32, is_maximizing: bool) -> i32 {
    if depth == 0 {
        return evaluate(board);
    }

    let mut alpha = alpha;
    let mut beta = beta;

    if is_maximizing {
        let mut max_eval = i32::MIN;
        for m in MoveGen::new_legal(&board) {
            let m = m.to_owned();
            let board = board.make_move_new(m);
            let eval = min_max(board, depth - 1, alpha, beta, false);
            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    } else {
        let mut min_eval = i32::MAX;
        for m in MoveGen::new_legal(&board) {
            let m = m.to_owned();
            let board = board.make_move_new(m);
            let eval = min_max(board, depth - 1, alpha, beta, true);
            min_eval = min_eval.min(eval);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        return min_eval;
    }
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
            let mut piece_score = value * 10;
            piece_score +=
                piece_position_weight(piece, unsafe { Square::new(square) }, color.unwrap());
            score += match color.unwrap() {
                chess::Color::White => piece_score,
                chess::Color::Black => -piece_score,
            };
        }
    }
    return score;
}

fn piece_position_weight(piece: Piece, square: Square, color: Color) -> i32 {
    let rank = square.get_rank().to_index() as i32;
    let file = square.get_file().to_index() as i32;
    if piece == Piece::Pawn {
        let rank_value = match color {
            chess::Color::White => rank,
            chess::Color::Black => 7 - rank,
        };
        let file_value = score_distance_from_center(rank, file);
        return rank_value * file_value;
    } else if piece == Piece::King {
        let rank_value = match color {
            chess::Color::White => rank,
            chess::Color::Black => 7 - rank,
        };
        let file_value = distance_from_center_single(file);
        return -rank_value + file_value;
    } else if piece == Piece::Knight {
        return -distance_from_center(rank, file);
    } else if piece == Piece::Bishop {
        return score_distance_from_center(rank, file);
    } else if piece == Piece::Rook {
        return score_distance_from_center(rank, file);
    } else if piece == Piece::Queen {
        return score_distance_from_center(rank, file);
    }

    return 0;
}

fn distance_from_center_single(n: i32) -> i32 {
    match n {
        3 | 4 => 0,
        _ if n < 3 => 3 - n,
        _ => n - 4,
    }
}

fn distance_from_center(rank: i32, file: i32) -> i32 {
    return distance_from_center_single(rank) + distance_from_center_single(file);
}

fn score_distance_from_center(rank: i32, file: i32) -> i32 {
    let distance = distance_from_center(rank, file);
    return 10 - distance;
}
