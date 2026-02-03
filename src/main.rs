//
// EPITECH PROJECT, 2026
// grade_echecs_rust
// File description:
// main
//
mod structs;
use crate::structs::Piece;
use crate::structs::Type;
mod init_board;
use crate::init_board::init_board;
mod disp_board;
use crate::disp_board::disp_board;

fn check_move() -> bool {
    return true;
}

fn mv_pawn(piece: Piece, mut board: [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    if check_move() != true {
        return false;
    }
    board[new_cords[0]][new_cords[1]].cords = piece.cords;
    board[new_cords[0]][new_cords[1]].moved = piece.moved;
    board[new_cords[0]][new_cords[1]].type_p = piece.type_p;
    board[piece.cords[0]][piece.cords[1]] = Piece{cords: piece.cords, type_p: Type::None, moved: false};
    board[new_cords[0]][new_cords[1]].cords = new_cords;
    return true;
}

fn main() {
    let board: [[Piece; 8]; 8] = init_board();

    disp_board(board);
    print!("\n\n");
    if mv_pawn(board[1][5], board, [2, 5]) == true {
        print!("moved!\n");
    }
    disp_board(board);
}