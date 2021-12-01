use std::cmp::Ordering;

/// The evaluation of a given position.
///
/// Smaller values mean an advantage for the opponent, bigger values an advantage for the current player.
#[derive(Debug)]
pub enum Evaluation {
    /// A material evaluation in centipawns.
    ///
    /// Negative numbers are an advantage for the opponent, positive numbers an advantage for the current player.
    Material(i32),
    /// The current player can give checkmate in the given number of plies.
    PlayerCheckmate(usize),
    /// The opponent can give checkmate in the given number of plies.
    OpponentCheckmate(usize),
}

impl Evaluation {
    pub fn for_other_player(&self) -> Self {
        match self {
            Evaluation::Material(mat) => Evaluation::Material(-mat),
            Evaluation::PlayerCheckmate(plies) => Evaluation::OpponentCheckmate(plies + 1),
            Evaluation::OpponentCheckmate(plies) => Evaluation::PlayerCheckmate(plies + 1),
        }
    }
}

impl Ord for Evaluation {
    /// Good evaluations for the current player are bigger.
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Evaluation::Material(mat_a) => match other {
                // Prefer positive material score (advantage for the player)
                Evaluation::Material(mat_b) => mat_a.cmp(mat_b),
                Evaluation::PlayerCheckmate(_) => Ordering::Less,
                Evaluation::OpponentCheckmate(_) => Ordering::Greater,
            },
            Evaluation::PlayerCheckmate(moves_a) => {
                match other {
                    // It's better for the player to mate in less moves
                    Evaluation::PlayerCheckmate(moves_b) => moves_b.cmp(moves_a),
                    // Mating the opponent is better than everything else
                    _ => Ordering::Greater,
                }
            }
            Evaluation::OpponentCheckmate(moves_a) => {
                match other {
                    // It's better for the player if the opponent needs more moves for mate
                    Evaluation::OpponentCheckmate(moves_b) => moves_a.cmp(moves_b),
                    // Everything is better than getting mated by the opponent
                    _ => Ordering::Less,
                }
            }
        }
    }
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Evaluation {}

impl PartialEq for Evaluation {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Evaluation;

    #[test]
    fn should_compare_material_values() {
        let bad_eval = Evaluation::Material(-6);
        let good_eval = Evaluation::Material(6);

        assert_eq!(good_eval.cmp(&bad_eval), Ordering::Greater);
        assert_eq!(bad_eval.cmp(&good_eval), Ordering::Less);
    }

    #[test]
    fn should_compare_different_player_checkmates() {
        let player_checkmate = Evaluation::PlayerCheckmate(3);
        let opponent_checkmate = Evaluation::OpponentCheckmate(3);

        assert_eq!(player_checkmate.cmp(&opponent_checkmate), Ordering::Greater);
        assert_eq!(opponent_checkmate.cmp(&player_checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_player_checkmates() {
        let fast_checkmate = Evaluation::PlayerCheckmate(3);
        let slow_checkmate = Evaluation::PlayerCheckmate(6);

        assert_eq!(fast_checkmate.cmp(&slow_checkmate), Ordering::Greater);
        assert_eq!(slow_checkmate.cmp(&fast_checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_opponent_checkmates() {
        let fast_checkmate = Evaluation::OpponentCheckmate(3);
        let slow_checkmate = Evaluation::OpponentCheckmate(6);

        assert_eq!(fast_checkmate.cmp(&slow_checkmate), Ordering::Less);
        assert_eq!(slow_checkmate.cmp(&fast_checkmate), Ordering::Greater);
    }

    #[test]
    fn should_compare_player_checkmate_with_material_value() {
        let checkmate = Evaluation::PlayerCheckmate(10);
        let eval = Evaluation::Material(100);

        assert_eq!(checkmate.cmp(&eval), Ordering::Greater);
        assert_eq!(eval.cmp(&checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_opponent_checkmate_with_material_value() {
        let checkmate = Evaluation::OpponentCheckmate(10);
        let eval = Evaluation::Material(100);

        assert_eq!(eval.cmp(&checkmate), Ordering::Greater);
        assert_eq!(checkmate.cmp(&eval), Ordering::Less);
    }
}
