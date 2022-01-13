use std::collections::HashMap;

use pleco::{BitMove, Board};

use super::{evaluation::Evaluation, node::Node};

pub struct HashTableEntry {
    pub evaluation: Evaluation,
    pub best_line: Line,
    pub depth: usize,
}

impl HashTableEntry {
    pub fn from_node(node: &Node) -> Self {
        Self {
            evaluation: node.evaluation,
            best_line: node.best_line.clone(),
            depth: node.depth,
        }
    }
}

pub type HashTable = HashMap<u64, HashTableEntry>;
pub type Line = Vec<BitMove>;
pub type Children = Vec<Node>;

#[derive(Debug, Clone)]
pub struct RepititionTable(HashMap<u64, usize>);

impl RepititionTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Add the board to the repitition table.
    ///
    /// Returns the new count for the given board.
    pub fn insert(&mut self, board: &Board) -> usize {
        let zobrist = board.zobrist();

        if let Some(&count) = self.0.get(&zobrist) {
            self.0.insert(zobrist, count + 1);
            count + 1
        } else {
            self.0.insert(zobrist, 1);
            1
        }
    }

    /// Add the board to the repitition table and check if it's a draw.
    /// 
    /// It is a draw if the position occurred 3 times (a player has to claim the draw)
    /// or if the position occurred 5 times (automatic draw).
    /// 
    /// See <https://lichess.org/faq#threefold>
    pub fn insert_check_draw(&mut self, board: &Board) -> bool {
        let occurances = self.insert(board);
        occurances == 3 || occurances >= 5
    }

    /// Remove the board to the repitition table.
    ///
    /// Returns the new count for the given board.
    pub fn remove(&mut self, board: &Board) -> usize {
        let zobrist = board.zobrist();

        if let Some(&count) = self.0.get(&zobrist) {
            let new_count = count.saturating_sub(1);

            if new_count == 0 {
                self.0.remove(&zobrist);
            } else {
                self.0.insert(zobrist, new_count);
            }

            new_count
        } else {
            0
        }
    }

    /// Get the number of times this position has been seen in the current line.
    #[allow(dead_code)]
    pub fn get(&self, board: &Board) -> usize {
        *self.0.get(&board.zobrist()).unwrap_or(&0)
    }
}
