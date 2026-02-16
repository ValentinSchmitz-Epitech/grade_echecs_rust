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


fn check_move_rook(piece: Piece, board: [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let delta_cords: [i32; 2] = [
        new_cords[0] as i32 - piece.cords[0] as i32,
        new_cords[1] as i32 - piece.cords[1] as i32
    ];
    if piece.type_p == Type::Wrook || piece.type_p == Type::Brook {
        if delta_cords[0] != 0 && delta_cords[1] != 0 {
            return false;
        }
        if delta_cords[0] == 0 && delta_cords[1] == 0 {
            return false;
        }
        let step_x = if delta_cords[0] != 0 { delta_cords[0].signum() } else { 0 };
        let step_y = if delta_cords[1] != 0 { delta_cords[1].signum() } else { 0 };
        
        let mut check_x = piece.cords[0] as i32 + step_x;
        let mut check_y = piece.cords[1] as i32 + step_y;
        
        while check_x != new_cords[0] as i32 || check_y != new_cords[1] as i32 {
            if board[check_x as usize][check_y as usize].type_p != Type::None {
                return false;
            }
            check_x += step_x;
            check_y += step_y;
        }
        let target = board[new_cords[0]][new_cords[1]].type_p;
        if piece.type_p == Type::Wrook && matches!(target, Type::Wpawn | Type::Wrook | Type::Wknight | Type::Wbishop | Type::Wqueen | Type::Wking) {
            return false;
        }
        if piece.type_p == Type::Brook && matches!(target, Type::Bpawn | Type::Brook | Type::Bknight | Type::Bbishop | Type::Bqueen | Type::Bking) {
            return false;
        }
        
        return true;
    } 
    return false;
}

pub fn mv_rook(piece: Piece, board: &mut [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let rt: bool = check_move_rook(piece, *board, new_cords);
    if rt {
        board[new_cords[0]][new_cords[1]].cords = piece.cords;
        board[new_cords[0]][new_cords[1]].moved = true;
        board[new_cords[0]][new_cords[1]].type_p = piece.type_p;
        board[piece.cords[0]][piece.cords[1]] = Piece{cords: piece.cords, type_p: Type::None, moved: false};
        board[new_cords[0]][new_cords[1]].cords = new_cords;
        return true;
    }
    return false;
}



fn check_move_knight(piece: Piece, board: [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let delta_cords: [i32; 2] = [
        new_cords[0] as i32 - piece.cords[0] as i32,
        new_cords[1] as i32 - piece.cords[1] as i32
    ];
    
    let mv_pos: [[i32; 2]; 8] = [
        [2, 1], [2, -1], [-2, 1], [-2, -1],
        [1, 2], [1, -2], [-1, 2], [-1, -2]
    ];

    if piece.type_p == Type::Wknight || piece.type_p == Type::Bknight {
        if !mv_pos.contains(&delta_cords) {
            return false;
        }
        let target = board[new_cords[0]][new_cords[1]].type_p;
        if piece.type_p == Type::Wknight && matches!(target, Type::Wpawn | Type::Wrook | Type::Wknight | Type::Wbishop | Type::Wqueen | Type::Wking) {
            return false;
        }
        if piece.type_p == Type::Bknight && matches!(target, Type::Bpawn | Type::Brook | Type::Bknight | Type::Bbishop | Type::Bqueen | Type::Bking) {
            return false;
        }
        return true;
    }
    return false;
}

pub fn mv_knight(piece: Piece, board: &mut [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let rt: bool = check_move_knight(piece, *board, new_cords);
    if rt {
        board[new_cords[0]][new_cords[1]].cords = piece.cords;
        board[new_cords[0]][new_cords[1]].moved = true;
        board[new_cords[0]][new_cords[1]].type_p = piece.type_p;
        board[piece.cords[0]][piece.cords[1]] = Piece{cords: piece.cords, type_p: Type::None, moved: false};
        board[new_cords[0]][new_cords[1]].cords = new_cords;
        return true;
    }
    return false;
}





fn check_move_bishop(piece: Piece, board: [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let delta_cords: [i32; 2] = [
        new_cords[0] as i32 - piece.cords[0] as i32,
        new_cords[1] as i32 - piece.cords[1] as i32
    ];

    if delta_cords[0].abs() != delta_cords[1].abs() || delta_cords[0] == 0 {
        return false;
    }

    if piece.type_p == Type::Bbishop || piece.type_p == Type::Wbishop {

        let step_x = delta_cords[0].signum();
        let step_y = delta_cords[1].signum();
        
        let mut check_x = piece.cords[0] as i32 + step_x;
        let mut check_y = piece.cords[1] as i32 + step_y;
        
        while check_x != new_cords[0] as i32 || check_y != new_cords[1] as i32 {
            if board[check_x as usize][check_y as usize].type_p != Type::None {
                return false;
            }
            check_x += step_x;
            check_y += step_y;
        }
        let target = board[new_cords[0]][new_cords[1]].type_p;
        if piece.type_p == Type::Wbishop && matches!(target, Type::Wpawn | Type::Wrook | Type::Wknight | Type::Wbishop | Type::Wqueen | Type::Wking) {
            return false;
        }
        if piece.type_p == Type::Bbishop && matches!(target, Type::Bpawn | Type::Brook | Type::Bknight | Type::Bbishop | Type::Bqueen | Type::Bking) {
            return false;
        }
        return true;
    }
    return false;
}

pub fn mv_bishop(piece: Piece, board: &mut [[Piece; 8]; 8], new_cords: [usize; 2]) -> bool {
    let rt: bool = check_move_bishop(piece, *board, new_cords);
    if rt {
        board[new_cords[0]][new_cords[1]].cords = piece.cords;
        board[new_cords[0]][new_cords[1]].moved = true;
        board[new_cords[0]][new_cords[1]].type_p = piece.type_p;
        board[piece.cords[0]][piece.cords[1]] = Piece{cords: piece.cords, type_p: Type::None, moved: false};
        board[new_cords[0]][new_cords[1]].cords = new_cords;
        return true;
    }
    return false;
}