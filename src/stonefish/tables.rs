use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use pleco::Board;

use super::{evaluation::Evaluation, node::Node, types::Line};

#[derive(Debug, Clone)]
pub struct RepetitionTable(HashMap<u64, usize>);

impl RepetitionTable {
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct HashTable(Arc<Mutex<HashMap<u64, HashTableEntry>>>);

impl HashTable {
    pub fn new() -> Self {
        HashTable(Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn insert(&mut self, node: &Node) {
        let mut map = self.0.lock().unwrap();
        map.insert(node.board.zobrist(), HashTableEntry::from_node(node));
    }

    pub fn get(self, board: &Board) -> Option<HashTableEntry> {
        let map = self.0.lock().unwrap();
        map.get(&board.zobrist()).map(|entry| entry.clone())
    }
}
