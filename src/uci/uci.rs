//! Implementation of the Universal Chess Interface (UCI).
// Adapted from the Weiawaga engine, licensed GPL-2.0
// https://github.com/Heiaha/Weiawaga/blob/493d8139f882b89380c298457267cb059d86dc2f/src/uci/uci.rs
use std::io::BufRead;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{io, sync, thread};

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
    fn change_position(&mut self, _fen_str: String, _moves: Vec<String>) {}

    /// Start the search.
    fn go(&mut self) {}

    /// Set an option to the provided value.
    fn set_option(&mut self, _name: String, _value: String) {}
}

pub struct UciRunner;

impl UciRunner {
    fn thread_loop<Engine: UciEngine>(
        thread: sync::mpsc::Receiver<UciCommand>,
        _abort: Arc<AtomicBool>,
    ) {
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
                UciCommand::Position(fen_str, moves) => {
                    // Move to a new position
                    engine.change_position(fen_str, moves);
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
                    println!("info Unknown option {}={}", name, value);
                }
                _ => {
                    // Ignore unknown commands
                    println!("info Unknown command");
                }
            }
        }
    }

    pub fn run<Engine: UciEngine>() {
        let stdin = io::stdin();
        let lock = stdin.lock();

        let thread_moved_abort = sync::Arc::new(sync::atomic::AtomicBool::new(false));
        let abort = sync::Arc::clone(&thread_moved_abort);
        let (main_tx, main_rx) = sync::mpsc::channel();
        thread::Builder::new()
            .name("Main thread".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || Self::thread_loop::<Engine>(main_rx, thread_moved_abort))
            .unwrap();

        for line in lock.lines() {
            let cmd = UciCommand::from(&*line.unwrap().to_owned());
            match cmd {
                UciCommand::Quit => return,
                UciCommand::Stop => {
                    abort.store(true, Ordering::SeqCst);
                }
                cmd => {
                    main_tx.send(cmd).unwrap();
                }
            }
        }
    }
}

pub enum UciCommand {
    Unknown(String),
    UciNewGame,
    Uci,
    IsReady,
    Position(String, Vec<String>),
    Go(String),
    Quit,
    Stop,
    Perft(u8),
    Option(String, String),
}

impl From<&str> for UciCommand {
    fn from(line: &str) -> Self {
        if line.starts_with("ucinewgame") {
            return UciCommand::UciNewGame;
        } else if line.starts_with("setoption") {
            let mut words = line.split_whitespace();
            let mut name_parts = Vec::new();
            let mut value_parts = Vec::new();

            // parse option name
            while let Some(word) = words.next() {
                if word == "value" {
                    break;
                } else {
                    name_parts.push(word);
                }
            }
            for word in words {
                value_parts.push(word);
            }
            let name = name_parts.last().unwrap();
            let value = value_parts.last().unwrap_or(&"");
            return UciCommand::Option(name.parse().unwrap(), value.parse().unwrap());
        } else if line.starts_with("uci") {
            return UciCommand::Uci;
        } else if line.starts_with("isready") {
            return UciCommand::IsReady;
        } else if line.starts_with("go") {
            return UciCommand::Go("".to_owned());
        } else if line.starts_with("position") {
            let mut moves = Vec::new();
            if line.contains("moves") {
                if let Some(moves_) = line.split_terminator("moves ").nth(1) {
                    for mov in moves_.split_whitespace() {
                        moves.push(String::from(mov));
                    }
                }
            }
            return UciCommand::Position("".to_owned(), moves);
        } else if line.starts_with("quit") {
            return UciCommand::Quit;
        } else if line.starts_with("perft") {
            let depth = line
                .split_whitespace()
                .nth(1)
                .and_then(|d| d.parse().ok())
                .unwrap_or(6);
            return UciCommand::Perft(depth);
        } else if line == "stop" {
            return UciCommand::Stop;
        }
        Self::Unknown(line.to_owned())
    }
}
