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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use pleco::Player;

    use crate::uci::uci_command::UciGoConfig;

    use super::get_max_time;

    #[test]
    fn should_not_take_longer_than_remaining_time() {
        let params = [
            (10_000_000, 10_000),
            (10_000_000, 0),
            (1_000_000, 10_000),
            (1_000_000, 0),
            (500_000, 10_000),
            (500_000, 0),
            (25_000, 10_000),
            (25_000, 0),
            (10_000, 10_000),
            (10_000, 0),
            (5_000, 10_000),
            (5_000, 0),
            (1_000, 10_000),
            (1_000, 0),
            (500, 10_000),
            (500, 0),
            (100, 10_000),
            (100, 0),
            (1, 10_000),
            (1, 0),
        ];

        for (time_ms, increment_ms) in params {
            let go_config = UciGoConfig {
                search_moves: None,
                ponder: false,
                white_time_ms: Some(time_ms),
                black_time_ms: Some(time_ms),
                white_increment_ms: increment_ms,
                black_increment_ms: increment_ms,
                moves_to_go: 0,
                max_depth: None,
                max_nodes: None,
                search_mate: None,
                move_time_ms: None,
                infinite: false,
            };

            let max_time = Some(Duration::from_millis(time_ms.try_into().unwrap()));

            let actual_white = get_max_time(go_config.clone(), Player::White);
            let actual_black = get_max_time(go_config, Player::Black);

            assert!(
                actual_white < max_time,
                "White takes {actual_white:?}, but has only {max_time:?} left."
            );
            assert!(
                actual_black < max_time,
                "Black takes {actual_black:?}, but has only {max_time:?} left."
            );
        }
    }

    #[test]
    fn should_take_move_time_if_available() {
        let go_config = UciGoConfig {
            search_moves: None,
            ponder: false,
            white_time_ms: Some(1_000_000_000),
            black_time_ms: Some(1_000_000_000),
            white_increment_ms: 1_000_000_000,
            black_increment_ms: 1_000_000_000,
            moves_to_go: 0,
            max_depth: None,
            max_nodes: None,
            search_mate: None,
            move_time_ms: Some(1_000),
            infinite: false,
        };

        let expected = Some(Duration::from_millis(1_000));

        let actual_white = get_max_time(go_config.clone(), Player::White);
        let actual_black = get_max_time(go_config, Player::Black);

        assert_eq!(actual_white, expected);
        assert_eq!(actual_black, expected);
    }

    #[test]
    fn should_respect_infinite_search() {
        let go_config = UciGoConfig {
            search_moves: None,
            ponder: false,
            white_time_ms: Some(1_000),
            black_time_ms: Some(1_000),
            white_increment_ms: 1_000,
            black_increment_ms: 1_000,
            moves_to_go: 0,
            max_depth: None,
            max_nodes: None,
            search_mate: None,
            move_time_ms: Some(1_000),
            infinite: true,
        };

        let actual_white = get_max_time(go_config.clone(), Player::White);
        let actual_black = get_max_time(go_config, Player::Black);

        assert_eq!(actual_white, None);
        assert_eq!(actual_black, None);
    }
}
