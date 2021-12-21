use pleco::Board;

use crate::stonefish::evaluation::evaluatable::Evaluatable;

use super::{minimax::HashTable, Node};

impl Node {
    /// Create a new node with the heuristic evaluation.
    pub fn new(state: Board) -> Self {
        let eval = state.heuristic();

        Self {
            board: state,
            evaluation: eval,
            children: None,
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
        let evaluation = if let Some(&eval) = hash_table.get(&zobrist) {
            eval
        } else {
            state.heuristic()
        };

        Self {
            board: state,
            evaluation,
            children: None,
            size: 1,
            depth: 0,
        }
    }

    /// Expands this node.
    ///
    /// This will generate all children of this node.
    pub fn expand(&mut self, hash_table: &HashTable) -> &mut Self {
        let children: Vec<Node> = self
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

        // Important: Keep attributes up-to-date
        self.update_attributes();
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
