mod evaluation;
pub mod evaluatable;

/// The evaluation of a given position.
///
/// Smaller values mean an advantage for the opponent, bigger values an advantage for the current player.
#[derive(Debug, Clone, Copy)]
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
