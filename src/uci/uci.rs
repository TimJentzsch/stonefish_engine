//! Implementation of the Universal Chess Interface (UCI).
use std::io::BufRead;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{io, sync, thread};

use super::uci_command::{UciCommand, UciGoConfig, UciPosition};
use super::uci_option::UciOption;

pub type AbortFlag = Arc<AtomicBool>;

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

    /// The options available in the engine.
    fn get_options(&self) -> Vec<UciOption> {
        vec![]
    }

    /// Set debugging mode on or off.
    fn set_debug(&mut self, _debug: bool) {}

    /// Set an option to the provided value.
    fn set_option(&mut self, _name: String, _value: Option<String>) {}

    /// Create a new game.
    fn new_game(&mut self) {}

    /// Move to a new position.
    fn change_position(&mut self, _pos: UciPosition, _moves: Vec<String>) {}

    /// Start the search.
    fn go(&mut self, _go_config: UciGoConfig, _stop_flag: AbortFlag) {}

    /// Stop calculating as soon as possible.
    fn stop(&mut self) {}

    /// The user has played the expected move.
    ///
    /// This will be sent if the engine was told to ponder on the same move the user has played.
    /// The engine should continue searching but switch from pondering to normal search.
    fn ponder_hit(&mut self) {}
}

pub struct UciRunner;

impl UciRunner {
    fn engine_loop<Engine: UciEngine>(
        thread: sync::mpsc::Receiver<UciCommand>,
        stop_flag: AbortFlag,
    ) {
        // Create a new instance of the engine
        let mut engine = Engine::new();

        for cmd in thread {
            match cmd {
                UciCommand::Uci => {
                    // Let the GUI now that UCI is supported
                    // Also provide basic info about the engine
                    if let Some(name) = engine.get_name() {
                        println!("id name {}", name);
                    }
                    if let Some(author) = engine.get_author() {
                        println!("id author {}", author);
                    }
                    for option in engine.get_options() {
                        option.send_option();
                    }
                    println!("uciok");
                }
                // Set debug mode
                UciCommand::Debug(debug) => engine.set_debug(debug),
                // Always return readyok as soon as possible
                UciCommand::IsReady => println!("readyok"),
                // Set an option value
                UciCommand::SetOption(name, value) => engine.set_option(name, value),
                // Create a new game
                UciCommand::UciNewGame => engine.new_game(),
                // Move to a new position
                UciCommand::Position(pos, moves) => engine.change_position(pos, moves),
                // Start the search
                UciCommand::Go(go_config) => engine.go(go_config, stop_flag.clone()),
                // Stop the search as soon as possible
                UciCommand::Stop => engine.stop(),
                // The user has played the expected move
                UciCommand::Ponderhit => engine.ponder_hit(),
                // Ignore unknown commands
                UciCommand::Unknown(command_str) => {
                    println!("info string Unknown command '{}'", command_str);
                }
                _ => (),
            }
        }
    }

    pub fn run<Engine: UciEngine>() {
        let stdin = io::stdin();
        let lock = stdin.lock();

        let (main_tx, main_rx) = sync::mpsc::channel();

        // A flag to indicate that the search should be stopped as soon as possible
        let stop_flag: AbortFlag = Arc::new(AtomicBool::new(false));
        let thread_stop_flag = stop_flag.clone();

        thread::Builder::new()
            .name("Engine thread".into())
            .stack_size(8 * 1024 * 1024)
            .spawn(move || Self::engine_loop::<Engine>(main_rx, thread_stop_flag))
            .unwrap();

        // Wait for new commands. Every command is a new line
        for line in lock.lines() {
            if let Ok(line_str) = line {
                // Parse the UCI command
                let cmd = UciCommand::from(line_str.as_str());
                match cmd {
                    // Quit the program
                    UciCommand::Quit => return,
                    // Stop the search as soon as possible
                    UciCommand::Stop => {
                        // Set the stop flag so that calculations can be stopped
                        stop_flag.store(true, Ordering::SeqCst);
                        // Send the stop command
                        main_tx.send(cmd).unwrap();
                    }
                    UciCommand::Go(_) => {
                        // Unset the stop flag so that calculations can be made
                        stop_flag.store(false, Ordering::SeqCst);
                        // Send the go command
                        main_tx.send(cmd).unwrap();
                    }
                    // Propagate commands to the engine
                    cmd => {
                        main_tx.send(cmd).unwrap();
                    }
                }
            }
        }
    }
}
