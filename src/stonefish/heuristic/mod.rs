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
        return Evaluation::Draw;
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
        evaluation::Evaluation,
        heuristic::{final_heuristic, initial_heuristic, move_heuristic},
        node::Node,
        types::HashTable,
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
            "r1bq1rk1/p2pnpbp/2pQ2p1/4p3/2B1P3/2N1B3/PPP2PPP/R3K2R w KQ - 2 11",
            // Castling white
            "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w - - 0 1",
            // Castling black
            "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R b - - 0 1",
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

    #[test]
    fn should_properly_update_heuristic_for_move_sequences() {
        let parameters = [(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            ["e2e3", "d7d5", "d1h5", "g8h6", "h5g5", "g7g6", "g5e5"],
        )];

        for (fen, moves) in parameters {
            let mut cur_board = Board::from_fen(fen).unwrap();
            let mut cur_eval = initial_heuristic(&cur_board);

            for uci_move in moves {
                let mut new_board = cur_board.clone();
                assert!(new_board.apply_uci_move(uci_move));
                let mv = new_board.last_move().unwrap();
                cur_eval = move_heuristic(cur_eval, &cur_board, mv, &new_board);
                let fresh_eval = initial_heuristic(&new_board);

                assert_eq!(cur_eval, fresh_eval, "{fen} after {uci_move}");
                cur_board = new_board;
            }
        }
    }

    #[test]
    fn should_prefer_good_openings() {
        // The left side is the better opening, the right side the worse one
        let parameters = [
            (
                "e2e4 is better than b8c3",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 1",
            ),
            (
                "e2e4 is better than g1g3",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1",
            ),
            (
                "e2e4 is better than e2e3",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
            ),
            (
                "pure castling is better than walking the king",
                "1k1q4/8/8/8/8/8/8/3Q1RK1 w - - 0 1",
                "1k1q4/8/8/8/8/8/8/3Q2KR w - - 0 1",
            ),
            (
                "position castling is better than walking the king first",
                "rnbqk1nr/pp1p1pbp/4p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 w kq - 1 5",
                "rnbqk1nr/pp1p1pbp/4p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ1K1R w kq - 1 5",
            ),
            (
                "position castling is better than walking the king second",
                "r1bqk1nr/pp1p1pbp/2n1p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 w kq - 2 6",
                "r1bqk1nr/pp1p1pbp/2n1p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ2KR w kq - 3 6",
            ),
        ];

        for (name, fen_better, fen_worse) in parameters {
            let board_better = Board::from_fen(fen_better).unwrap();
            let board_worse = Board::from_fen(fen_worse).unwrap();

            let initial_eval_better = initial_heuristic(&board_better);
            let initial_eval_worse = initial_heuristic(&board_worse);

            assert!(
                initial_eval_better > initial_eval_worse,
                "Initial: {initial_eval_better:?} <= {initial_eval_worse:?} {name}"
            );

            let final_eval_better = final_heuristic(initial_eval_better, &board_better);
            let final_eval_worse = final_heuristic(initial_eval_worse, &board_worse);

            assert!(
                final_eval_better > final_eval_worse,
                "Final: {final_eval_better:?} <= {final_eval_worse:?} {name}"
            );
        }
    }
}
