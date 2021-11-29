//! Implementation of the Universal Chess Interface (UCI).
// Adapted from the Weiawaga engine, licensed GPL-2.0
// https://github.com/Heiaha/Weiawaga/blob/493d8139f882b89380c298457267cb059d86dc2f/src/uci/uci.rs
use std::io::BufRead;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{io, sync, thread};

pub enum UCICommand {
    Unknown(String),
    UCINewGame,
    UCI,
    IsReady,
    Position(String, Vec<String>),
    Go(String),
    Quit,
    Stop,
    Perft(u8),
    Option(String, String),
}



impl UCICommand {
    fn thread_loop(thread: sync::mpsc::Receiver<UCICommand>, abort: Arc<AtomicBool>) {
        for cmd in thread {
            match cmd {
                UCICommand::IsReady => {
                    println!("readyok");
                }
                UCICommand::UCINewGame => {
                    todo!();
                }
                UCICommand::Position(new_board, moves) => {
                    todo!();
                }
                UCICommand::Go(time_control) => {
                    todo!();
                }
                UCICommand::Perft(depth) => {
                    todo!();
                }
                UCICommand::Option(name, value) => {
                    todo!();
                },
                _ => {
                    // Ignore unknown commands
                }
            }
        }
    }

    pub fn run() {
        let stdin = io::stdin();
        let lock = stdin.lock();

        let thread_moved_abort = sync::Arc::new(sync::atomic::AtomicBool::new(false));
        let abort = sync::Arc::clone(&thread_moved_abort);
        let (main_tx, main_rx) = sync::mpsc::channel();
        let builder = thread::Builder::new()
            .name("Main thread".into())
            .stack_size(8 * 1024 * 1024);
        let thread = builder
            .spawn(move || Self::thread_loop(main_rx, thread_moved_abort))
            .unwrap();

        for line in lock.lines() {
            let cmd = UCICommand::from(&*line.unwrap().to_owned());
            match cmd {
                UCICommand::Quit => return,
                UCICommand::Stop => {
                    abort.store(true, Ordering::SeqCst);
                }
                UCICommand::UCI => {
                    println!("id name Stonefish");
                    println!("id author Tim3303");
                    println!("uciok");
                }
                cmd => {
                    main_tx.send(cmd).unwrap();
                }
            }
        }
    }
}

impl From<&str> for UCICommand {
    fn from(line: &str) -> Self {
        if line.starts_with("ucinewgame") {
            return UCICommand::UCINewGame;
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
            return UCICommand::Option(name.parse().unwrap(), value.parse().unwrap());
        } else if line.starts_with("uci") {
            return UCICommand::UCI;
        } else if line.starts_with("isready") {
            return UCICommand::IsReady;
        } else if line.starts_with("go") {
            return UCICommand::Go("".to_owned());
        } else if line.starts_with("position") {
            let mut moves = Vec::new();
            if line.contains("moves") {
                if let Some(moves_) = line.split_terminator("moves ").nth(1) {
                    for mov in moves_.split_whitespace() {
                        moves.push(String::from(mov));
                    }
                }
            }
            return UCICommand::Position("".to_owned(), moves);
        } else if line.starts_with("quit") {
            return UCICommand::Quit;
        } else if line.starts_with("perft") {
            let depth = line
                .split_whitespace()
                .nth(1)
                .and_then(|d| d.parse().ok())
                .unwrap_or(6);
            return UCICommand::Perft(depth);
        } else if line == "stop" {
            return UCICommand::Stop;
        }
        Self::Unknown(line.to_owned())
    }
}
