/// The evaluation of a given position.
///
/// Smaller values mean an advantage for the opponent, bigger values an advantage for the current player.
#[derive(Debug, Clone, Copy)]
pub enum Evaluation {
    /// A material evaluation in centipawns.
    ///
    /// Negative numbers are an advantage for the opponent, positive numbers an advantage for the current player.
    Centipawns(i32),
    /// The current player can give checkmate in the given number of plies.
    PlayerCheckmate(usize),
    /// The opponent can give checkmate in the given number of plies.
    OpponentCheckmate(usize),
    /// A draw, e.g. through threefold-repetition
    Draw,
}
use std::cmp::Ordering;

impl Evaluation {
    /// Determine if the evaluation marks the end of the game.
    pub fn is_game_over(&self) -> bool {
        !matches!(self, &Evaluation::Centipawns(_))
    }

    /// Convert the evaluation to the view of the opponent.
    pub fn for_opponent(&self) -> Self {
        match self {
            Evaluation::Centipawns(mat) => Evaluation::Centipawns(-mat),
            Evaluation::PlayerCheckmate(plies) => Evaluation::OpponentCheckmate(*plies),
            Evaluation::OpponentCheckmate(plies) => Evaluation::PlayerCheckmate(*plies),
            Evaluation::Draw => Evaluation::Draw,
        }
    }

    /// Convert the evaluation to the previous plie.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Evaluation::PlayerCheckmate(3).previous_plie(), Evaluation::PlayerCheckmate(4));
    /// ```
    pub fn previous_plie(&self) -> Self {
        match self {
            Evaluation::Centipawns(mat) => Evaluation::Centipawns(*mat),
            Evaluation::PlayerCheckmate(plies) => Evaluation::PlayerCheckmate(plies + 1),
            Evaluation::OpponentCheckmate(plies) => Evaluation::OpponentCheckmate(plies + 1),
            Evaluation::Draw => Evaluation::Draw,
        }
    }
}

impl Ord for Evaluation {
    /// Good evaluations for the current player are bigger.
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Evaluation::Centipawns(mat_a) => match other {
                // Prefer positive material score (advantage for the player)
                Evaluation::Centipawns(mat_b) => mat_a.cmp(mat_b),
                Evaluation::PlayerCheckmate(_) => Ordering::Less,
                Evaluation::OpponentCheckmate(_) => Ordering::Greater,
                Evaluation::Draw => 0.cmp(mat_a),
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
            Evaluation::Draw => {
                match other {
                    Evaluation::Centipawns(mat_b) => 0.cmp(mat_b),
                    Evaluation::PlayerCheckmate(_) => Ordering::Less,
                    Evaluation::OpponentCheckmate(_) => Ordering::Greater,
                    Evaluation::Draw => Ordering::Equal,
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
        matches!(self.cmp(other), Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Evaluation;

    #[test]
    fn should_recognize_game_over() {
        assert!(Evaluation::PlayerCheckmate(3).is_game_over());
        assert!(Evaluation::OpponentCheckmate(3).is_game_over());
        assert!(Evaluation::Draw.is_game_over());
        assert_eq!(Evaluation::Centipawns(100).is_game_over(), false);
        assert_eq!(Evaluation::Centipawns(-100).is_game_over(), false);
        assert_eq!(Evaluation::Centipawns(0).is_game_over(), false);
    }

    #[test]
    fn should_convert_to_opponent_view() {
        assert_eq!(
            Evaluation::PlayerCheckmate(3).for_opponent(),
            Evaluation::OpponentCheckmate(3)
        );
        assert_eq!(
            Evaluation::OpponentCheckmate(3).for_opponent(),
            Evaluation::PlayerCheckmate(3)
        );
        assert_eq!(
            Evaluation::Centipawns(100).for_opponent(),
            Evaluation::Centipawns(-100)
        );
        assert_eq!(
            Evaluation::Centipawns(-100).for_opponent(),
            Evaluation::Centipawns(100)
        );
        assert_eq!(
            Evaluation::Centipawns(0).for_opponent(),
            Evaluation::Centipawns(0)
        );
    }

    #[test]
    fn should_convert_to_previous_plie() {
        assert_eq!(
            Evaluation::PlayerCheckmate(3).previous_plie(),
            Evaluation::PlayerCheckmate(4)
        );
        assert_eq!(
            Evaluation::OpponentCheckmate(3).previous_plie(),
            Evaluation::OpponentCheckmate(4)
        );
        assert_eq!(
            Evaluation::Centipawns(100).previous_plie(),
            Evaluation::Centipawns(100)
        );
    }

    #[test]
    fn should_compare_material_values() {
        let bad_eval = Evaluation::Centipawns(-6);
        let good_eval = Evaluation::Centipawns(6);

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
        let eval = Evaluation::Centipawns(100);

        assert_eq!(checkmate.cmp(&eval), Ordering::Greater);
        assert_eq!(eval.cmp(&checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_opponent_checkmate_with_material_value() {
        let checkmate = Evaluation::OpponentCheckmate(10);
        let eval = Evaluation::Centipawns(100);

        assert_eq!(eval.cmp(&checkmate), Ordering::Greater);
        assert_eq!(checkmate.cmp(&eval), Ordering::Less);
    }
}
