use pleco::Board;

use super::{evaluatable::Evaluatable, evaluation::Evaluation};

/// A node of a search tree.
pub struct Node {
    /// The current board state.
    pub state: Board,
    /// The current evaluation for this position.
    pub evaluation: Evaluation,
    /// The children of this node.
    ///
    /// This will be `None` if the node has not been expanded yet.
    pub children: Option<Vec<Node>>,
}

impl Node {
    /// Create a new node with the heuristic evaluation.
    fn new(state: Board) -> Node {
        let eval = state.evaluate();

        Node {
            state: state,
            evaluation: eval,
            children: None,
        }
    }

    /// Expands this node.
    ///
    /// This will generate all children of this node.
    fn expand(&mut self) {
        let children: Vec<Node> = self
            .state
            // Generate all possible moves
            .generate_moves()
            .iter()
            // Create a new child for each move
            .map(|mv| {
                // Play the move on a new board
                let mut new_state = self.state.clone();
                new_state.apply_move(*mv);
                // Create a new node with the standard evaluation
                Node::new(new_state)
            })
            .collect();

        self.children = Some(children);
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
