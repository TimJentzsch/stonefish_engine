use std::time::Duration;

use pleco::Player;

use crate::uci::uci_command::UciGoConfig;

pub fn get_max_time(go_config: UciGoConfig, player: Player) -> Option<Duration> {
    // Determine player time and increment
    let (time, increment) = match player {
        pleco::Player::White => (go_config.white_time_ms, go_config.white_increment_ms),
        pleco::Player::Black => (go_config.black_time_ms, go_config.black_increment_ms),
    };

    // Determine maximum time
    if let Some(move_time_ms) = go_config.move_time_ms {
        Some(Duration::from_millis(move_time_ms.try_into().unwrap()))
    } else if go_config.infinite {
        // Search infinitely
        None
    } else if let Some(time_ms) = time {
        // Take 5 seconds reserve time for each move
        let base_time_ms: u64 = 5000.min(time_ms.try_into().unwrap());
        // Additionally use the increment time
        let increment_time_ms: u64 = increment.try_into().unwrap();
        let mut total_time_ms = base_time_ms + increment_time_ms;
        // Consider a delay of 500 ms and cap at 10 seconds
        total_time_ms = total_time_ms.saturating_sub(500).min(10000);
        Some(Duration::from_millis(total_time_ms))
    } else if go_config.max_depth.or(go_config.search_mate).is_some() {
        None
    } else {
        // Search for 10 seconds
        Some(Duration::from_millis(10000))
    }
}
