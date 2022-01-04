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
        let mut depth: usize = 0;
        let mut best_child: Option<&Node> = None;

        for child in children {
            size += child.size;
            depth = depth.max(child.depth + 1);

            best_child = if let Some(prev_best) = best_child {
                if child.evaluation > prev_best.evaluation {
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
        
        if let Some(best_child) = best_child {
            self.evaluation = best_child.evaluation;
            let mv = best_child.board.last_move().unwrap();
            let mut best_line = best_child.best_line.clone();
            best_line.splice(0..0, [mv]);
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
            Self::format_line(&self.best_line),
        );
    }
}
