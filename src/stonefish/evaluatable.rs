use pleco::{Board, PieceType, Player};

use super::evaluation::Evaluation;

pub trait Evaluatable {
    fn material_value(&self, player: Player) -> i32;

    /// Evaluate the position from the view of the given player.
    fn evaluate(&self, _player: Player) -> Evaluation {
        Evaluation::Material(0)
    }
}

impl Evaluatable for Board {
    /// The material value for the given player in centipawns.
    fn material_value(&self, player: Player) -> i32 {
        self.count_piece(player, PieceType::P) as i32 * 100
        + self.count_piece(player, PieceType::N) as i32 * 300
        + self.count_piece(player, PieceType::B) as i32 * 300
        + self.count_piece(player, PieceType::R) as i32 * 500
        + self.count_piece(player, PieceType::Q) as i32 * 800
    }

    /// Evaluate the given position directly.
    fn evaluate(&self, player: Player) -> Evaluation {
        if self.checkmate() {
            if player == self.turn() {
                // The player got checkmated, it's a win for the opponent
                Evaluation::OpponentCheckmate(0)
            } else {
                // The opponent got checkmated, it's a win for the player
                Evaluation::PlayerCheckmate(0)
            }
        } else {
            // A better position for white is a positive value
            let player_mat = self.material_value(player);
            let opponent_mat = self.material_value(player.other_player());

            // Convert to centipawns
            Evaluation::Material(opponent_mat - player_mat)
        }
    }
}
