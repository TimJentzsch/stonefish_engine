use pleco::Board;

use crate::{
    stonefish::node::Node,
    uci::{
        uci::{StopFlag, UciEngine},
        uci_command::{UciGoConfig, UciPosition}, uci_option::{UciOption, UciOptionType},
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
            if !new_board.apply_uci_move(move_str.as_str()) {
                // The move couldn't be applied, don't change the board
                println!("info string '{}' is an invalid move string.", move_str);
                return;
            }
        }

        // Save the new board
        self.board = new_board;
    }

    fn go(&mut self, go_config: UciGoConfig, stop_flag: StopFlag) {
        let mut root = Node::new(self.board.clone());

        // Determine search depth if one is given
        let max_depth = if let Some(max_depth) = go_config.max_depth {
            Some(max_depth)
        } else if let Some(search_mate) = go_config.search_mate {
            Some(search_mate)
        } else {
            None
        };

        // Search for the best move
        root.iterative_deepening(max_depth, stop_flag);

        root.send_info();
        root.send_best_move();
    }
}
