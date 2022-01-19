mod abort_flags;
mod evaluation;
mod heuristic;
mod node;
mod tables;
mod time_management;
mod types;

use pleco::Board;

use crate::{
    stonefish::node::Node,
    uci::{
        uci_command::{UciGoConfig, UciPosition},
        uci_option::{UciOption, UciOptionType},
        AbortFlag, UciEngine,
    },
};

use self::{tables::RepetitionTable, time_management::get_max_time};

#[derive(Debug, Clone)]
pub struct Stonefish {
    /// The board depicting the current position.
    board: Board,
    /// Table to track threefold repetion.
    repetition_table: RepetitionTable,
}

impl Stonefish {
    /// Create a new Stonefish instance.
    pub fn new() -> Stonefish {
        Stonefish {
            board: Board::start_pos(),
            repetition_table: RepetitionTable::new(),
        }
    }

    /// Try to reconstruct the move history from the last searched position.
    ///
    /// This is _not_ part of the UCI specification and should be provided
    /// directly via the `position` command.
    ///
    /// However, if the `moves` are not provided, we try to reconstruct
    /// the history from the last searched position.
    /// This will allow us to properly recognize threefold-repetitions.
    fn reconstruct_move_history(
        old_board: &Board,
        new_board: &Board,
        repetition_table: &mut RepetitionTable,
    ) {
        let new_zobrist = new_board.zobrist();

        // First, check if the same position is being searched again
        if old_board.zobrist() == new_zobrist {
            return;
        }

        // Second, look up to two moves ahead
        for mv_one in old_board.generate_moves() {
            let mut mv_one_board = old_board.clone();
            mv_one_board.apply_move(mv_one);

            if mv_one_board.zobrist() == new_zobrist {
                repetition_table.insert(&mv_one_board);
                return;
            }

            for mv_two in mv_one_board.generate_moves() {
                let mut mv_two_board = mv_one_board.clone();
                mv_two_board.apply_move(mv_two);

                if mv_two_board.zobrist() == new_zobrist {
                    repetition_table.insert(&mv_one_board);
                    repetition_table.insert(&mv_two_board);
                    return;
                }
            }
        }

        // Otherwise, fall back to a new position
        *repetition_table = RepetitionTable::new();
        repetition_table.insert(new_board);
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
        self.repetition_table = RepetitionTable::new();
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

        // We clone the table so that we can fall back to the old position
        // if parts of the moves are invalid
        let mut repetition_table = self.repetition_table.clone();

        if moves.len() == 0 {
            // No move history was provided, try to reconstruct it
            Self::reconstruct_move_history(&self.board, &new_board, &mut repetition_table);
        } else {
            self.repetition_table = RepetitionTable::new();

            // Try to apply the moves
            for move_str in moves {
                // Convert to lowercase to make sure it can be parsed
                if !new_board.apply_uci_move(move_str.to_lowercase().as_str()) {
                    // The move couldn't be applied, don't change the board
                    println!("info string '{}' is an invalid move string.", move_str);
                    return;
                }

                repetition_table.insert(&new_board);
            }
        }

        // Save the new position
        self.board = new_board;
        self.repetition_table = repetition_table;
    }

    fn go(&mut self, go_config: UciGoConfig, stop_flag: AbortFlag) {
        let mut root = Node::new(self.board.clone());

        // Determine search depth and time
        let max_depth = go_config.max_depth.or(go_config.search_mate);
        let max_time = get_max_time(go_config, root.board.turn());

        // Search for the best move
        root.iterative_deepening(
            max_depth,
            max_time,
            self.repetition_table.clone(),
            stop_flag,
        );
        root.send_best_move();
    }
}
