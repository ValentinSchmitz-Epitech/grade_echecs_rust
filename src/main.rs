//
// EPITECH PROJECT, 2026
// grade_echecs_rust
// File description:
// main
//
mod structs;
use crate::structs::Piece;
mod init_board;
use crate::init_board::init_board;
mod disp_board;
use crate::disp_board::disp_board;
mod move_pieces;
use crate::move_pieces::mv_pawn;

fn main() {
    let mut board: [[Piece; 8]; 8] = init_board();

    disp_board(board);
    if mv_pawn(board[1][3], &mut board, [3, 3]) == true {
        print!("\nmoved!\n\n");
        disp_board(board);
    }
    if mv_pawn(board[3][3], &mut board, [4, 3]) == true {
        print!("\nmoved!\n\n");
        disp_board(board);
    }
}