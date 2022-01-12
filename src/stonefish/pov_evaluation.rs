/// The evaluation of a given position.
///
/// Smaller values mean an advantage for the opponent, bigger values an advantage for the current player.
#[derive(Debug, Clone, Copy)]
pub enum PovEvaluation {
    /// A material evaluation in centipawns.
    ///
    /// Negative numbers are an advantage for the opponent, positive numbers an advantage for the current player.
    Centipawns(i32),
    /// The current player can give checkmate in the given number of plies.
    PlayerCheckmate(usize),
    /// The opponent can give checkmate in the given number of plies.
    OpponentCheckmate(usize),
}
use std::cmp::Ordering;

impl PovEvaluation {
    /// Determine if the evaluation is a forced checkmate.
    pub fn is_forced_mate(&self) -> bool {
        !matches!(self, &PovEvaluation::Centipawns(_))
    }

    /// Convert the evaluation to the view of the opponent.
    pub fn for_opponent(&self) -> Self {
        match self {
            PovEvaluation::Centipawns(mat) => PovEvaluation::Centipawns(-mat),
            PovEvaluation::PlayerCheckmate(plies) => PovEvaluation::OpponentCheckmate(*plies),
            PovEvaluation::OpponentCheckmate(plies) => PovEvaluation::PlayerCheckmate(*plies),
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
            PovEvaluation::Centipawns(mat) => PovEvaluation::Centipawns(*mat),
            PovEvaluation::PlayerCheckmate(plies) => PovEvaluation::PlayerCheckmate(plies + 1),
            PovEvaluation::OpponentCheckmate(plies) => PovEvaluation::OpponentCheckmate(plies + 1),
        }
    }
}

impl Ord for PovEvaluation {
    /// Good evaluations for the current player are bigger.
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PovEvaluation::Centipawns(mat_a) => match other {
                // Prefer positive material score (advantage for the player)
                PovEvaluation::Centipawns(mat_b) => mat_a.cmp(mat_b),
                PovEvaluation::PlayerCheckmate(_) => Ordering::Less,
                PovEvaluation::OpponentCheckmate(_) => Ordering::Greater,
            },
            PovEvaluation::PlayerCheckmate(moves_a) => {
                match other {
                    // It's better for the player to mate in less moves
                    PovEvaluation::PlayerCheckmate(moves_b) => moves_b.cmp(moves_a),
                    // Mating the opponent is better than everything else
                    _ => Ordering::Greater,
                }
            }
            PovEvaluation::OpponentCheckmate(moves_a) => {
                match other {
                    // It's better for the player if the opponent needs more moves for mate
                    PovEvaluation::OpponentCheckmate(moves_b) => moves_a.cmp(moves_b),
                    // Everything is better than getting mated by the opponent
                    _ => Ordering::Less,
                }
            }
        }
    }
}

impl PartialOrd for PovEvaluation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PovEvaluation {}

impl PartialEq for PovEvaluation {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::PovEvaluation;

    #[test]
    fn should_recognize_forced_mate() {
        assert!(PovEvaluation::PlayerCheckmate(3).is_forced_mate());
        assert!(PovEvaluation::OpponentCheckmate(3).is_forced_mate());
        assert_eq!(PovEvaluation::Centipawns(100).is_forced_mate(), false);
        assert_eq!(PovEvaluation::Centipawns(-100).is_forced_mate(), false);
    }

    #[test]
    fn should_convert_to_opponent_view() {
        assert_eq!(
            PovEvaluation::PlayerCheckmate(3).for_opponent(),
            PovEvaluation::OpponentCheckmate(3)
        );
        assert_eq!(
            PovEvaluation::OpponentCheckmate(3).for_opponent(),
            PovEvaluation::PlayerCheckmate(3)
        );
        assert_eq!(
            PovEvaluation::Centipawns(100).for_opponent(),
            PovEvaluation::Centipawns(-100)
        );
        assert_eq!(
            PovEvaluation::Centipawns(-100).for_opponent(),
            PovEvaluation::Centipawns(100)
        );
        assert_eq!(
            PovEvaluation::Centipawns(0).for_opponent(),
            PovEvaluation::Centipawns(0)
        );
    }

    #[test]
    fn should_convert_to_previous_plie() {
        assert_eq!(
            PovEvaluation::PlayerCheckmate(3).previous_plie(),
            PovEvaluation::PlayerCheckmate(4)
        );
        assert_eq!(
            PovEvaluation::OpponentCheckmate(3).previous_plie(),
            PovEvaluation::OpponentCheckmate(4)
        );
        assert_eq!(
            PovEvaluation::Centipawns(100).previous_plie(),
            PovEvaluation::Centipawns(100)
        );
    }

    #[test]
    fn should_compare_material_values() {
        let bad_eval = PovEvaluation::Centipawns(-6);
        let good_eval = PovEvaluation::Centipawns(6);

        assert_eq!(good_eval.cmp(&bad_eval), Ordering::Greater);
        assert_eq!(bad_eval.cmp(&good_eval), Ordering::Less);
    }

    #[test]
    fn should_compare_different_player_checkmates() {
        let player_checkmate = PovEvaluation::PlayerCheckmate(3);
        let opponent_checkmate = PovEvaluation::OpponentCheckmate(3);

        assert_eq!(player_checkmate.cmp(&opponent_checkmate), Ordering::Greater);
        assert_eq!(opponent_checkmate.cmp(&player_checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_player_checkmates() {
        let fast_checkmate = PovEvaluation::PlayerCheckmate(3);
        let slow_checkmate = PovEvaluation::PlayerCheckmate(6);

        assert_eq!(fast_checkmate.cmp(&slow_checkmate), Ordering::Greater);
        assert_eq!(slow_checkmate.cmp(&fast_checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_opponent_checkmates() {
        let fast_checkmate = PovEvaluation::OpponentCheckmate(3);
        let slow_checkmate = PovEvaluation::OpponentCheckmate(6);

        assert_eq!(fast_checkmate.cmp(&slow_checkmate), Ordering::Less);
        assert_eq!(slow_checkmate.cmp(&fast_checkmate), Ordering::Greater);
    }

    #[test]
    fn should_compare_player_checkmate_with_material_value() {
        let checkmate = PovEvaluation::PlayerCheckmate(10);
        let eval = PovEvaluation::Centipawns(100);

        assert_eq!(checkmate.cmp(&eval), Ordering::Greater);
        assert_eq!(eval.cmp(&checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_opponent_checkmate_with_material_value() {
        let checkmate = PovEvaluation::OpponentCheckmate(10);
        let eval = PovEvaluation::Centipawns(100);

        assert_eq!(eval.cmp(&checkmate), Ordering::Greater);
        assert_eq!(checkmate.cmp(&eval), Ordering::Less);
    }
}
