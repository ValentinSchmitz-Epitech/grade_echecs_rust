//
// EPITECH PROJECT, 2026
// grade_echecs_rust
// File description:
// move_pieces
//
use crate::structs::Piece;
use crate::structs::Type;

fn check_move_pawn(piece: Piece, board: [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let delta_cords: [i32; 2] = [new_cords[0] as i32 - piece.cords[0] as i32,
                                new_cords[1]  as i32 - piece.cords[1] as i32];
    let mv_pos_w: [[i32; 2]; 4] = [[1, 0], [2, 0], [1, -1], [1, 1]];
    let mv_pos_b: [[i32; 2]; 4] = [[-1, 0], [-2, 0], [-1, -1], [-1, 1]];

    if delta_cords[0] == 0 {
        return false;
    }
    if (piece.type_p == Type::Wpawn) && (mv_pos_w.contains(&delta_cords)) {
        if board[new_cords[0]][new_cords[1]].type_p != Type::None &&
            (delta_cords == [1, -1] || delta_cords == [1, 1]) {
            return true;
        }
        if board[new_cords[0]][new_cords[1]].type_p == Type::None &&
            (delta_cords == [1, 0] ||
            (delta_cords == [2, 0] && !piece.moved &&
            board[new_cords[0] - 1][new_cords[1]].type_p == Type::None)) {
            return true;
        }
    }
    if (piece.type_p == Type::Bpawn) && (mv_pos_b.contains(&delta_cords)) {
        if board[new_cords[0]][new_cords[1]].type_p != Type::None &&
            (delta_cords == [-1, -1] || delta_cords == [-1, 1]) {
            return true;
        }
        if board[new_cords[0]][new_cords[1]].type_p == Type::None &&
            (delta_cords == [-1, 0] ||
            (delta_cords == [-2, 0] && !piece.moved &&
            board[new_cords[0] + 1][new_cords[1]].type_p == Type::None)) {
            return true;
        }
        return false;
    }
    return false;
}

pub fn mv_pawn(piece: Piece, board: &mut [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let rt: bool = check_move_pawn(piece, *board, new_cords);

    if rt {
        board[new_cords[0]][new_cords[1]].cords = piece.cords;
        if !piece.moved {
            board[new_cords[0]][new_cords[1]].moved = true;
        } else {
            board[new_cords[0]][new_cords[1]].moved = piece.moved;
        }
        board[new_cords[0]][new_cords[1]].type_p = piece.type_p;
        board[piece.cords[0]][piece.cords[1]] = Piece{cords: piece.cords, type_p: Type::None, moved: false};
        board[new_cords[0]][new_cords[1]].cords = new_cords;
        return true;
    }
    return false;
}