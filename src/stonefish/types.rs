use std::collections::HashMap;

use pleco::BitMove;

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