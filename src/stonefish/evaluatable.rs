use pleco::{Board, PieceType, Player};

use super::evaluation::Evaluation;

pub trait Evaluatable {
    fn material_value(&self, player: Player) -> i32;

    /// Evaluate the position from the view of the current player.
    fn evaluate(&self) -> Evaluation {
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
    fn evaluate(&self) -> Evaluation {
        if self.checkmate() {
            // The player got checkmated, it's a win for the opponent
            Evaluation::OpponentCheckmate(0)
        } else {
            // A better position for white is a positive value
            let player_mat = self.material_value(self.turn());
            let opponent_mat = self.material_value(self.turn().other_player());

            // Convert to centipawns
            Evaluation::Material(player_mat - opponent_mat)
        }
    }
}
#[cfg(test)]
mod tests {
    use pleco::Board;

    use crate::stonefish::evaluation::Evaluation;

    use super::Evaluatable;

    #[test]
    fn should_evaluate_start_position() {
        let board = Board::start_pos();
        let expected = Evaluation::Material(0);
        let actual = board.evaluate();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_checkmate() {
        let board = Board::from_fen("k1R5/8/1K6/8/8/8/8/8 b - - 1 1").unwrap();
        let expected = Evaluation::OpponentCheckmate(0);
        let actual = board.evaluate();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_player_material_advantage() {
        let board = Board::from_fen("k7/8/8/8/8/1K4R1/8/8 w - - 0 1").unwrap();
        let expected = Evaluation::Material(500);
        let actual = board.evaluate();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_opponent_material_advantage() {
        let board = Board::from_fen("k7/8/8/8/8/1K4R1/8/8 b - - 0 1").unwrap();
        let expected = Evaluation::Material(-500);
        let actual = board.evaluate();

        assert_eq!(actual, expected);
    }
}
