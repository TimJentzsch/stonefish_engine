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
