use pleco::{Board, PieceType, Player};

use super::Evaluation;

pub trait Evaluatable {
    fn material_value(&self, player: Player) -> i32;
    fn positional_value(&self) -> i32;
    fn heuristic(&self) -> Evaluation;
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

    /// Calculate the positional value for the current player.
    fn positional_value(&self) -> i32 {
        let mut value = 0;
        let pseudo_moves = self.generate_pseudolegal_moves();

        for mv in pseudo_moves {
            let src_piece = self.piece_at_sq(mv.get_src());

            // It's good when the pieces can move freely
            value += match src_piece.type_of() {
                PieceType::Q => 3,
                PieceType::R => 2,
                PieceType::N => 2,
                PieceType::B => 1,
                _ => 0,
            };

            let dest_piece = self.piece_at_sq(mv.get_dest());

            // It's good to attack pieces
            value += match dest_piece.type_of() {
                PieceType::Q => 40,
                PieceType::R => 25,
                PieceType::B => 15,
                PieceType::N => 15,
                _ => 0,
            };
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

    #[test]
    fn should_evaluate_player_material_advantage() {
        let board = Board::from_fen("k7/8/8/8/8/1K4R1/8/8 w - - 0 1").unwrap();
        let expected = Evaluation::Material(500);
        let actual = board.heuristic();

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_opponent_material_advantage() {
        let board = Board::from_fen("k7/8/8/8/8/1K4R1/8/8 b - - 0 1").unwrap();
        let expected = Evaluation::Material(-500);
        let actual = board.heuristic();

        assert_eq!(actual, expected);
    }
}
