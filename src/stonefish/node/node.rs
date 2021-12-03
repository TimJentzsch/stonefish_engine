use pleco::Board;

use crate::stonefish::evaluation::evaluatable::Evaluatable;

use super::{Node};

impl Node {
    /// Create a new node with the heuristic evaluation.
    pub fn new(state: Board) -> Node {
        let eval = state.evaluate();

        Node {
            board: state,
            evaluation: eval,
            children: None,
        }
    }

    /// Expands this node.
    ///
    /// This will generate all children of this node.
    pub fn expand(&mut self) -> &mut Self {
        let mut children: Vec<Node> = self
            .board
            // Generate all possible moves
            .generate_moves()
            .iter()
            // Create a new child for each move
            .map(|mv| {
                // Play the move on a new board
                let mut new_state = self.board.clone();
                new_state.apply_move(*mv);
                // Create a new node with the standard evaluation
                // The next node will have the view of the opponent
                Node::new(new_state)
            })
            .collect();

        children.sort();
        self.children = Some(children);

        self
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
        match self.cmp(other) {
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
    }
}
