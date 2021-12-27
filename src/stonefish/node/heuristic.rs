use pleco::{Board, PieceType, Player};

use super::Evaluation;

#[derive(Debug, PartialEq)]
struct InCheckError;

/// Get the value of the given piece.
fn get_piece_value(piece: PieceType) -> i32 {
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
fn material_value(board: &Board) -> i32 {
    let player_mat = player_material_value(&board, board.turn());
    let opponent_mat = player_material_value(&board, board.turn().other_player());

    player_mat - opponent_mat
}

/// Calculate the positional value for the current player.
fn player_positional_value(board: &Board, player: Player) -> i32 {
    // For the opponent we have to play a null move to generate the moves
    let mut board = board.clone();

    if player != board.turn() {
        unsafe {
            board.apply_null_move();
        }
    }

    let mut value = 0;
    let captures = board.generate_pseudolegal_moves_of_type(pleco::core::GenTypes::Captures);

    for mv in captures {
        let src_piece = board.piece_at_sq(mv.get_src()).type_of();
        let dest_piece = board.piece_at_sq(mv.get_dest()).type_of();

        // It's good to attack pieces of higher value
        value += 0.max(get_piece_value(dest_piece) - get_piece_value(src_piece)) / 2;
    }

    value
}

/// The current positional value.
///
/// Returns a positive number if the current player has a positional advantage.
fn positional_value(board: &Board) -> i32 {
    if board.in_check() {
        return -50;
    }

    let player_pos = player_positional_value(board, board.turn());
    let opponent_pos = player_positional_value(board, board.turn().other_player());

    player_pos - opponent_pos
}

/// The heuristic evaluation of the current position for move ordering.
pub fn move_order_heuristic(board: &Board) -> Evaluation {
    if board.checkmate() {
        // The player got checkmated, it's a win for the opponent
        Evaluation::OpponentCheckmate(0)
    } else {
        let mat_value = material_value(board);
        let pos_value = positional_value(board);
        let value = mat_value + pos_value;

        Evaluation::Material(value)
    }
}

#[cfg(test)]
mod tests {
    use pleco::Board;

    use crate::stonefish::{evaluation::Evaluation, node::heuristic::move_order_heuristic};

    #[test]
    fn should_evaluate_start_position() {
        let board = Board::start_pos();
        let expected = Evaluation::Material(0);
        let actual = move_order_heuristic(&board);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_checkmate() {
        let board = Board::from_fen("k1R5/8/1K6/8/8/8/8/8 b - - 1 1").unwrap();
        let expected = Evaluation::OpponentCheckmate(0);
        let actual = move_order_heuristic(&board);

        assert_eq!(actual, expected);
    }
}
