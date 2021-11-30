use pleco::{BitMove, Board};

use crate::{
    stonefish::evaluation::Evaluation,
    uci::{
        uci::{StopFlag, UciEngine},
        uci_command::{UciGoConfig, UciPosition},
    },
};

use super::evaluatable::Evaluatable;

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

        let moves = self.board.generate_moves();

        // Apply every possible move to a new board
        let mut move_boards: Vec<(&BitMove, Board)> = moves
            .iter()
            .map(|mv| {
                let mut new_board = self.board.clone();
                new_board.apply_move(*mv);
                (mv, new_board)
            })
            .collect();

        move_boards.sort_by_key(|(_, board)| board.evaluate());

        let best_move = match self.board.turn() {
            pleco::Player::White => move_boards.last(),
            pleco::Player::Black => move_boards.first(),
        };

        if let Some((mv, board)) = best_move {
            // Calculate the score after the move was played, in centipawns
            let score = match board.evaluate() {
                Evaluation::Eval(eval) => eval * 100,
                Evaluation::Checkmate(_, player) => match player {
                    pleco::Player::White => 100000,
                    pleco::Player::Black => -100000,
                },
            };

            // Convert to the score from the engine's point of view
            let cp = match board.turn().other_player() {
                pleco::Player::White => score,
                pleco::Player::Black => -score,
            };

            println!("info pv {} score cp {}", mv.stringify(), cp);
            println!("bestmove {}", mv.stringify());
        }
    }
}
