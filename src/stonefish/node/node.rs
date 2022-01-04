use pleco::Board;

use crate::stonefish::heuristic::heuristic;

use super::{minimax::HashTable, Node, info::Children};

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
        }
    }


    /// Create a new node.
    ///
    /// If the state is already available in the hash table, it is taken as evaluation.
    /// Otherwise, the heuristic value is used.
    pub fn new_from_hash_table(state: Board, hash_table: &HashTable) -> Self {
        let zobrist = state.zobrist();
        // Check if the hash table already has an entry for this position
        let evaluation = if let Some(cached_node) = hash_table.get(&zobrist) {
            cached_node.evaluation
        } else {
            heuristic(&state)
        };

        Self {
            board: state,
            evaluation,
            best_line: vec![],
            size: 1,
            depth: 0,
        }
    }

    /// Expands this node.
    ///
    /// This will generate all children of this node.
    pub fn expand(&mut self, hash_table: &HashTable) -> Children {
        let mut children: Children = self
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
                Node::new_from_hash_table(new_state, hash_table)
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
        match self.cmp(other) {
            std::cmp::Ordering::Equal => true,
            _ => false,
        }
    }
}
