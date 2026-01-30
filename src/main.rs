//
// EPITECH PROJECT, 2026
// grade_echecs_rust
// File description:
// main
//

#[derive(Copy,Clone, Debug)]
enum Type {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy,Clone, Debug)]
struct Piece {
    cords: [usize; 2],
    type_p: Type,
    dead: bool,
}

fn init_board() -> ([Piece; 16], [Piece; 16]){
    let mut w_pieces: [Piece; 16] = [Piece{
        cords: [0, 0], type_p: Type::Pawn, dead: false}; 16];
    let mut b_pieces: [Piece; 16] = [Piece{
        cords: [0, 0], type_p: Type::Pawn, dead: false}; 16];

    w_pieces[8].type_p = Type::Rook;
    w_pieces[9].type_p = Type::Rook;
    w_pieces[10].type_p = Type::Knight;
    w_pieces[11].type_p = Type::Knight;
    w_pieces[12].type_p = Type::Bishop;
    w_pieces[13].type_p = Type::Bishop;
    w_pieces[14].type_p = Type::Queen;
    w_pieces[15].type_p = Type::King;

    b_pieces[8].type_p = Type::Rook;
    b_pieces[9].type_p = Type::Rook;
    b_pieces[10].type_p = Type::Knight;
    b_pieces[11].type_p = Type::Knight;
    b_pieces[12].type_p = Type::Bishop;
    b_pieces[13].type_p = Type::Bishop;
    b_pieces[14].type_p = Type::Queen;
    b_pieces[15].type_p = Type::King;

    for i in 0..8 {
        w_pieces[i].cords = [1, i];
        b_pieces[i].cords = [6, i];
    }
    w_pieces[8].cords = [0, 0];
    w_pieces[9].cords = [0, 7];
    w_pieces[10].cords = [0, 1];
    w_pieces[11].cords = [0, 6];
    w_pieces[12].cords = [0, 2];
    w_pieces[13].cords = [0, 5];
    w_pieces[14].cords = [0, 4];
    w_pieces[15].cords = [0, 3];

    b_pieces[8].cords = [7, 0];
    b_pieces[9].cords = [7, 7];
    b_pieces[11].cords = [7, 6];
    b_pieces[12].cords = [7, 2];
    b_pieces[10].cords = [7, 1];
    b_pieces[13].cords = [7, 5];
    b_pieces[14].cords = [7, 4];
    b_pieces[15].cords = [7, 3];

    return (w_pieces, b_pieces);
}

fn main() {
    let tmp: ([Piece; 16], [Piece; 16]) = init_board();
    let mut w_pieces: [Piece; 16] = tmp.0;
    let mut b_pieces: [Piece; 16] = tmp.1;

    println!("{:?}", w_pieces);
}