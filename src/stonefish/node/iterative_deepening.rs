use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

use crate::{stonefish::evaluation::Evaluation, uci::uci::StopFlag};

use super::{Node, minimax::HashTable};

impl Node {
    /// Set a timer to abort the search.
    ///
    /// This function will set the time flag to true once the time runs out.
    fn set_timer(max_time: Option<Duration>, time_flag: StopFlag) {
        if let Some(max_time) = max_time {
            // Start a new thread so that we don't block the main thread
            thread::spawn(move || {
                // Wait for the given time
                thread::sleep(max_time);
                // Set the time flag to true
                time_flag.store(true, Ordering::SeqCst);
            });
        }
    }

    /// The iterative deepening search algorithm.
    pub fn iterative_deepening(
        &mut self,
        max_depth: Option<usize>,
        max_time: Option<Duration>,
        stop_flag: StopFlag,
    ) -> Evaluation {
        let start = Instant::now();
        // When this flag is set to true, time has run out
        let time_flag: StopFlag = Arc::new(AtomicBool::new(false));
        Self::set_timer(max_time, time_flag.clone());

        let mut depth: usize = 1;

        let mut eval = self.evaluation;

        // Search at higher and higher depths
        loop {
            if let Some(max_depth) = max_depth {
                if depth > max_depth {
                    break;
                }
            }

            let mut hash_table = HashTable::new();

            // Search at the current depth and update the evaluation
            if let Ok(new_eval) = self.minimax(depth, &mut hash_table, stop_flag.clone(), time_flag.clone()) {
                eval = new_eval;
            } else {
                // Abort the search
                break;
            }

            println!("info string Hash size: {}", hash_table.len());

            // Update the GUI on the current evaluation
            self.send_info(start.elapsed());
            depth += 1;
        }

        eval
    }
}
