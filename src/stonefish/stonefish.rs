use crate::uci::uci::UciEngine;

pub struct Stonefish;

impl UciEngine for Stonefish {
    fn new_game(&self) {
        println!("info new_game");
    }

    fn change_position(&self, fen_str: String, moves: Vec<String>) {
        println!("info change_position");
    }

    fn go(&self) {
        println!("info go");
    }
}
