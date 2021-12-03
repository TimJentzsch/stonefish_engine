use std::sync::atomic::Ordering;

use crate::{uci::uci::StopFlag, stonefish::evaluation::Evaluation};

use super::{Node};

/// The search has been aborted.
#[derive(Debug, PartialEq)]
pub struct StoppedSearch;

impl Node {
    /// The implementation of minimax with alpha-beta-pruning.
    fn minimax_helper(
        &mut self,
        depth: usize,
        alpha: &mut Evaluation,
        beta: &mut Evaluation,
        stop_flag: StopFlag,
    ) -> Result<Evaluation, StoppedSearch> {
        if depth == 0 {
            return Ok(self.evaluation);
        }

        // Check if the search has been aborted
        if stop_flag.load(Ordering::SeqCst) {
            return Err(StoppedSearch);
        }

        // Expand the node if necessary
        if let None = self.children {
            self.expand();
        }

        // Expect the worst
        let mut eval = Evaluation::OpponentCheckmate(0);

        if let Some(children) = &mut self.children {
            if children.len() == 0 {
                // There are no moves to play
                return Ok(self.evaluation);
            }

            // Search through all moves to find the best option
            for child in children {
                let child_eval = child
                    // We have to swap alpha and beta here, because it's the other player's turn
                    .minimax_helper(depth - 1, beta, alpha, stop_flag.clone());

                // Check if the search has been aborted
                if let Err(err) = child_eval {
                    return Err(err);
                }

                // Convert the evaluation to this player's point of view and take the best value
                eval = eval.max(child_eval.unwrap().for_other_player());

                if eval >= *beta {
                    // Prun the branch
                    break;
                }

                *alpha = eval.max(*alpha);
            }
        }

        // Re-sort the children
        if let Some(children) = &mut self.children {
            children.sort();
        }

        self.evaluation = eval;
        Ok(eval)
    }

    /// The minimax search algorithm with alpha-beta-pruning.
    ///
    /// See https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning.
    pub fn minimax(&mut self, depth: usize, stop_flag: StopFlag) -> Result<Evaluation, StoppedSearch> {
        self.minimax_helper(
            depth,
            &mut Evaluation::OpponentCheckmate(0),
            &mut Evaluation::PlayerCheckmate(0),
            stop_flag,
        )
    }
}

#[cfg(test)]
mod test {
    use std::sync::{atomic::AtomicBool, Arc};

    use pleco::Board;

    use crate::stonefish::{evaluation::Evaluation, node::Node};

    #[test]
    fn should_find_mate_in_one_opponent() {
        // Mate in 1 (0 plies)
        let board = Board::from_fen("3Q1k2/5p1p/p3p2P/3p4/8/2Pq2P1/1P3PK1/8 b - - 2 37").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(0, Arc::new(AtomicBool::new(false)));
        let expected = Ok(Evaluation::OpponentCheckmate(0));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_one_player() {
        // Mate in 1 (1 plie)
        let board = Board::from_fen("5k2/5p1p/p3p2P/3p2Q1/8/2Pq2P1/1P3PK1/8 w - - 1 37").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(1, Arc::new(AtomicBool::new(false)));
        let expected = Ok(Evaluation::PlayerCheckmate(1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_two_opponent() {
        // Mate in 2 (2 plies)
        let board = Board::from_fen("8/8/1r3p2/1p6/p5kR/2rB2P1/5P1K/8 b - - 21 47").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(2, Arc::new(AtomicBool::new(false)));
        let expected = Ok(Evaluation::OpponentCheckmate(2));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_two_player() {
        // Mate in 2 (3 plies)
        let board = Board::from_fen("8/7R/1r3p2/1p6/p5k1/2rB2P1/5P1K/8 w - - 20 47").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(3, Arc::new(AtomicBool::new(false)));
        let expected = Ok(Evaluation::PlayerCheckmate(3));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_three_opponent() {
        // Mate in 3 (4 plies)
        let board =
            Board::from_fen("6k1/pp4pp/4p3/3p4/1P1qn3/N3Q3/P2B2PP/2r3K1 w - - 0 21").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(4, Arc::new(AtomicBool::new(false)));
        let expected = Ok(Evaluation::OpponentCheckmate(4));

        assert_eq!(actual, expected);
    }
}
