/// The evaluation of a given position.
///
/// Smaller values mean an advantage for Black, bigger values an advantage for White.
#[derive(Debug, Clone, Copy)]
pub enum Evaluation {
    /// A material evaluation in centipawns.
    ///
    /// Negative numbers are an advantage for Black, positive numbers an advantage for White.
    Centipawns(i32),
    /// White can give checkmate in the given number of plies.
    WhiteCheckmate(usize),
    /// Black can give checkmate in the given number of plies.
    BlackCheckmate(usize),
}
use std::cmp::Ordering;

use pleco::Player;

use super::pov_evaluation::PovEvaluation;

impl Evaluation {
    /// Determine if the evaluation is a forced checkmate.
    pub fn is_forced_mate(&self) -> bool {
        !matches!(self, &Evaluation::Centipawns(_))
    }

    /// Construct a checkmate, given the player who got checkmated.
    pub fn from_checkmated_player(checkmated: Player) -> Self {
        match checkmated {
            // White lost, Black can give a checkmate "in 0 plies"
            Player::White => Self::BlackCheckmate(0),
            // Black lost, White can give a checkmate "in 0 plies"
            Player::Black => Self::WhiteCheckmate(0),
        }
    }

    /// Convert the evaluation to the point of view of the given player.
    pub fn to_pov(&self, player: Player) -> PovEvaluation {
        match player {
            Player::White => match self {
                Evaluation::Centipawns(value) => PovEvaluation::Centipawns(*value),
                Evaluation::WhiteCheckmate(plies) => PovEvaluation::PlayerCheckmate(*plies),
                Evaluation::BlackCheckmate(plies) => PovEvaluation::OpponentCheckmate(*plies),
            },
            Player::Black => match self {
                Evaluation::Centipawns(value) => PovEvaluation::Centipawns(-*value),
                Evaluation::WhiteCheckmate(plies) => PovEvaluation::OpponentCheckmate(*plies),
                Evaluation::BlackCheckmate(plies) => PovEvaluation::PlayerCheckmate(*plies),
            },
        }
    }

    /// Convert the evaluation to the previous plie.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!(Evaluation::WhiteCheckmate(3).previous_plie(), Evaluation::WhiteCheckmate(4));
    /// ```
    pub fn previous_plie(&self) -> Self {
        match self {
            Evaluation::Centipawns(mat) => Evaluation::Centipawns(*mat),
            Evaluation::WhiteCheckmate(plies) => Evaluation::WhiteCheckmate(plies + 1),
            Evaluation::BlackCheckmate(plies) => Evaluation::BlackCheckmate(plies + 1),
        }
    }
}

impl Ord for Evaluation {
    /// Good evaluations for White are bigger.
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Evaluation::Centipawns(mat_a) => match other {
                // Prefer positive material score (advantage for White)
                Evaluation::Centipawns(mat_b) => mat_a.cmp(mat_b),
                Evaluation::WhiteCheckmate(_) => Ordering::Less,
                Evaluation::BlackCheckmate(_) => Ordering::Greater,
            },
            Evaluation::WhiteCheckmate(moves_a) => {
                match other {
                    // It's better for White to mate in less moves
                    Evaluation::WhiteCheckmate(moves_b) => moves_b.cmp(moves_a),
                    // Mating the opponent is better than everything else
                    _ => Ordering::Greater,
                }
            }
            Evaluation::BlackCheckmate(moves_a) => {
                match other {
                    // It's better for White get mated in more moves
                    Evaluation::BlackCheckmate(moves_b) => moves_a.cmp(moves_b),
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
        matches!(self.cmp(other), Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Evaluation;

    #[test]
    fn should_recognize_forced_mate() {
        assert!(Evaluation::WhiteCheckmate(3).is_forced_mate());
        assert!(Evaluation::BlackCheckmate(3).is_forced_mate());
        assert_eq!(Evaluation::Centipawns(100).is_forced_mate(), false);
        assert_eq!(Evaluation::Centipawns(-100).is_forced_mate(), false);
    }

    #[test]
    fn should_convert_to_previous_plie() {
        assert_eq!(
            Evaluation::WhiteCheckmate(3).previous_plie(),
            Evaluation::WhiteCheckmate(4)
        );
        assert_eq!(
            Evaluation::BlackCheckmate(3).previous_plie(),
            Evaluation::BlackCheckmate(4)
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
        let player_checkmate = Evaluation::WhiteCheckmate(3);
        let opponent_checkmate = Evaluation::BlackCheckmate(3);

        assert_eq!(player_checkmate.cmp(&opponent_checkmate), Ordering::Greater);
        assert_eq!(opponent_checkmate.cmp(&player_checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_white_checkmates() {
        let fast_checkmate = Evaluation::WhiteCheckmate(3);
        let slow_checkmate = Evaluation::WhiteCheckmate(6);

        assert_eq!(fast_checkmate.cmp(&slow_checkmate), Ordering::Greater);
        assert_eq!(slow_checkmate.cmp(&fast_checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_black_checkmates() {
        let fast_checkmate = Evaluation::BlackCheckmate(3);
        let slow_checkmate = Evaluation::BlackCheckmate(6);

        assert_eq!(fast_checkmate.cmp(&slow_checkmate), Ordering::Less);
        assert_eq!(slow_checkmate.cmp(&fast_checkmate), Ordering::Greater);
    }

    #[test]
    fn should_compare_white_checkmate_with_material_value() {
        let checkmate = Evaluation::WhiteCheckmate(10);
        let eval = Evaluation::Centipawns(100);

        assert_eq!(checkmate.cmp(&eval), Ordering::Greater);
        assert_eq!(eval.cmp(&checkmate), Ordering::Less);
    }

    #[test]
    fn should_compare_black_checkmate_with_material_value() {
        let checkmate = Evaluation::BlackCheckmate(10);
        let eval = Evaluation::Centipawns(100);

        assert_eq!(eval.cmp(&checkmate), Ordering::Greater);
        assert_eq!(checkmate.cmp(&eval), Ordering::Less);
    }
}
