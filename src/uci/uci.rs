//! Implementation of the Universal Chess Interface (UCI).
use std::io::BufRead;
use std::{io, sync, thread};

use super::uci_command::{UciCommand, UciPosition};

pub trait UciEngine {
    /// Create a new engine instance.
    fn new() -> Self
    where
        Self: Sized;

    /// The name of the engine.
    fn get_name(&self) -> Option<&str> {
        None
    }

    /// The author of the engine.
    fn get_author(&self) -> Option<&str> {
        None
    }

    /// Create a new game.
    fn new_game(&mut self) {}

    /// Move to a new position.
    fn change_position(&mut self, _pos: UciPosition, _moves: Vec<String>) {}

    /// Start the search.
    fn go(&mut self) {}

    /// Set an option to the provided value.
    fn set_option(&mut self, _name: String, _value: String) {}
}

pub struct UciRunner;

impl UciRunner {
    fn engine_loop<Engine: UciEngine>(thread: sync::mpsc::Receiver<UciCommand>) {
        // Create a new instance of the engine
        let mut engine = Engine::new();

        for cmd in thread {
            match cmd {
                UciCommand::IsReady => {
                    // Always return readyok as soon as possible
                    println!("readyok");
                }
                UciCommand::Uci => {
                    // Let the GUI now that UCI is supported
                    // Also provide basic info about the engine
                    if let Some(name) = engine.get_name() {
                        println!("id name {}", name);
                    }
                    if let Some(author) = engine.get_author() {
                        println!("id author {}", author);
                    }
                    println!("uciok");
                }
                UciCommand::UciNewGame => {
                    // Create a new game
                    engine.new_game();
                }
                UciCommand::Position(pos, moves) => {
                    // Move to a new position
                    engine.change_position(pos, moves);
                }
                UciCommand::Go(_time_control) => {
                    // Start the search
                    engine.go();
                }
                UciCommand::Perft(_depth) => {
                    // TODO: Implement Perft
                }
                UciCommand::Option(name, value) => {
                    // Ignore options for now
                    println!("info string Unknown option {}={}", name, value);
                }
                _ => {
                    // Ignore unknown commands
                    println!("info string Unknown command");
                }
            }
        }
    }

    pub fn run<Engine: UciEngine>() {
        let stdin = io::stdin();
        let lock = stdin.lock();

        let (main_tx, main_rx) = sync::mpsc::channel();
        thread::Builder::new()
            .name("Engine thread".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || Self::engine_loop::<Engine>(main_rx))
            .unwrap();

        // Wait for new commands. Every command is a new line
        for line in lock.lines() {
            if let Ok(line_str) = line {
                // Parse the UCI command
                let cmd = UciCommand::from(line_str.as_str());
                match cmd {
                    // Quit the program
                    UciCommand::Quit => return,
                    // Propagate commands to the engine
                    cmd => {
                        main_tx.send(cmd).unwrap();
                    }
                }
            }
        }
    }
}
