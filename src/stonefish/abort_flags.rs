use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::uci::uci::AbortFlag;

/// The search has been aborted.
#[derive(Debug, PartialEq)]
pub struct SearchAborted;

#[derive(Debug, Clone)]
pub struct AbortFlags {
    /// Flag to check if the search has been stopped manually.
    stop_flag: AbortFlag,
    /// Flag to check if the search ran out of time.
    time_flag: AbortFlag,
}

impl AbortFlags {
    /// Create new abort flags.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            stop_flag: Arc::new(AtomicBool::new(false)),
            time_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Create new abort flags from existing flags.
    pub fn from_flags(stop_flag: AbortFlag, time_flag: AbortFlag) -> Self {
        Self {
            stop_flag,
            time_flag,
        }
    }

    /// Check if the search has been aborted.
    pub fn check(&self) -> Result<(), SearchAborted> {
        // Check if the search has been aborted
        return if self.stop_flag.load(Ordering::SeqCst) || self.time_flag.load(Ordering::SeqCst) {
            Err(SearchAborted)
        } else {
            Ok(())
        };
    }
}
