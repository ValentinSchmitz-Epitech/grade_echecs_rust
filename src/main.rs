//
// EPITECH PROJECT, 2026
// grade_echecs_rust
// File description:
// main
//

#[derive(Copy,Clone, Debug)]
struct Piece {
    cords: [usize; 2],
}

fn init_board() -> Vec<[Piece; 8]>{
    let mut board: Vec<[Piece; 8]> = vec![[Piece{cords: [0, 0]}; 8]; 8];

    for i in 0..8 {
        for j in 0..8 {
            (board[i][j]).cords = [i, j];
        }
    }
    return board;
}

fn main() {
    let board: Vec<[Piece; 8]> = init_board();
    println!("{:?}", board);
}