use crate::stonefish::{
    abort_flags::{AbortFlags, SearchAborted},
    evaluation::Evaluation,
    heuristic::final_heuristic,
    types::{HashTable, HashTableEntry, RepititionTable},
};

use super::Node;

impl Node {
    /// The implementation of minimax with alpha-beta-pruning.
    ///
    /// - `alpha`: Minimum value the current player is assured of
    /// - `beta`: Minimum value the opponent player is assured of
    fn minimax_helper(
        &mut self,
        depth: usize,
        alpha: Evaluation,
        beta: Evaluation,
        hash_table: &mut HashTable,
        repitition_table: &mut RepititionTable,
        abort_flags: AbortFlags,
    ) -> Result<Evaluation, SearchAborted> {
        // Check for repitition
        if repitition_table.insert_check_draw(&self.board) {
            repitition_table.remove(&self.board);
            return Ok(Evaluation::DRAW);
        }

        if depth == 0 {
            // Update the evaluation with a more expensive analysis
            self.evaluation = final_heuristic(self.evaluation, &self.board);
            repitition_table.remove(&self.board);
            return Ok(self.evaluation);
        }

        // Check if the search has been aborted
        abort_flags.check()?;

        // Check if the value has been cached
        // TODO: Fix this breaking the evaluation
        // if let Some(HashTableEntry {
        //     evaluation: cache_eval,
        //     best_line: cache_line,
        //     depth: cache_depth,
        // }) = hash_table.get(&self.board.zobrist())
        // {
        //     // Only use the cached value if it has sufficient depth
        //     if cache_eval.is_forced_mate() || *cache_depth >= depth {
        //         self.evaluation = *cache_eval;
        //         self.best_line = cache_line.clone();
        //         repitition_table.remove(&self.board);
        //         return Ok(self.evaluation);
        //     }
        // }

        // Expect the worst
        let mut cur_evaluation = Evaluation::OpponentCheckmate(0);
        let mut alpha = alpha;

        // Expand the node
        let mut children = self.expand(hash_table);

        if children.is_empty() {
            // Update the evaluation with a more expensive analysis
            self.evaluation = final_heuristic(self.evaluation, &self.board);
            repitition_table.remove(&self.board);
            return Ok(self.evaluation);
        }

        // Search through all moves to find the best option
        for child in &mut children {
            let child_eval = child
                // We have to swap alpha and beta here, because it's the other player's turn
                .minimax_helper(
                    depth - 1,
                    beta,
                    alpha,
                    hash_table,
                    repitition_table,
                    abort_flags.clone(),
                );

            // Check if the search has been aborted
            if let Err(err) = child_eval {
                self.update_attributes(&children);
                repitition_table.remove(&self.board);
                return Err(err);
            }

            // Convert the evaluation to this player's point of view and take the best value
            cur_evaluation = cur_evaluation.max(child_eval.unwrap().for_opponent().previous_plie());

            if cur_evaluation.for_opponent() <= beta {
                // The opponent has a better option in another branch, they won't choose this one
                break;
            }

            // Update what our current best option is
            alpha = alpha.max(cur_evaluation);
        }

        // Keep depth and size up-to-date
        self.update_attributes(&children);
        self.evaluation = cur_evaluation;
        hash_table.insert(self.board.zobrist(), HashTableEntry::from_node(self));
        repitition_table.remove(&self.board);
        Ok(self.evaluation)
    }

    /// The minimax search algorithm with alpha-beta-pruning.
    ///
    /// See https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning.
    pub fn minimax(
        &mut self,
        depth: usize,
        hash_table: &mut HashTable,
        repitition_table: &mut RepititionTable,
        abort_flags: AbortFlags,
    ) -> Result<Evaluation, SearchAborted> {
        self.minimax_helper(
            depth,
            Evaluation::OpponentCheckmate(0),
            Evaluation::OpponentCheckmate(0),
            hash_table,
            repitition_table,
            abort_flags,
        )
    }
}

#[cfg(test)]
mod test {
    use pleco::Board;

    use crate::stonefish::{
        abort_flags::AbortFlags,
        evaluation::Evaluation,
        node::{minimax::HashTable, Node},
        types::RepititionTable,
    };

    #[test]
    fn should_find_mate_in_one_opponent() {
        // Mate in 1 (0 plies)
        let board = Board::from_fen("3Q1k2/5p1p/p3p2P/3p4/8/2Pq2P1/1P3PK1/8 b - - 2 37").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(
            0,
            &mut HashTable::new(),
            &mut RepititionTable::new(),
            AbortFlags::new(),
        );
        let expected = Ok(Evaluation::OpponentCheckmate(0));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_one_player() {
        // Mate in 1 (1 plie)
        let board = Board::from_fen("5k2/5p1p/p3p2P/3p2Q1/8/2Pq2P1/1P3PK1/8 w - - 1 37").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(
            1,
            &mut HashTable::new(),
            &mut RepititionTable::new(),
            AbortFlags::new(),
        );
        let expected = Ok(Evaluation::PlayerCheckmate(1));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_two_opponent() {
        // Mate in 2 (2 plies)
        let board = Board::from_fen("8/8/1r3p2/1p6/p5kR/2rB2P1/5P1K/8 b - - 21 47").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(
            2,
            &mut HashTable::new(),
            &mut RepititionTable::new(),
            AbortFlags::new(),
        );
        let expected = Ok(Evaluation::OpponentCheckmate(2));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_two_player() {
        // Mate in 2 (3 plies)
        let board = Board::from_fen("8/7R/1r3p2/1p6/p5k1/2rB2P1/5P1K/8 w - - 20 47").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(
            3,
            &mut HashTable::new(),
            &mut RepititionTable::new(),
            AbortFlags::new(),
        );
        let expected = Ok(Evaluation::PlayerCheckmate(3));

        assert_eq!(actual, expected);
    }

    #[test]
    fn should_find_mate_in_three_opponent() {
        // Mate in 3 (4 plies)
        let board =
            Board::from_fen("6k1/pp4pp/4p3/3p4/1P1qn3/N3Q3/P2B2PP/2r3K1 w - - 0 21").unwrap();
        let mut node = Node::new(board);
        let actual = node.minimax(
            4,
            &mut HashTable::new(),
            &mut RepititionTable::new(),
            AbortFlags::new(),
        );
        let expected = Ok(Evaluation::OpponentCheckmate(4));

        assert_eq!(actual, expected);
    }
}
