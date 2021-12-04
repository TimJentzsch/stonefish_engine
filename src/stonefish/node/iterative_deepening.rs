use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use crate::{stonefish::evaluation::Evaluation, uci::uci::StopFlag};

use super::Node;

impl Node {
    pub fn iterative_deepening(
        &mut self,
        max_depth: Option<usize>,
        max_time: Option<Duration>,
        stop_flag: StopFlag,
    ) -> Evaluation {
        // When this flag is set to true, time has run out
        let time_flag: StopFlag = Arc::new(AtomicBool::new(false));

        let mut depth: usize = 1;

        let mut eval = self.evaluation;

        // Search at higher and higher depths
        loop {
            if let Some(max_depth) = max_depth {
                if depth > max_depth {
                    break;
                }
            }

            // Search at the current depth and update the evaluation
            if let Ok(new_eval) = self.minimax(depth, stop_flag.clone(), time_flag.clone()) {
                eval = new_eval;
            } else {
                // Abort the search
                break;
            }

            // Update the GUI on the current evaluation
            self.send_info();
            depth += 1;
        }

        eval
    }
}
