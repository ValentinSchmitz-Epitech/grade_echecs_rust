//
// EPITECH PROJECT, 2026
// grade_echecs_rust
// File description:
// move_pieces
//
use crate::structs::Piece;
use crate::structs::Type;

fn check_move() -> bool {
    return true;
}

pub fn mv_pawn(piece: Piece, board: &mut [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
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