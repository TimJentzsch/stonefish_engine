use pleco::Board;

use crate::uci::uci::UciEngine;

#[derive(Debug, Clone)]
pub struct Stonefish {
    /// The board depicting the current position.
    board: Board,
}

impl Stonefish {
    /// Create a new Stonefish instance.
    pub fn new() -> Stonefish {
        Stonefish {
            board: Board::start_pos()
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

    fn change_position(&mut self, fen_str: String, moves: Vec<String>) {
        // Try to convert the FEN in a position
        if let Ok(mut new_board) = Board::from_fen(fen_str.as_str()) {

            // Try to apply the moves
            for move_str in moves {
                if !new_board.apply_uci_move(move_str.as_str()) {
                    // The move couldn't be applied, don't change the board
                    println!("info string '{}' is an invalid move string.", move_str);
                    return;
                }
            }

            println!("info string change_position");

            self.board = new_board;
        } else {
            // The FEN string couldn't be parsed, don't change the board
            println!("info string '{}' is an invalid FEN string.", fen_str);
        }
    }

    fn go(&mut self) {
        println!("info string go");
    }
}
