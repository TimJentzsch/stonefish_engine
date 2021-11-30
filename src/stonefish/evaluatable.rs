use pleco::{Board, PieceType, Player};

use super::evaluation::Evaluation;

pub trait Evaluatable {
    /// Evaluate the position.
    fn evaluate(&self) -> Evaluation {
        Evaluation::Eval(0)
    }
}

impl Evaluatable for Board {
    /// Evaluate the given position directly.
    fn evaluate(&self) -> Evaluation {
        if self.checkmate() {
            // The other player has won
            Evaluation::Checkmate(0, self.turn().other_player())
        } else {
            // A better position for white is a positive value
            let white_eval = self.non_pawn_material(Player::White)
                + self.count_piece(Player::White, PieceType::P) as i32;
            let black_eval = self.non_pawn_material(Player::Black)
                + self.count_piece(Player::Black, PieceType::P) as i32;

            Evaluation::Eval(white_eval - black_eval)
        }
    }
}
