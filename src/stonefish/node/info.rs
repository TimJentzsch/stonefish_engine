use std::time::Duration;

use pleco::BitMove;

use crate::stonefish::evaluation::Evaluation;

use super::Node;

pub type Line = Vec<BitMove>;

impl Node {
    /// Update the size attribute of the node.
    /// 
    /// This should always be called after the children have been modified.
    fn update_size(&mut self) {
        if let Some(children) = &self.children {
            let mut size: usize = 1;

            for child in children {
                size += child.size;
            }

            self.size = size;
        } else {
            self.size = 1
        }
    }

    /// Update the depth attribute of the node.
    /// 
    /// This should always be called after the children have been modified.
    fn update_depth(&mut self) {
        if let Some(children) = &self.children {
            let mut depth: usize = 0;

            for child in children {
                depth = depth.max(child.depth + 1);
            }

            self.depth = depth;
        } else {
            self.depth = 0;
        }
    }

    /// Update the attributes of the node.
    /// 
    /// This should always be called after the children have been modified.
    pub fn update_attributes(&mut self) {
        self.update_size();
        self.update_depth();
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

    /// The best line to play.
    pub fn best_line(&self) -> Line {
        if let Some(node) = self.best_node() {
            let best_move = vec![self.best_move().unwrap()];
            let best_child_line = node.best_line();
            let best_line = vec![best_move, best_child_line].concat();
            best_line
        } else {
            vec![]
        }
    }

    /// Format a line of moves.
    fn format_line(line: &Line) -> String {
        line.iter()
            .map(|mv| mv.stringify())
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Send the best move to the engine.
    pub fn send_best_move(&self) {
        if let Some(mv) = self.best_move() {
            println!("bestmove {}", mv.stringify());
        }
    }

    /// Send info about the current position to the engine.
    pub fn send_info(&self, duration: Duration) {
        // The evaluation of the current position
        let score = match self.evaluation {
            Evaluation::Material(cp) => format!("cp {}", cp),
            Evaluation::PlayerCheckmate(plies) => {
                // Convert plies to moves
                format!("mate {}", (plies as f32 / 2.0).ceil() as i32)
            }
            Evaluation::OpponentCheckmate(plies) => {
                // Convert plies to moves
                format!("mate {}", -((plies as f32 / 2.0).ceil() as i32))
            }
        };

        // Example from Stockfish:
        // info depth 1 seldepth 1 multipv 1 score cp 112 nodes 20 nps 20000 tbhits 0 time 1 pv e2e4
        println!(
            "info depth {} seldepth {} multipv {} score {} nodes {} nps {} tbhits {} time {} pv {}  ",
            // Depth
            self.depth,
            // Seldepth (we search the same depth for all moves)
            self.depth,
            // Multi PV (we can only show one line at a time at the moment)
            1,
            // Score
            score,
            // Nodes
            self.size,
            // Nps
            self.size as u64 / duration.as_secs().max(1),
            // Tbhits (not implemented yet)
            0,
            // Time
            duration.as_millis(),
            // Pv
            Self::format_line(&self.best_line()),
        );
    }
}
