//import library
use chess::{Board, MoveGen};

fn main() {
    //get board
    let board = Board::default();

    //store possible moves in a vector (rust's array)
    let moves: Vec<_> =

        //generate the moves
        MoveGen::new_legal(&board).collect();

    //show moves
    println!("Legal opening moves: {}", moves.len());
    
    for chess_move in moves {
        println!("{}", chess_move);
    }
}
