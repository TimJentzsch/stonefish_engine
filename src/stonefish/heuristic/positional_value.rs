use pleco::{Board, Piece, Player, SQ};

use super::material_value::get_piece_value;

struct Guard {
    origin: (SQ, Piece),
    target: (SQ, Piece),
}

impl Guard {
    fn new(origin: (SQ, Piece), target: (SQ, Piece)) -> Self {
        Self { origin, target }
    }
}

/// Gets the squares and pieces that the player guards (defends or attacks).
fn player_guards(board: &Board, player: Player) -> Vec<Guard> {
    let piece_locations = board.get_piece_locations();

    let mut guards = vec![];

    for (sq, piece) in piece_locations {
        // Only consider pieces of the current player
        if piece.player().unwrap_or(player.other_player()) != player {
            continue;
        }

        // Look at every square that the piece attacks
        for attack_sq in board.attacks_from(piece.type_of(), sq, player) {
            let target = board.piece_at_sq(attack_sq);

            guards.push(Guard::new((sq, piece), (attack_sq, target)));
        }
    }

    guards
}

/// Calculate the positional value for the current player.
fn player_positional_value(board: &Board, player: Player) -> i32 {
    let mut value = 0;
    
    let guards = player_guards(board, player);

    for guard in guards {
        let (_, origin_piece) = guard.origin;
        let (_, target_piece) = guard.target;

        if origin_piece.player() == target_piece.player() {
            value += 20;
        } else {
            value += get_piece_value(target_piece.type_of()) / 10;
        }
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
