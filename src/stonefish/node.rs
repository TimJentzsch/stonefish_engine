use pleco::Board;

use super::evaluation::Evaluation;

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
