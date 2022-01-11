use pleco::{BitMove, Board};

use self::{
    material_value::material_move_delta,
    positional_value::{move_positional_value, threat_value},
};

use super::evaluation::Evaluation;

mod material_value;
mod positional_value;

/// The initial heuristic value of a position.
pub fn initial_heuristic(board: &Board) -> Evaluation {
    if board.checkmate() {
        // The player got checkmated, it's a win for the opponent
        Evaluation::OpponentCheckmate(0)
    } else {
        let mat_value = material_value::material_value(board);
        let pos_value = positional_value::initial_positional_value(board);
        let value = mat_value + pos_value;

        Evaluation::Centipawns(value)
    }
}

/// The rough heuristic evaluation for a given move, used for move ordering.
pub fn move_heuristic(
    old_eval: Evaluation,
    old_board: &Board,
    mv: BitMove,
    new_board: &Board,
) -> Evaluation {
    let delta =
        move_positional_value(old_board, mv, new_board) + material_move_delta(old_board, mv);

    let new_eval = match old_eval {
        Evaluation::Centipawns(old_val) => Evaluation::Centipawns(old_val + delta),
        _ => old_eval,
    };

    new_eval.for_opponent()
}

/// Determine the heuristic value of the final position.
pub fn final_heuristic(old_eval: Evaluation, board: &Board) -> Evaluation {
    // First check if the board is in a final state
    let delta = if board.checkmate() {
        return Evaluation::OpponentCheckmate(0);
    } else if board.stalemate() {
        return Evaluation::Centipawns(0);
    } else {
        threat_value(board)
    };

    match old_eval {
        Evaluation::Centipawns(old_val) => Evaluation::Centipawns(old_val + delta),
        _ => old_eval,
    }
}

#[cfg(test)]
mod tests {
    use pleco::Board;

    use crate::stonefish::{evaluation::Evaluation, heuristic::initial_heuristic};

    #[test]
    fn should_evaluate_start_position() {
        let board = Board::start_pos();
        let expected = Evaluation::Centipawns(0);
        let actual = initial_heuristic(&board);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_evaluate_checkmate() {
        let board = Board::from_fen("k1R5/8/1K6/8/8/8/8/8 b - - 1 1").unwrap();
        let expected = Evaluation::OpponentCheckmate(0);
        let actual = initial_heuristic(&board);

        assert_eq!(actual, expected);
    }
}
