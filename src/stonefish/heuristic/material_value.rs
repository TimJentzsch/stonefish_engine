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

/// The material value of the current board.
///
/// A positive number is an advantage for White.
pub fn material_value(board: &Board) -> i32 {
    let white_mat = player_material_value(board, Player::White);
    let black_mat = player_material_value(board, Player::Black);

    white_mat - black_mat
}

/// The change in material value of the given move.
pub fn material_move_delta(old_board: &Board, mv: BitMove) -> i32 {
    let mut value = 0;

    if mv.is_capture() {
        // We gain the captured piece
        let captured_piece = old_board.piece_at_sq(mv.get_dest()).type_of();
        value += get_piece_value(captured_piece)
    }
    if mv.is_promo() {
        // We gain the promotion piece and lose a pawn
        value += get_piece_value(mv.promo_piece()) - get_piece_value(PieceType::P)
    }

    match old_board.turn() {
        Player::White => value,
        Player::Black => -value,
    }
}
