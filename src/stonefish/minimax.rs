use super::{evaluation::Evaluation, node::Node};

impl Node {
    /// The minimax search algorithm.
    fn minimax(&mut self, depth: usize) -> Evaluation {
        if depth == 0 {
            return self.evaluation;
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
                return self.evaluation;
            }

            // Search through all moves to find the best option
            for child in children {
                // Convert the evaluation to this player's point of view
                let child_eval = child.minimax(depth - 1).for_other_player();
                eval = eval.max(child_eval);
            }
        }

        self.evaluation = eval;
        eval
    }
}

#[cfg(test)]
mod test {
    use pleco::Board;

    use crate::stonefish::{evaluation::Evaluation, node::Node};

    #[test]
    fn should_find_mate_in_one_opponent() {
        // Mate in 1 (0 plies)
        let board = Board::from_fen("3Q1k2/5p1p/p3p2P/3p4/8/2Pq2P1/1P3PK1/8 b - - 2 37").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(0);
        let expected = Evaluation::OpponentCheckmate(0);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_one_player() {
        // Mate in 1 (1 plie)
        let board = Board::from_fen("5k2/5p1p/p3p2P/3p2Q1/8/2Pq2P1/1P3PK1/8 w - - 1 37").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(1);
        let expected = Evaluation::PlayerCheckmate(1);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_two_opponent() {
        // Mate in 2 (2 plies)
        let board = Board::from_fen("8/8/1r3p2/1p6/p5kR/2rB2P1/5P1K/8 b - - 21 47").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(2);
        let expected = Evaluation::OpponentCheckmate(2);

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_two_player() {
        // Mate in 2 (3 plies)
        let board = Board::from_fen("8/7R/1r3p2/1p6/p5k1/2rB2P1/5P1K/8 w - - 20 47").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(3);
        let expected = Evaluation::PlayerCheckmate(3);

        assert_eq!(actual, expected);
    }
}
