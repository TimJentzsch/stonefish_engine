use pleco::{BitMove, Board, SQ};

use self::{material_value::{get_piece_value, material_move_delta}, positional_value::positional_move_delta};

use super::evaluation::Evaluation;

mod material_value;
mod positional_value;

/// The heuristic evaluation of the current position.
pub fn heuristic(board: &Board) -> Evaluation {
    if board.checkmate() {
        // The player got checkmated, it's a win for the opponent
        Evaluation::OpponentCheckmate(0)
    } else {
        let mat_value = material_value::material_value(board);
        let pos_value = positional_value::positional_value(board);
        let value = mat_value + pos_value;

        Evaluation::Centipawns(value)
    }
}

/// The rough heuristic evaluation for a given move, used for move ordering.
pub fn move_heuristic(old_eval: Evaluation, old_board: &Board, mv: BitMove, new_board: &Board) -> Evaluation {
    if new_board.checkmate() {
        return Evaluation::PlayerCheckmate(0);
    }
    
    let delta = positional_move_delta(old_board, mv, new_board) + material_move_delta(old_board, mv);

    let new_eval = match old_eval {
        Evaluation::Centipawns(old_val) => Evaluation::Centipawns(old_val + delta),
        _ => old_eval,
    };

    new_eval.for_opponent()
}

#[cfg(test)]
mod tests {
    use pleco::Board;

    use crate::stonefish::{evaluation::Evaluation, heuristic::heuristic};

    #[test]
    fn should_evaluate_start_position() {
        let board = Board::start_pos();
        let expected = Evaluation::Centipawns(0);
        let actual = heuristic(&board);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_checkmate() {
        let board = Board::from_fen("k1R5/8/1K6/8/8/8/8/8 b - - 1 1").unwrap();
        let expected = Evaluation::OpponentCheckmate(0);
        let actual = heuristic(&board);

        assert_eq!(actual, expected);
    }
}
