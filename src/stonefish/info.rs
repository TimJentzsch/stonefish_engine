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
}
