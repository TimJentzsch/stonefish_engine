use pleco::{Board, PieceType, Player};

use super::Evaluation;

pub trait Evaluatable {
    fn material_value(&self, player: Player) -> i32;
    fn positional_value(&self) -> i32;
    fn heuristic(&self) -> Evaluation;
}

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

impl Evaluatable for Board {
    /// The material value for the given player in centipawns.
    fn material_value(&self, player: Player) -> i32 {
        [
            PieceType::P,
            PieceType::N,
            PieceType::B,
            PieceType::R,
            PieceType::Q,
        ]
        .into_iter()
        .map(|piece| self.count_piece(player, piece) as i32 * get_piece_value(piece))
        .sum()
    }

    /// Calculate the positional value for the current player.
    fn positional_value(&self) -> i32 {
        let mut value = 0;
        let captures = self.generate_pseudolegal_moves_of_type(pleco::core::GenTypes::Captures);

        for mv in captures {
            let src_piece = self.piece_at_sq(mv.get_src()).type_of();
            let dest_piece = self.piece_at_sq(mv.get_dest()).type_of();

            // It's good to attack pieces of higher value
            value += 0.max(get_piece_value(dest_piece) - get_piece_value(src_piece)) / 2;
        }

        value
    }

    /// Get a heuristic evaluation for the current position.
    fn heuristic(&self) -> Evaluation {
        if self.checkmate() {
            // The player got checkmated, it's a win for the opponent
            Evaluation::OpponentCheckmate(0)
        } else {
            // A better position for white is a positive value
            let player_mat = self.material_value(self.turn());
            let opponent_mat = self.material_value(self.turn().other_player());

            let mut value = player_mat - opponent_mat;

            if self.in_check() {
                // Being in check is bad
                value -= 50;
            } else {
                // Calculate positional value
                let mut opponent_board = self.clone();
                unsafe {
                    opponent_board.apply_null_move();
                }

                let player_pos = self.positional_value();
                let opponent_pos = opponent_board.positional_value();

                value += player_pos - opponent_pos;
            }

            // Convert to centipawns
            Evaluation::Material(value)
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
        let actual = board.heuristic();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_checkmate() {
        let board = Board::from_fen("k1R5/8/1K6/8/8/8/8/8 b - - 1 1").unwrap();
        let expected = Evaluation::OpponentCheckmate(0);
        let actual = board.heuristic();

        assert_eq!(actual, expected);
    }
}
