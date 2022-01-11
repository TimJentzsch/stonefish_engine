use pleco::{BitMove, Board, PieceType, Player};

/// Get the value of the given piece.
pub fn get_piece_value(piece: PieceType) -> i32 {
    match piece {
        PieceType::P => 100,
        PieceType::N | PieceType::B => 300,
        PieceType::R => 500,
        PieceType::Q => 800,
        _ => 0,
    }
}

/// The material value for the given player in centipawns.
fn player_material_value(board: &Board, player: Player) -> i32 {
    [
        PieceType::P,
        PieceType::N,
        PieceType::B,
        PieceType::R,
        PieceType::Q,
    ]
    .into_iter()
    .map(|piece| board.count_piece(player, piece) as i32 * get_piece_value(piece))
    .sum()
}

/// The material value from the view of the current player.
///
/// Returns a positive value for a material advantage.
pub fn material_value(board: &Board) -> i32 {
    let player_mat = player_material_value(board, board.turn());
    let opponent_mat = player_material_value(board, board.turn().other_player());

    player_mat - opponent_mat
}

/// The change in material value of the given move.
pub fn material_move_delta(old_board: &Board, mv: BitMove) -> i32 {
    if mv.is_capture() {
        let captured_piece = old_board.piece_at_sq(mv.get_dest()).type_of();
        get_piece_value(captured_piece)
    } else {
        0
    }
}
