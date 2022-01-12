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

    use crate::stonefish::{
        evaluation::Evaluation, types::HashTable, heuristic::initial_heuristic, node::Node,
    };

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

    #[test]
    fn should_properly_update_heuristic() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            // TODO: Fix bug with promotion capture
            // "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 1 1",
            "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 1 1",
            "r1b2k1r/ppp1bppp/8/1B1Q4/5q2/2P5/PPP2PPP/R3R1K1 w - - 1 1",
            "r1b2k1r/ppp1bppp/8/1B1Q4/5q2/2P5/PPP2PPP/R3R1K1 w - - 1 1",
            "5rkr/pp2Rp2/1b1p1Pb1/3P2Q1/2n3P1/2p5/P4P2/4R1K1 w - - 1 1",
            "1r1kr3/Nbppn1pp/1b6/8/6Q1/3B1P2/Pq3P1P/3RR1K1 w - - 1 1",
            "5rk1/1p1q2bp/p2pN1p1/2pP2Bn/2P3P1/1P6/P4QKP/5R2 w - - 1 1",
        ];

        for fen in fens {
            let mut node = Node::new(Board::from_fen(fen).unwrap());
            let parent_heuristic = node.evaluation;
            let children = node.expand(&HashTable::new());

            for child in children {
                let initial_heuristic = initial_heuristic(&child.board).for_opponent();
                let incremental_heuristic = child.evaluation.for_opponent();

                assert_eq!(
                    incremental_heuristic,
                    initial_heuristic,
                    "'{}' ({:?}) -> {}",
                    fen,
                    parent_heuristic,
                    child.board.last_move().unwrap().stringify()
                );
            }
        }
    }
}
