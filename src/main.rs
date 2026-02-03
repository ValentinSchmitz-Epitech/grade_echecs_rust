//
// EPITECH PROJECT, 2026
// grade_echecs_rust
// File description:
// main
//

#[derive(Copy,Clone, Debug, PartialEq, Eq)]
enum Type {
    Wpawn,
    Wrook,
    Wknight,
    Wbishop,
    Wqueen,
    Wking,
    Bpawn,
    Brook,
    Bknight,
    Bbishop,
    Bqueen,
    Bking,
    None,
}

#[derive(Copy,Clone, Debug)]
struct Piece {
    cords: [usize; 2],
    type_p: Type,
    moved: bool,
}

fn disp_board(board: [[Piece; 8]; 8]) {
    for i in 0..8 {
        for j in 0..8 {
            if board[i][j].type_p == Type::Bpawn || board[i][j].type_p == Type::Wpawn {
                print!("P");
            }
            if board[i][j].type_p == Type::Brook || board[i][j].type_p == Type::Wrook {
                print!("R");
            }
            if board[i][j].type_p == Type::Bknight || board[i][j].type_p == Type::Wknight {
                print!("H");
            }
            if board[i][j].type_p == Type::Bbishop || board[i][j].type_p == Type::Wbishop {
                print!("B");
            }
            if board[i][j].type_p == Type::Bqueen || board[i][j].type_p == Type::Wqueen {
                print!("Q");
            }
            if board[i][j].type_p == Type::Bking || board[i][j].type_p == Type::Wking {
                print!("K");
            }
            if board[i][j].type_p == Type::None {
                print!(" ");
            }
            print!(" ");
        }
        print!("\n");
    }
}

fn init_board() -> [[Piece; 8]; 8] {
    let mut board: [[Piece; 8]; 8] = [[Piece{
        cords: [0, 0], type_p: Type::None, moved: false}; 8]; 8];

    for i in 0..8 {
        for j in 0..8 {
            board[i][j].cords = [i, j];
        }
    }
    board[0][0].type_p = Type::Wrook;
    board[0][7].type_p = Type::Wrook;
    board[0][1].type_p = Type::Wknight;
    board[0][6].type_p = Type::Wknight;
    board[0][2].type_p = Type::Wbishop;
    board[0][5].type_p = Type::Wbishop;
    board[0][4].type_p = Type::Wqueen;
    board[0][3].type_p = Type::Wking;

    board[7][0].type_p = Type::Brook;
    board[7][7].type_p = Type::Brook;
    board[7][6].type_p = Type::Bknight;
    board[7][2].type_p = Type::Bknight;
    board[7][1].type_p = Type::Bbishop;
    board[7][5].type_p = Type::Bbishop;
    board[7][4].type_p = Type::Bqueen;
    board[7][3].type_p = Type::Bking;

    for i in 0..8 {
        board[1][i].type_p = Type::Wpawn;
        board[6][i].type_p = Type::Bpawn;
    }

    return board;
}

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