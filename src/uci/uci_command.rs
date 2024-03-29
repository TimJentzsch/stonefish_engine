use std::str::SplitWhitespace;

#[derive(Debug, Eq, PartialEq)]
pub enum UciCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(String, Option<String>),
    UciNewGame,
    Position(UciPosition, Vec<String>),
    Go(UciGoConfig),
    Stop,
    Ponderhit,
    Quit,
    Unknown(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum UciPosition {
    Fen(String),
    Startpos,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct UciGoConfig {
    /// Restrict search to these moves only.
    pub search_moves: Option<Vec<String>>,
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
    pub ponder: bool,
    /// The time that white has left on the clock, in milliseconds.
    pub white_time_ms: Option<usize>,
    /// The time that black has left on the clock, in milliseconds.
    pub black_time_ms: Option<usize>,
    /// White increment per move in milliseconds.
    pub white_increment_ms: usize,
    /// Black increment per move in milliseconds.
    pub black_increment_ms: usize,
    /// The amount of moves to the next time control.
    ///
    /// If you don't get this and get the `white_time_ms` and `black_time_ms` it's sudden death.
    pub moves_to_go: usize,
    /// The maximum depth to search, in plies.
    pub max_depth: Option<usize>,
    /// The maximum amount of nodes to saerch.
    pub max_nodes: Option<usize>,
    /// Search for a mate in the specified number of moves.
    pub search_mate: Option<usize>,
    /// The exact amount of time to search for, in milliseconds.
    pub move_time_ms: Option<usize>,
    /// Search until the `stop` command.
    /// Do not exit the search without being told so in this mode!
    pub infinite: bool,
}

impl UciCommand {
    /// Try to parse a positive integer number.
    fn try_parse_usize(token: Option<&str>) -> Option<usize> {
        if let Some(token_str) = token {
            if let Ok(token_usize) = token_str.parse::<usize>() {
                return Some(token_usize);
            }
        }
        None
    }

    /// Determine if the string is a move in long algebraic notation.
    fn is_move(move_str: &str) -> bool {
        let move_regex = regex::Regex::new(r"^((([a-h][1-8]){2}[bnrq]?)|(0000))$").unwrap();
        move_regex.is_match(move_str)
    }

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
                    for move_str in tokens {
                        moves.push(move_str.to_owned());
                    }
                }
            }

            return UciCommand::Position(pos, moves);
        }

        UciCommand::Unknown(line.to_owned())
    }

    /// Try to parse the contents of a UCI go command.
    fn try_parse_go(go_str: &str) -> Self {
        let mut tokens = go_str.split_whitespace().peekable();

        // Set the default values
        let mut go_config = UciGoConfig {
            search_moves: None,
            ponder: false,
            white_time_ms: None,
            black_time_ms: None,
            white_increment_ms: 0,
            black_increment_ms: 0,
            moves_to_go: 0,
            max_depth: None,
            max_nodes: None,
            search_mate: None,
            move_time_ms: None,
            infinite: false,
        };

        while let Some(go_token) = tokens.next() {
            match go_token {
                "ponder" => go_config.ponder = true,
                "infinite" => go_config.infinite = true,
                "wtime" => {
                    if let Some(white_time_ms) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.white_time_ms = Some(white_time_ms);
                    }
                }
                "btime" => {
                    if let Some(black_time_ms) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.black_time_ms = Some(black_time_ms);
                    }
                }
                "winc" => {
                    if let Some(white_increment_ms) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.white_increment_ms = white_increment_ms;
                    }
                }
                "binc" => {
                    if let Some(black_increment_ms) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.black_increment_ms = black_increment_ms;
                    }
                }
                "movestogo" => {
                    if let Some(moves_to_go) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.moves_to_go = moves_to_go;
                    }
                }
                "depth" => {
                    if let Some(max_depth) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.max_depth = Some(max_depth);
                    }
                }
                "nodes" => {
                    if let Some(max_nodes) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.max_nodes = Some(max_nodes);
                    }
                }
                "mate" => {
                    if let Some(search_mate) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.search_mate = Some(search_mate);
                    }
                }
                "movetime" => {
                    if let Some(move_time_ms) = UciCommand::try_parse_usize(tokens.next()) {
                        go_config.move_time_ms = Some(move_time_ms);
                    }
                }
                "searchmoves" => {
                    let mut search_moves: Vec<String> = vec![];

                    while let Some(move_token) = tokens.peek() {
                        if UciCommand::is_move(move_token) {
                            search_moves.push(tokens.next().unwrap().to_string());
                        } else {
                            break;
                        }
                    }

                    go_config.search_moves = Some(search_moves);
                }
                _ => (),
            }
        }

        UciCommand::Go(go_config)
    }

    /// Try to parse the contents of a UCI go command.
    fn try_parse_set_option(set_option_str: &str) -> Self {
        let mut tokens = set_option_str.split_whitespace().peekable();

        // Set the default values
        let mut name = "".to_string();
        let mut value = None;

        while let Some(option_token) = tokens.next() {
            match option_token {
                "name" => {
                    let mut name_str = "".to_string();

                    // Add to the name while no keyword is encountered
                    while let Some(name_token) = tokens.peek() {
                        if name_token != &"name" && name_token != &"value" {
                            name_str += tokens.next().unwrap();
                            // TODO: Preserve whitespace between name tokens
                            name_str += " ";
                        } else {
                            break;
                        }
                    }

                    name = name_str.trim_end().to_string();
                }
                "value" => {
                    let mut value_str = "".to_string();

                    // Add to the value while no keyword is encountered
                    while let Some(value_token) = tokens.peek() {
                        if value_token != &"name" && value_token != &"value" {
                            value_str += tokens.next().unwrap();
                            // TODO: Preserve whitespace between value tokens
                            value_str += " ";
                        } else {
                            break;
                        }
                    }

                    value = Some(value_str.trim_end().to_string());
                }
                _ => (),
            }
        }

        UciCommand::SetOption(name, value)
    }

    /// Try to parse the contents of a UCI go command.
    fn try_parse_debug(debug_str: &str) -> Self {
        let tokens = debug_str.split_whitespace();

        // Set the default values
        let mut debug = false;

        for option_token in tokens {
            match option_token {
                "on" => debug = true,
                "off" => debug = false,
                _ => (),
            }
        }

        UciCommand::Debug(debug)
    }
}

impl From<&str> for UciCommand {
    fn from(line: &str) -> Self {
        let mut tokens = line.split_whitespace();

        if let Some(cmd_token) = tokens.next() {
            let rest = &rest_str(&tokens);

            match cmd_token {
                "uci" => UciCommand::Uci,
                "debug" => UciCommand::try_parse_debug(rest),
                "isready" => UciCommand::IsReady,
                "setoption" => UciCommand::try_parse_set_option(rest),
                "ucinewgame" => UciCommand::UciNewGame,
                "position" => UciCommand::try_parse_position(line, rest),
                "go" => UciCommand::try_parse_go(rest),
                "stop" => UciCommand::Stop,
                "ponderhit" => UciCommand::Ponderhit,
                "quit" => UciCommand::Quit,
                // Unknown command
                _ => UciCommand::Unknown(line.to_owned()),
            }
        } else {
            // Unknown (empty) command
            UciCommand::Unknown(line.to_owned())
        }
    }
}

/// Get the rest of the tokens as `&str`.
///
/// Temporary workaround until `str_split_whitespace_as_str` has been stabilized.
/// See <https://github.com/rust-lang/rust/issues/77998>.
fn rest_str(tokens: &SplitWhitespace) -> String {
    let token_vec: Vec<_> = tokens.clone().into_iter().collect();
    token_vec.join(" ")
}

#[cfg(test)]
mod tests {
    use crate::uci::uci_command::{UciCommand, UciGoConfig, UciPosition};

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
    fn should_parse_go_default() {
        let actual = UciCommand::from("go");
        let expected = UciCommand::Go(UciGoConfig {
            search_moves: None,
            ponder: false,
            white_time_ms: None,
            black_time_ms: None,
            white_increment_ms: 0,
            black_increment_ms: 0,
            moves_to_go: 0,
            max_depth: None,
            max_nodes: None,
            search_mate: None,
            move_time_ms: None,
            infinite: false,
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_go_all_params() {
        let actual = UciCommand::from(
            "go ponder infinite searchmoves e2e3 e2e4 wtime 5201 btime 10662 winc 1000 binc 1000 movestogo 1 depth 10 nodes 10000 mate 5 movetime 10000",
        );
        let expected = UciCommand::Go(UciGoConfig {
            search_moves: Some(vec!["e2e3".to_string(), "e2e4".to_string()]),
            ponder: true,
            white_time_ms: Some(5201),
            black_time_ms: Some(10662),
            white_increment_ms: 1000,
            black_increment_ms: 1000,
            moves_to_go: 1,
            max_depth: Some(10),
            max_nodes: Some(10000),
            search_mate: Some(5),
            move_time_ms: Some(10000),
            infinite: true,
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_set_option_no_value() {
        let actual = UciCommand::from("setoption name Clear Hash");
        let expected = UciCommand::SetOption("Clear Hash".to_string(), None);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_set_option_with_value() {
        let actual = UciCommand::from("setoption name Nullmove value true");
        let expected = UciCommand::SetOption("Nullmove".to_string(), Some("true".to_string()));
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_debug_on() {
        let actual = UciCommand::from("debug on");
        let expected = UciCommand::Debug(true);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_debug_off() {
        let actual = UciCommand::from("debug off");
        let expected = UciCommand::Debug(false);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_parse_unknown_command() {
        let actual = UciCommand::from("asdjasjgasbdfoa");
        let expected = UciCommand::Unknown("asdjasjgasbdfoa".to_owned());
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_recognize_null_move() {
        let actual = UciCommand::is_move("0000");
        assert_eq!(actual, true);
    }

    #[test]
    fn should_recognize_standard_move() {
        let actual = UciCommand::is_move("e2e4");
        assert_eq!(actual, true);
    }

    #[test]
    fn should_recognize_promotion_move() {
        let actual = UciCommand::is_move("e7e8q");
        assert_eq!(actual, true);
    }
}
