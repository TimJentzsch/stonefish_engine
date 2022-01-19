use std::time::Duration;

use pleco::Player;

use crate::uci::uci_command::UciGoConfig;

pub fn get_max_time(go_config: UciGoConfig, player: Player) -> Option<Duration> {
    // Check if the search time is already determined by the GUI
    if go_config.infinite {
        // Search infinitely
        return None;
    } else if let Some(move_time_ms) = go_config.move_time_ms {
        // Search for a fixed time
        return Some(Duration::from_millis(move_time_ms.try_into().unwrap()));
    }

    // Determine player time and increment
    let (time, increment_ms) = match player {
        pleco::Player::White => (go_config.white_time_ms, go_config.white_increment_ms),
        pleco::Player::Black => (go_config.black_time_ms, go_config.black_increment_ms),
    };

    // Get the remaining time on the clock
    let time_ms = if let Some(time_ms) = time {
        time_ms
    } else if go_config.max_depth.or(go_config.search_mate).is_some() {
        // No time is given, but a maximum depth
        // Just consider the depth and don't restrict the time
        return None;
    } else {
        // No time information is given, only search for 10 seconds
        return Some(Duration::from_millis(10_000));
    };

    // Determine base time to search for
    let base_time_ms = match time_ms {
        // Up to 10 seconds
        0..=10_000 => 1_000,
        // 10 seconds to 1 minute
        10_001..=60_000 => 5_000,
        // 1 minute to 3 minutes
        60_001..=180_000 => 7_000,
        // 3 minutes to 5 minutes
        180_001..=300_000 => 10_000,
        // 5 minutes to 10 minutes
        300_001..=600_000 => 20_000,
        // 10 minutes to 30 minutes
        600_001..=1_800_000 => 30_000,
        // More than 30 minutes
        _ => 60_000,
    };

    // Also add the increment time
    let mut search_time_ms = base_time_ms + increment_ms;

    // Don't search longer than the remaining time
    // Including a 500 ms delay to account for API latency
    search_time_ms = search_time_ms.min(time_ms).saturating_sub(500);

    Some(Duration::from_millis(search_time_ms.try_into().unwrap()))
}
