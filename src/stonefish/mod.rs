mod abort_flags;
mod evaluation;
mod heuristic;
mod node;

use std::time::Duration;

use pleco::Board;

use crate::{
    stonefish::node::Node,
    uci::{
        uci_command::{UciGoConfig, UciPosition},
        uci_option::{UciOption, UciOptionType},
        AbortFlag, UciEngine,
    },
};

#[derive(Debug, Clone)]
pub struct Stonefish {
    /// The board depicting the current position.
    board: Board,
}

impl Stonefish {
    /// Create a new Stonefish instance.
    pub fn new() -> Stonefish {
        Stonefish {
            board: Board::start_pos(),
        }
    }
}

impl UciEngine for Stonefish {
    fn new() -> Self {
        Stonefish::new()
    }

    fn get_name(&self) -> Option<&str> {
        Some("Stonefish")
    }

    fn get_author(&self) -> Option<&str> {
        Some("Tim3303")
    }

    fn get_options(&self) -> Vec<UciOption> {
        vec![
            // We don't use this yet, but it's mandatory for some GUIs
            UciOption::new_with_default("Hash", UciOptionType::Spin, "32"),
            // We don't change behavior, but we wanna do analysis
            UciOption::new("UCI_AnalyseMode", UciOptionType::Check),
        ]
    }

    fn new_game(&mut self) {
        // Reset the board
        self.board = Board::start_pos();
    }

    fn change_position(&mut self, pos: UciPosition, moves: Vec<String>) {
        // Try to apply the position
        let mut new_board = match pos {
            UciPosition::Startpos => Board::start_pos(),
            UciPosition::Fen(fen_str) => {
                if let Ok(parsed_board) = Board::from_fen(fen_str.as_str()) {
                    parsed_board
                } else {
                    // The FEN string couldn't be parsed, don't change the board
                    println!("info string '{}' is an invalid FEN string.", fen_str);
                    return;
                }
            }
        };

        // Try to apply the moves
        for move_str in moves {
            // Convert to lowercase to make sure it can be parsed
            if !new_board.apply_uci_move(move_str.to_lowercase().as_str()) {
                // The move couldn't be applied, don't change the board
                println!("info string '{}' is an invalid move string.", move_str);
                return;
            }
        }

        // Save the new board
        self.board = new_board;
    }

    fn go(&mut self, go_config: UciGoConfig, stop_flag: AbortFlag) {
        let mut root = Node::new(self.board.clone());

        // Determine search depth if one is given
        let max_depth = if let Some(max_depth) = go_config.max_depth {
            Some(max_depth)
        } else {
            go_config.search_mate
        };

        // Determine player time and increment
        let (time, increment) = match root.board.turn() {
            pleco::Player::White => (go_config.white_time_ms, go_config.white_increment_ms),
            pleco::Player::Black => (go_config.black_time_ms, go_config.black_increment_ms),
        };

        // Determine maximum time
        let max_time = if let Some(move_time_ms) = go_config.move_time_ms {
            Some(Duration::from_millis(move_time_ms.try_into().unwrap()))
        } else if go_config.infinite {
            // Search infinitely
            None
        } else if let Some(time_ms) = time {
            // Take 5 seconds reserve time for each move
            let base_time_ms: u64 = 5000.min(time_ms.try_into().unwrap());
            // Additionally use the increment time
            let increment_time_ms: u64 = increment.try_into().unwrap();
            let mut total_time_ms = base_time_ms + increment_time_ms;
            // Consider a delay of 100 ms and cap at 7 seconds
            total_time_ms = total_time_ms.saturating_sub(100).min(7000);
            Some(Duration::from_millis(total_time_ms))
        } else if max_depth.is_some() {
            None
        } else {
            // Search for 10 seconds
            Some(Duration::from_millis(10000))
        };

        // Search for the best move
        root.iterative_deepening(max_depth, max_time, stop_flag);
        root.send_best_move();
    }
}
