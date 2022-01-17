use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc,
    },
    thread,
    time::{Duration, Instant},
};

use crate::{
    stonefish::{
        abort_flags::AbortFlags,
        evaluation::Evaluation,
        types::{HashTable, RepetitionTable},
    },
    uci::AbortFlag,
};

use super::Node;

impl Node {
    /// Set a timer to abort the search.
    ///
    /// This function will set the time flag to true once the time runs out.
    fn set_timer(max_time: Option<Duration>, time_flag: AbortFlag) {
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
        repetition_table: RepetitionTable,
        stop_flag: AbortFlag,
    ) -> Evaluation {
        let start = Instant::now();
        // When this flag is set to true, time has run out
        let time_flag: AbortFlag = Arc::new(AtomicBool::new(false));
        Self::set_timer(max_time, time_flag.clone());

        let mut depth: usize = 1;

        // Search at higher and higher depths
        loop {
            if let Some(max_depth) = max_depth {
                if depth > max_depth {
                    break;
                }
            }

            let (tx, rx) = mpsc::channel();

            let mut node = self.clone();
            let children = node.reset().expand(&HashTable::new());

            // Search every move in a separate thread
            for child in &children {
                let tx = tx.clone();
                let mut child = child.clone();

                let mut hash_table = HashTable::new();
                let mut repetition_table = repetition_table.clone();
                if repetition_table.insert_check_draw(&self.board) {
                    repetition_table.remove(&self.board);
                    child.evaluation = Evaluation::DRAW;
                    tx.send((child, Ok(Evaluation::DRAW))).unwrap();
                    continue;
                }

                let abort_flags = AbortFlags::from_flags(stop_flag.clone(), time_flag.clone());

                thread::Builder::new()
                    .name(child.board.last_move().unwrap().stringify())
                    .spawn(move || {
                        let result = child.minimax(
                            depth - 1,
                            &mut hash_table,
                            &mut repetition_table,
                            abort_flags,
                        );
                        tx.send((child, result)).unwrap();
                    })
                    .unwrap();
            }

            let mut updated_children = vec![];
            let mut abort = false;

            // Aggregate the results
            for _ in &children {
                let (child, result) = rx.recv().unwrap();
                if result.is_err() {
                    abort = true;
                }
                updated_children.push(child);
            }

            if !abort {
                // Update the node with the new evaluation
                node.update_attributes(&updated_children);
                self.copy_values(&node);
            }

            // Update the GUI on the current evaluation
            self.send_info(start.elapsed());
            depth += 1;

            // If the time is limited and there is a forced mate, just play it out
            let play_forced_mate =
                self.evaluation.is_forced_mate() && (max_depth.is_some() || max_time.is_some());

            if abort || play_forced_mate {
                break;
            }
        }

        self.evaluation
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{atomic::AtomicBool, Arc};

    use pleco::Board;

    use crate::stonefish::{evaluation::Evaluation, node::Node, types::RepetitionTable};

    fn assert_forced_mate(fen: &str, plies: usize) {
        let board = Board::from_fen(fen).unwrap();
        let mut node = Node::new(board);
        node.iterative_deepening(
            Some(plies),
            None,
            RepetitionTable::new(),
            Arc::new(AtomicBool::new(false)),
        );

        assert_eq!(
            node.evaluation,
            Evaluation::PlayerCheckmate(plies),
            "fen: {}, eval: {:?}",
            fen,
            node.evaluation
        );
    }

    #[test]
    fn should_solve_mate_in_1_puzzles() {
        let puzzle_fens = [
            "r2qkb1r/pp2np1p/3p1p2/2p1N1B1/2BnP3/3P4/PPP2PPP/R2bK2R w KQkq - 0 2",
            "1rb4r/p1Pp3p/kb1P3n/3Q4/N3Pp2/8/P1P3PP/7K w - - 3 2",
            "1n2kb1r/p4ppp/4q3/4p1B1/4P3/8/PPP2PPP/2KR4 w k - 0 2",
        ];

        for fen in puzzle_fens {
            assert_forced_mate(fen, 1);
        }
    }

    #[test]
    fn should_solve_mate_in_2_puzzles() {
        // See https://wtharvey.com/m8n2.txt
        let puzzle_fens = [
            "1rb4r/pkPp3p/1b1P3n/1Q6/N3Pp2/8/P1P3PP/7K w - - 1 1",
            "4kb1r/p2n1ppp/4q3/4p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 1 1",
            "r1b2k1r/ppp1bppp/8/1B1Q4/5q2/2P5/PPP2PPP/R3R1K1 w - - 1 1",
            "r1b2k1r/ppp1bppp/8/1B1Q4/5q2/2P5/PPP2PPP/R3R1K1 w - - 1 1",
            "5rkr/pp2Rp2/1b1p1Pb1/3P2Q1/2n3P1/2p5/P4P2/4R1K1 w - - 1 1",
            "1r1kr3/Nbppn1pp/1b6/8/6Q1/3B1P2/Pq3P1P/3RR1K1 w - - 1 1",
            "5rk1/1p1q2bp/p2pN1p1/2pP2Bn/2P3P1/1P6/P4QKP/5R2 w - - 1 1",
        ];

        for fen in puzzle_fens {
            assert_forced_mate(fen, 3);
        }
    }

    #[test]
    fn should_solve_mate_in_3_puzzles() {
        // See https://wtharvey.com/m8n3.txt
        let puzzle_fens = [
            "r1b1kb1r/pppp1ppp/5q2/4n3/3KP3/2N3PN/PPP4P/R1BQ1B1R b kq - 0 1",
            "r3k2r/ppp2Npp/1b5n/4p2b/2B1P2q/BQP2P2/P5PP/RN5K w kq - 1 1",
            "r1b3kr/ppp1Bp1p/1b6/n2P4/2p3q1/2Q2N2/P4PPP/RN2R1K1 w - - 1 1",
        ];

        for fen in puzzle_fens {
            assert_forced_mate(fen, 5);
        }
    }

    #[test]
    fn should_not_wrongly_assume_mate() {
        let paramerters = [
            ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 3),
            ("8/r1N1k1pp/b4p2/3Qp3/P6q/4P3/2PP1PPP/R3K2R w KQ - 1 25", 3),
            // TODO: Fix mate here
            // ("4r3/1b2rpk1/p6R/1p4p1/2pN4/P1P2P2/1P3KP1/R7 w - - 2 29", 5),
        ];

        for (fen, depth) in paramerters {
            let mut node = Node::new(Board::from_fen(fen).unwrap());
            node.iterative_deepening(
                Some(depth),
                None,
                RepetitionTable::new(),
                Arc::new(AtomicBool::new(false)),
            );

            assert!(
                !node.evaluation.is_forced_mate(),
                "'{}': {:?}",
                fen,
                node.evaluation
            );
        }
    }
}
