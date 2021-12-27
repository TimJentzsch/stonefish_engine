use pleco::Board;

use super::evaluation::Evaluation;

mod node;
mod minimax;
mod iterative_deepening;
mod info;
mod heuristic;

/// A node of a search tree.
pub struct Node {
    /// The current board state.
    pub board: Board,
    /// The current evaluation for this position.
    pub evaluation: Evaluation,
    /// The children of this node.
    /// 
    /// They should always be sorted by their evaluation.
    ///
    /// This will be `None` if the node has not been expanded yet.
    pub children: Option<Vec<Node>>,
    /// The current size of the tree.
    /// 
    /// This has to be kept up to date!
    pub size: usize,
    /// The current depth of the tree.
    /// 
    /// This has to be kept up to date!
    pub depth: usize,
}
