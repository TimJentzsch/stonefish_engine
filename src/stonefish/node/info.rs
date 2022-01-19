use std::time::Duration;

use crate::stonefish::{evaluation::Evaluation, types::Line};

use super::Node;

impl Node {
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
            Evaluation::Centipawns(cp) => format!("cp {cp}"),
            Evaluation::Draw => format!("cp 0"),
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
