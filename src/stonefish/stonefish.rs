use pleco::Board;

use crate::{
    stonefish::{evaluation::Evaluation, node::Node},
    uci::{
        uci::{StopFlag, UciEngine},
        uci_command::{UciGoConfig, UciPosition},
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

    fn new_game(&mut self) {
        println!("info string new_game");
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

        println!("info string Changed position to {}", new_board.fen());

        // Save the new board
        self.board = new_board;
    }

    fn go(&mut self, _go_config: UciGoConfig, _stop_flag: StopFlag) {
        println!("info string go");

        let mut root = Node::new(self.board.clone());
        root.expand();

        if let Some(node) = root.children.unwrap().first() {
            let mv = node.state.last_move().unwrap();
            // The current score evaluation from the engine's point of view
            let score = match node.evaluation.for_other_player() {
                Evaluation::Material(cp) => format!("cp {}", cp),
                Evaluation::PlayerCheckmate(moves) => format!("mate {}", moves),
                Evaluation::OpponentCheckmate(moves) => format!("mate {}", -(moves as i32)),
            };

            println!("info pv {} score {}", mv.stringify(), score);
            println!("bestmove {}", mv.stringify());
        }
    }
}
