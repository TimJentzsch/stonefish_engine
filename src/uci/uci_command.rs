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
    Go(String),
    Perft(u8),
    Option(String, String),
    Unknown(String),
}

#[derive(Debug, PartialEq)]
pub enum UciPosition {
    Fen(String),
    Startpos,
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
