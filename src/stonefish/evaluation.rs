use std::cmp::Ordering;

use pleco::{Board, Player};

pub enum Evaluation {
    /// The evaluation of the given position in centipawns.
    ///
    /// Positive numbers are an advantage for white, negative numbers an advantage for black.
    Eval(i32),
    /// Checkmate in the given number of moves for the given player.
    Checkmate(u8, Player),
}

impl Ord for Evaluation {
    /// Good evaluations for white are greater.
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Evaluation::Eval(eval_a) => match other {
                Evaluation::Eval(eval_b) => eval_a.cmp(eval_b),
                Evaluation::Checkmate(_, player_b) => match player_b {
                    Player::White => Ordering::Less,
                    Player::Black => Ordering::Greater,
                },
            },
            Evaluation::Checkmate(mv_a, player_a) => {
                match other {
                    Evaluation::Eval(_) => match player_a {
                        Player::White => Ordering::Greater,
                        Player::Black => Ordering::Less,
                    },
                    Evaluation::Checkmate(mv_b, player_b) => {
                        if player_a == player_b {
                            match player_a {
                                // Checkmate in less moves is better for white
                                Player::White => mv_b.cmp(mv_a),
                                // Checkmate in more moves is better for white
                                Player::Black => mv_a.cmp(mv_b),
                            }
                        } else {
                            // Checkmate for white is better for white
                            match player_a {
                                Player::White => Ordering::Greater,
                                Player::Black => Ordering::Less,
                            }
                        }
                    }
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

/// Evaluate the given position.
pub fn evaluate(board: Board) -> Evaluation {
    Evaluation::Eval(0)
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use pleco::Player;

    use super::Evaluation;

    #[test]
    fn should_compare_evals() {
        let black_eval = Evaluation::Eval(-6);
        let white_eval = Evaluation::Eval(6);

        assert_eq!(black_eval.cmp(&white_eval), Ordering::Less);
        assert_eq!(white_eval.cmp(&black_eval), Ordering::Greater);
    }

    #[test]
    fn should_compare_checkmates_different_players() {
        let black_checkmate = Evaluation::Checkmate(3, Player::Black);
        let white_checkmate = Evaluation::Checkmate(3, Player::White);

        assert_eq!(black_checkmate.cmp(&white_checkmate), Ordering::Less);
        assert_eq!(white_checkmate.cmp(&black_checkmate), Ordering::Greater);
    }

    #[test]
    fn should_compare_checkmates_white() {
        let white_checkmate_high = Evaluation::Checkmate(6, Player::White);
        let white_checkmate_low = Evaluation::Checkmate(3, Player::White);

        assert_eq!(
            white_checkmate_high.cmp(&white_checkmate_low),
            Ordering::Less
        );
        assert_eq!(
            white_checkmate_low.cmp(&white_checkmate_high),
            Ordering::Greater
        );
    }

    #[test]
    fn should_compare_checkmates_black() {
        let black_checkmate_low = Evaluation::Checkmate(3, Player::Black);
        let black_checkmate_high = Evaluation::Checkmate(6, Player::Black);

        assert_eq!(
            black_checkmate_low.cmp(&black_checkmate_high),
            Ordering::Less
        );
        assert_eq!(
            black_checkmate_high.cmp(&black_checkmate_low),
            Ordering::Greater
        );
    }

    #[test]
    fn should_compare_eval_checkmate_white() {
        let eval = Evaluation::Eval(100);
        let white_checkmate = Evaluation::Checkmate(10, Player::White);

        assert_eq!(
            eval.cmp(&white_checkmate),
            Ordering::Less
        );
        assert_eq!(
            white_checkmate.cmp(&eval),
            Ordering::Greater
        );
    }

    #[test]
    fn should_compare_eval_checkmate_black() {
        let black_checkmate = Evaluation::Checkmate(10, Player::Black);
        let eval = Evaluation::Eval(100);

        assert_eq!(
            black_checkmate.cmp(&eval),
            Ordering::Less
        );
        assert_eq!(
            eval.cmp(&black_checkmate),
            Ordering::Greater
        );
    }
}
