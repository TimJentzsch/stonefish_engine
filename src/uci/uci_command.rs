// Adapted from the Weiawaga engine, licensed GPL-2.0
// https://github.com/Heiaha/Weiawaga/blob/493d8139f882b89380c298457267cb059d86dc2f/src/uci/uci.rs
#[derive(Debug, PartialEq)]
pub enum UciCommand {
    Uci,
    UciNewGame,
    IsReady,
    Quit,
    Stop,
    Position(String, Vec<String>),
    Go(String),
    Perft(u8),
    Option(String, String),
    Unknown(String),
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
    use crate::uci::uci_command::UciCommand;

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
    fn should_parse_unknown_command() {
        let actual = UciCommand::from("asdjasjgasbdfoa");
        let expected = UciCommand::Unknown("asdjasjgasbdfoa".to_owned());
        assert_eq!(actual, expected);
    }
}
