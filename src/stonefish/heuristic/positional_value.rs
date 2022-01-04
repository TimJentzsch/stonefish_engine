use pleco::{Board, Player};

use super::material_value::get_piece_value;

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
pub fn positional_value(board: &Board) -> i32 {
    if board.in_check() {
        return -50;
    }

    let player_pos = player_positional_value(board, board.turn());
    let opponent_pos = player_positional_value(board, board.turn().other_player());

    player_pos - opponent_pos
}
