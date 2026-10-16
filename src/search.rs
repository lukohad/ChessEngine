use chess::{Board, ChessMove, MoveGen};

/// Choose a move from the given position.
///
/// IMPORTANT: Currently returns the first legal move. No search yet!

//look for best move (the board, depth of search underscore is for rust to not worry about it)
//  returns the chess move as option (enum Option<T>)
//      similar to pointer and returning NULL in C
pub fn find_best_move(board: &Board, _depth: u8) -> Option<ChessMove> {
    //evaluate each legal successor position with board.make_move_new(move).
    
    //implement minimax or negamax to explore to a given depth.
    
    //handle terminal positions (checkmate and stalemate).
    
    //add alpha-beta pruning.
    
    //eval scores are White-relative. Negamax convention
    
    //usually expects side-to-move-relative scores; convert consistently.
    
    MoveGen::new_legal(board).next()
}