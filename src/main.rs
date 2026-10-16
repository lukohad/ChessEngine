
use chess::{Board, MoveGen};

fn main() {
    let board = Board::default();

    let moves: Vec<_> =
        MoveGen::new_legal(&board).collect();

    println!("Legal opening moves: {}", moves.len());

    for chess_move in moves {
        println!("{}", chess_move);
    }
}
