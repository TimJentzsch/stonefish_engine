use pleco::{BitMove, Board};

use self::info::Line;

use super::{evaluation::Evaluation, heuristic::move_heuristic};

mod info;
mod iterative_deepening;
mod minimax;

/// A node of a search tree.
#[derive(Debug, Clone)]
pub struct Node {
    /// The current board state.
    pub board: Board,
    /// The current evaluation for this position.
    pub evaluation: Evaluation,
    /// The best line to play from this position.
    pub best_line: Line,
    /// The current size of the tree.
    ///
    /// This has to be kept up to date!
    pub size: usize,
    /// The current minimum depth of the tree.
    ///
    /// This has to be kept up to date!
    pub depth: usize,
    /// The current maximum depth of the tree.
    ///
    /// This has to be kept up to date!
    pub sel_depth: usize,
}

use crate::stonefish::heuristic::heuristic;

use self::{info::Children, minimax::HashTable};

impl Node {
    /// Create a new node with move order heuristic.
    pub fn new(state: Board) -> Self {
        let evaluation = heuristic(&state);

        Self {
            board: state,
            evaluation,
            best_line: vec![],
            size: 1,
            depth: 0,
            sel_depth: 0,
        }
    }

    /// Copy the values from another node.
    pub fn copy_values(&mut self, other: &Node) {
        self.board = other.board.clone();
        self.evaluation = other.evaluation;
        self.best_line = other.best_line.clone();
        self.size = other.size;
        self.depth = other.depth;
        self.sel_depth = other.sel_depth;
    }

    /// Create a new node from a given move.
    pub fn new_from_move(
        old_eval: Evaluation,
        old_board: &Board,
        mv: BitMove,
        hash_table: &HashTable,
    ) -> Self {
        let mut board = old_board.clone();
        board.apply_move(mv);

        // If the board is already cached, return it
        if let Some(cached_node) = hash_table.get(&board.zobrist()) {
            return Self {
                board,
                evaluation: cached_node.evaluation,
                best_line: cached_node.best_line.clone(),
                size: 1,
                depth: 0,
                sel_depth: 0,
            };
        }

        let evaluation = move_heuristic(old_eval, old_board, mv, &board);

        Self {
            board,
            evaluation,
            best_line: vec![],
            size: 1,
            depth: 0,
            sel_depth: 0,
        }
    }

    /// Reset the evaluation of the node.
    pub fn reset(&mut self) -> &mut Self {
        self.evaluation = heuristic(&self.board);
        self.depth = 0;
        self.sel_depth = 0;
        self.size = 0;
        self.best_line = vec![];
        self
    }

    /// Update the attributes of the node.
    ///
    /// This should always be called after the children have been modified.
    pub fn update_attributes(&mut self, children: &Children) {
        let mut size: usize = 1;
        let mut depth: usize = 0;
        let mut best_child: Option<&Node> = None;

        for child in children {
            size += child.size;
            depth = depth.max(child.depth + 1);

            best_child = if let Some(prev_best) = best_child {
                if child.evaluation.for_opponent().previous_plie()
                    > prev_best.evaluation.for_opponent().previous_plie()
                {
                    Some(child)
                } else {
                    Some(prev_best)
                }
            } else {
                Some(child)
            }
        }

        self.size = size;
        self.depth = depth;

        if let Some(best_child) = best_child {
            // The evaluation of the node is the evaluation of the best child
            self.evaluation = best_child.evaluation.for_opponent().previous_plie();
            // The best line to play is the best child and its line
            let mv = best_child.board.last_move().unwrap();
            let mut best_line = best_child.best_line.clone();
            best_line.splice(0..0, [mv]);

            self.best_line = best_line;
        } else {
            self.best_line = vec![];
        }

        self.sel_depth = self.best_line.len();
    }

    /// Expands this node.
    ///
    /// This will generate all children of this node.
    pub fn expand(&mut self, high_quality: bool, hash_table: &HashTable) -> Children {
        let mut children: Children = self
            .board
            // Generate all possible moves
            .generate_moves()
            .iter()
            // Create a new child for each move
            .map(|mv| {
                if high_quality {
                    let mut board = self.board.clone();
                    board.apply_move(*mv);
                    Node::new(board)
                } else {
                    Node::new_from_move(self.evaluation, &self.board, *mv, hash_table)
                }
            })
            .collect();

        children.sort();

        // Important: Keep attributes up-to-date
        self.update_attributes(&children);

        children
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.evaluation.cmp(&other.evaluation)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), std::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use pleco::Board;

    use crate::stonefish::{
        evaluation::Evaluation,
        node::{minimax::HashTable, Node},
    };

    #[test]
    fn should_expand_startpos() {
        let mut startpos = Node::new(Board::start_pos());

        assert_eq!(startpos.size, 1);
        assert_eq!(startpos.depth, 0);
        assert_eq!(startpos.sel_depth, 0);
        assert_eq!(startpos.best_line.len(), 0);

        let children = startpos.expand(true, &HashTable::new());

        for child in children {
            assert_eq!(child.size, 1);
            assert_eq!(child.depth, 0);
            assert_eq!(child.sel_depth, 0);
            assert_eq!(child.best_line.len(), 0);
        }

        // 20 moves are possible, plus the root node
        assert_eq!(startpos.size, 21);
        assert_eq!(startpos.depth, 1);
        assert_eq!(startpos.sel_depth, 1);
        assert_eq!(startpos.best_line.len(), 1);
    }

    #[test]
    fn should_expand_forced_mate_0() {
        let mut pos = Node::new(
            Board::from_fen("1Nb4r/p2p3p/kb1P3n/3Q4/N3Pp2/8/P1P3PP/7K b - - 0 2").unwrap(),
        );

        assert_eq!(pos.size, 1);
        assert_eq!(pos.depth, 0);
        assert_eq!(pos.sel_depth, 0);
        assert_eq!(pos.best_line.len(), 0);
        assert_eq!(pos.evaluation, Evaluation::OpponentCheckmate(0));

        let children = pos.expand(true, &HashTable::new());
        assert_eq!(children.len(), 0);

        assert_eq!(pos.depth, 0);
        assert_eq!(pos.sel_depth, 0);
        assert_eq!(pos.best_line.len(), 0);
        assert_eq!(pos.evaluation, Evaluation::OpponentCheckmate(0));
    }

    #[test]
    fn should_expand_forced_mate_1() {
        let mut pos = Node::new(
            Board::from_fen("1rb4r/p1Pp3p/kb1P3n/3Q4/N3Pp2/8/P1P3PP/7K w - - 3 2").unwrap(),
        );

        assert_eq!(pos.size, 1);
        assert_eq!(pos.depth, 0);
        assert_eq!(pos.sel_depth, 0);
        assert_eq!(pos.best_line.len(), 0);
        assert!(!pos.evaluation.is_forced_mate());

        let children = pos.expand(true, &HashTable::new());

        for child in children {
            assert_eq!(child.size, 1);
            assert_eq!(child.depth, 0);
            assert_eq!(child.sel_depth, 0);
            assert_eq!(child.best_line.len(), 0);
        }

        assert_eq!(pos.depth, 1);
        assert_eq!(pos.sel_depth, 1);
        assert_eq!(pos.best_line.len(), 1);
        assert_eq!(pos.evaluation, Evaluation::PlayerCheckmate(1));
    }
}
