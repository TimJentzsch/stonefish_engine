use std::time::Duration;

use pleco::BitMove;

use crate::stonefish::evaluation::Evaluation;

use super::Node;

pub type Line = Vec<BitMove>;
pub type Children = Vec<Node>;

impl Node {
    /// Update the attributes of the node.
    ///
    /// This should always be called after the children have been modified.
    pub fn update_attributes(&mut self, children: &Children) {
        let mut size: usize = 1;
        let mut depth: usize = children.first().map_or(0, |child| child.depth + 1);
        let mut sel_depth: usize = 0;
        let mut best_child: Option<&Node> = None;

        for child in children {
            size += child.size;
            depth = depth.min(child.depth + 1);
            sel_depth = sel_depth.max(child.sel_depth + 1);

            best_child = if let Some(prev_best) = best_child {
                if child.evaluation.for_opponent().previous_plie()
                    > prev_best.evaluation.for_opponent().previous_plie()
                {
                    Some(child)
                } else {
                    Some(prev_best)
                }
            } else {
                Some(child)
            }
        }

        self.size = size;
        self.depth = depth;
        self.sel_depth = sel_depth;

        if let Some(best_child) = best_child {
            // The evaluation of the node is the evaluation of the best child
            self.evaluation = best_child.evaluation.for_opponent().previous_plie();
            // The best line to play is the best child and its line
            let mv = best_child.board.last_move().unwrap();
            let mut best_line = best_child.best_line.clone();
            best_line.splice(0..0, [mv]);

            self.best_line = best_line;
        } else {
            self.best_line = vec![];
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
        if let Some(mv) = self.best_line.first() {
            println!("bestmove {}", mv.stringify());
        }
    }

    /// Send info about the current position to the engine.
    pub fn send_info(&self, duration: Duration) {
        // The evaluation of the current position
        let score = match self.evaluation {
            Evaluation::Centipawns(cp) => format!("cp {}", cp),
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
            // Seldepth
            self.sel_depth,
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
            Self::format_line(&self.best_line),
        );
    }
}

#[cfg(test)]
mod tests {
    use pleco::Board;

    use crate::stonefish::{
        evaluation::Evaluation,
        node::{minimax::HashTable, Node},
    };

    #[test]
    fn should_update_attributes_startpos() {
        let mut startpos = Node::new(Board::start_pos());

        assert_eq!(startpos.size, 1);
        assert_eq!(startpos.depth, 0);
        assert_eq!(startpos.sel_depth, 0);
        assert_eq!(startpos.best_line.len(), 0);

        let children = startpos.expand(&HashTable::new());

        for child in children {
            assert_eq!(child.size, 1);
            assert_eq!(child.depth, 0);
            assert_eq!(child.sel_depth, 0);
            assert_eq!(child.best_line.len(), 0);
        }

        // 20 moves are possible, plus the root node
        assert_eq!(startpos.size, 21);
        assert_eq!(startpos.depth, 1);
        assert_eq!(startpos.sel_depth, 1);
        assert_eq!(startpos.best_line.len(), 1);
    }

    #[test]
    fn should_update_attributes_forced_mate_0() {
        let mut pos = Node::new(
            Board::from_fen("1Nb4r/p2p3p/kb1P3n/3Q4/N3Pp2/8/P1P3PP/7K b - - 0 2").unwrap(),
        );

        assert_eq!(pos.size, 1);
        assert_eq!(pos.depth, 0);
        assert_eq!(pos.sel_depth, 0);
        assert_eq!(pos.best_line.len(), 0);
        assert_eq!(pos.evaluation, Evaluation::OpponentCheckmate(0));

        let children = pos.expand(&HashTable::new());
        assert_eq!(children.len(), 0);

        assert_eq!(pos.depth, 0);
        assert_eq!(pos.sel_depth, 0);
        assert_eq!(pos.best_line.len(), 0);
        assert_eq!(pos.evaluation, Evaluation::OpponentCheckmate(0));
    }

    #[test]
    fn should_update_attributes_forced_mate_1() {
        let mut pos = Node::new(
            Board::from_fen("1rb4r/p1Pp3p/kb1P3n/3Q4/N3Pp2/8/P1P3PP/7K w - - 3 2").unwrap(),
        );

        assert_eq!(pos.size, 1);
        assert_eq!(pos.depth, 0);
        assert_eq!(pos.sel_depth, 0);
        assert_eq!(pos.best_line.len(), 0);
        assert!(!pos.evaluation.is_forced_mate());

        let children = pos.expand(&HashTable::new());

        for child in children {
            assert_eq!(child.size, 1);
            assert_eq!(child.depth, 0);
            assert_eq!(child.sel_depth, 0);
            assert_eq!(child.best_line.len(), 0);
        }

        assert_eq!(pos.depth, 1);
        assert_eq!(pos.sel_depth, 1);
        assert_eq!(pos.best_line.len(), 1);
        assert_eq!(pos.evaluation, Evaluation::PlayerCheckmate(1));
    }
}
