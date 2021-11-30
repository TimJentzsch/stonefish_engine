use std::cmp::Ordering;

use pleco::Board;

use super::evaluation::Evaluation;

pub trait Evaluatable {
    /// Evaluate the position.
    fn evaluate(&self) -> Evaluation {
        Evaluation::Eval(0)
    }
}

impl Evaluatable for Board {
    /// Evaluate the given position.
    fn evaluate(&self) -> Evaluation {
        // TODO: Properly evaluate the position
        Evaluation::Eval(0)
    }
}

impl Ord for dyn Evaluatable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluate().cmp(&other.evaluate())
    }
}

impl PartialOrd for dyn Evaluatable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for dyn Evaluatable {}

impl PartialEq for dyn Evaluatable {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}
