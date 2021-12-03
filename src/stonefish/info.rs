use pleco::BitMove;

use super::node::Node;

impl Node {
    /// Determine the number of nodes in the tree.
    pub fn size(&self) -> usize {
        if let Some(children) = &self.children {
            let mut size: usize = 1;

            for child in children {
                size += child.size();
            }

            size
        } else {
            1
        }
    }

    /// Determine the depth of the tree.
    pub fn depth(&self) -> usize {
        if let Some(children) = &self.children {
            let mut depth: usize = 0;

            for child in children {
                depth += depth.max(child.depth() + 1);
            }

            depth
        } else {
            0
        }
    }

    /// The best successor node.
    pub fn best_node(&self) -> Option<&Node> {
        if let Some(children) = &self.children {
            children.first()
        } else {
            None
        }
    }

    /// The best move to play.
    pub fn best_move(&self) -> Option<BitMove> {
        if let Some(node) = self.best_node() {
            node.board.last_move()
        } else {
            None
        }
    }
}
