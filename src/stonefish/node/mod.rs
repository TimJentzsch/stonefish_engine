use pleco::Board;

use self::info::Line;

use super::evaluation::Evaluation;

mod node;
mod minimax;
mod iterative_deepening;
mod info;

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
    /// The current depth of the tree.
    /// 
    /// This has to be kept up to date!
    pub depth: usize,
}
