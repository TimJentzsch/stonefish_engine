// Adapted from the Weiawaga engine, licensed GPL-2.0
// https://github.com/Heiaha/Weiawaga/blob/493d8139f882b89380c298457267cb059d86dc2f/src/uci/uci.rs
#[derive(Debug, PartialEq)]
pub enum UciCommand {
    Uci,
    UciNewGame,
    IsReady,
    Quit,
    Stop,
    Position(UciPosition, Vec<String>),
    Go(UciGoConfig),
    Perft(u8),
    Option(String, String),
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub enum UciPosition {
    Fen(String),
    Startpos,
}

#[derive(Debug, PartialEq)]
pub struct UciGoConfig {
    /// Restrict search to these moves only.
    search_moves: Option<Vec<String>>,
    /// Start searching in pondering mode.
    ///
    /// Do not exit the search in ponder mode, even if it's mate!
    ///
    /// This means that the last move sent in in the position string is the ponder move.
    /// The engine can do what it wants to do, but after a `ponderhit` command it should execute
    /// the suggested move to ponder on. This means that the ponder move sent by the GUI can be 
    /// interpreted as a recommendation about which move to ponder. However, if the engine decides
    /// to ponder on a different move, it should not display any mainlines as they are likely to be
    /// misinterpreted by the GUI because the GUI expects the engine to ponder on the suggested move.
    ponder: bool,
    /// The time that white has left on the clock, in milliseconds.
    white_time_ms: Option<usize>,
    /// The time that black has left on the clock, in milliseconds.
    black_time_ms: Option<usize>,
    /// White increment per move in milliseconds.
    white_increment_ms: usize,
    /// Black increment per move in milliseconds.
    black_increment_ms: usize,
    /// The amount of moves to the next time control.
    /// 
    /// If you don't get this and get the `white_time_ms` and `black_time_ms` it's sudden death.
    moves_to_go: usize,
    /// The maximum depth to search, in plies.
    max_depth: Option<usize>,
    /// The maximum amount of nodes to saerch.
    max_nodes: Option<usize>,
    /// Search for a mate in the specified number of moves.
    search_mate: Option<usize>,
    /// The exact amount of time to search for, in milliseconds.
    move_time_ms: Option<usize>,
    /// Search until the `stop` command.
    /// Do not exit the search without being told so in this mode!
    infinite: bool,
}

impl UciCommand {
    /// Try to parse the contents of a UCI position command.
    fn try_parse_position(line: &str, pos_str: &str) -> Self {
        let mut tokens = pos_str.split_ascii_whitespace();

        if let Some(pos_str) = tokens.next() {
            // The position to start from
            let pos = match pos_str {
                // The starting position can be provided directly
                "startpos" => UciPosition::Startpos,
                // A position in FEN notation
                // rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
                "fen" => {
                    let mut fen_str = "".to_owned();

                    // A FEN string is composed of 6 tokens
                    for _ in 0..6 {
                        if let Some(fen_part) = tokens.next() {
                            fen_str += fen_part;
                            fen_str += " ";
                        } else {
                            return UciCommand::Unknown(line.to_owned());
                        }
                    }

                    UciPosition::Fen(fen_str.trim_end().to_owned())
                }
                _ => return UciCommand::Unknown(line.to_owned()),
            };

            let mut moves = vec![];

            // Optionally, moves to play after the given position
            if let Some(move_token) = tokens.next() {
                if move_token == "moves" {
                    // Add all given moves
                    while let Some(move_str) = tokens.next() {
                        moves.push(move_str.to_owned());
                    }
                }
            }

            return UciCommand::Position(pos, moves);
        }

        return UciCommand::Unknown(line.to_owned());
    }
}

impl From<&str> for UciCommand {
    fn from(line: &str) -> Self {
        let mut tokens = line.trim().split_whitespace();

        return if let Some(cmd_token) = tokens.next() {
            match cmd_token {
                "uci" => UciCommand::Uci,
                "ucinewgame" => UciCommand::UciNewGame,
                "isready" => UciCommand::IsReady,
                "quit" => UciCommand::Quit,
                "stop" => UciCommand::Stop,
                "position" => {
                    let pos_str = tokens.as_str();
                    UciCommand::try_parse_position(line, pos_str)
                }
                // Unknown command
                _ => UciCommand::Unknown(line.to_owned()),
            }
        } else {
            // Unknown (empty) command
            UciCommand::Unknown(line.to_owned())
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::uci::uci_command::{UciCommand, UciPosition};

    #[test]
    fn should_parse_uci() {
        let actual = UciCommand::from("uci");
        let expected = UciCommand::Uci;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_ucinewgame() {
        let actual = UciCommand::from("ucinewgame");
        let expected = UciCommand::UciNewGame;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_isready() {
        let actual = UciCommand::from("isready");
        let expected = UciCommand::IsReady;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_quit() {
        let actual = UciCommand::from("quit");
        let expected = UciCommand::Quit;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_stop() {
        let actual = UciCommand::from("stop");
        let expected = UciCommand::Stop;
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_position_startpos() {
        let actual = UciCommand::from("position startpos");
        let expected = UciCommand::Position(UciPosition::Startpos, vec![]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_position_fen() {
        // The startposition, but as FEN
        let actual = UciCommand::from(
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        );
        let expected = UciCommand::Position(
            UciPosition::Fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned()),
            vec![],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_position_startpos_moves() {
        let actual = UciCommand::from("position startpos moves e2e4 d7d5 e4d5");
        let expected = UciCommand::Position(
            UciPosition::Startpos,
            vec!["e2e4".to_string(), "d7d5".to_string(), "e4d5".to_string()],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_position_fen_moves() {
        // The startposition, but as FEN
        let actual = UciCommand::from(
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4 d7d5 e4d5",
        );
        let expected = UciCommand::Position(
            UciPosition::Fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_owned()),
            vec!["e2e4".to_string(), "d7d5".to_string(), "e4d5".to_string()],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_unknown_command() {
        let actual = UciCommand::from("asdjasjgasbdfoa");
        let expected = UciCommand::Unknown("asdjasjgasbdfoa".to_owned());
        assert_eq!(actual, expected);
    }
}
