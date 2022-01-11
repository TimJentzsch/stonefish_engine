//! Evaluation of the positional value.
//!
//! Values inspired by https://www.chessprogramming.org/Simplified_Evaluation_Function
use pleco::{BitBoard, BitMove, Board, Piece, PieceType, Player, SQ};

use super::material_value::get_piece_value;

/// The bitboard of the border squares.
pub const BORDER_BB: BitBoard =
    BitBoard(0b11111111_10000001_10000001_10000001_10000001_10000001_10000001_11111111);
/// The bitboard of the corner squares.
pub const CORNER_BB: BitBoard =
    BitBoard(0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_10000001);
/// The bitboard of the center squares (radius 1).
pub const CENTER_ONE_BB: BitBoard =
    BitBoard(0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000);
/// The bitboard of the squares around the center squares.
pub const CENTER_RING_BB: BitBoard =
    BitBoard(0b00000000_00000000_00111100_00100100_00100100_00111100_00000000_00000000);
/// The bitboard of the center squares (radius 2).
pub const CENTER_TWO_BB: BitBoard =
    BitBoard(0b00000000_00000000_00111100_00111100_00111100_00111100_00000000_00000000);

struct Guard {
    origin: (SQ, Piece),
    target: (SQ, Piece),
}

impl Guard {
    fn new(origin: (SQ, Piece), target: (SQ, Piece)) -> Self {
        Self { origin, target }
    }
}

/// Gets the squares and pieces that the player guards (defends or attacks).
fn player_guards(board: &Board, player: Player) -> Vec<Guard> {
    let piece_locations = board.get_piece_locations();

    let mut guards = vec![];

    for (sq, piece) in piece_locations {
        // Only consider pieces of the current player
        if piece.player().unwrap_or_else(|| player.other_player()) != player {
            continue;
        }

        // Look at every square that the piece attacks
        for attack_sq in board.attacks_from(piece.type_of(), sq, player) {
            let target = board.piece_at_sq(attack_sq);

            guards.push(Guard::new((sq, piece), (attack_sq, target)));
        }
    }

    guards
}

/// Score how many pieces have the corresponding positions.
fn score_position(piece_bb: BitBoard, position_bb: BitBoard, score: i32) -> i32 {
    (piece_bb & position_bb).count_bits() as i32 * score
}

/// Determine if the player is in the endgame.
fn player_is_endgame(board: &Board, player: Player) -> bool {
    // The player has no queen
    if board.count_piece(player, PieceType::Q) == 0 {
        return true;
    }

    // The player has only one minor piece in addition to the queen
    board.count_piece(player, PieceType::R) == 0
        && board.count_piece(player, PieceType::B) + board.count_piece(player, PieceType::N) <= 1
}

/// Determine if the board is in the endgame.
fn is_endgame(board: &Board) -> bool {
    player_is_endgame(board, Player::White) && player_is_endgame(board, Player::Black)
}

/// Evaluate the position of the king.
fn player_king_position(board: &Board, piece_bb: BitBoard, player: Player) -> i32 {
    if is_endgame(board) {
        let mut value = 0;

        // Stay away from the borders
        value += score_position(piece_bb, BORDER_BB, -30);
        value += score_position(piece_bb, CORNER_BB, -20);

        // Go to the center
        value += score_position(piece_bb, CENTER_ONE_BB, 40);
        value += score_position(piece_bb, CENTER_RING_BB, 20);

        value
    } else {
        let mut value = 0;

        // Encourage castling
        let castle_bb = match player {
            Player::White => SQ::G1.to_bb() | SQ::C1.to_bb(),
            Player::Black => SQ::G8.to_bb() | SQ::C8.to_bb(),
        };
        value += (piece_bb & castle_bb).count_bits() as i32 * 50;

        // Don't stand around in the center
        let center_bb = match player {
            Player::White => SQ::E1.to_bb() | SQ::D1.to_bb() | SQ::E2.to_bb() | SQ::D2.to_bb(),
            Player::Black => SQ::E8.to_bb() | SQ::D8.to_bb() | SQ::E7.to_bb() | SQ::D7.to_bb(),
        };
        value += (piece_bb & center_bb).count_bits() as i32 * -20;

        value
    }
}

/// Evaluate the position of the pawns.
fn player_pawn_position(_board: &Board, piece_bb: BitBoard, player: Player) -> i32 {
    let mut value = 0;

    // Being close to promotion is good
    let rank_seven_bb = match player {
        Player::White => BitBoard::RANK_7,
        Player::Black => BitBoard::RANK_2,
    };
    value += (rank_seven_bb & piece_bb).count_bits() as i32 * 50;

    let rank_six_bb = match player {
        Player::White => BitBoard::RANK_6,
        Player::Black => BitBoard::RANK_3,
    };
    value += (rank_six_bb & piece_bb).count_bits() as i32 * 30;

    let center_files_bb = BitBoard::FILE_D | BitBoard::FILE_E;

    // Developing the center pawns is good
    let rank_five_center_bb = match player {
        Player::White => BitBoard::RANK_5 & center_files_bb,
        Player::Black => BitBoard::RANK_4 & center_files_bb,
    };
    value += (rank_five_center_bb & piece_bb).count_bits() as i32 * 25;

    let rank_four_center_bb = match player {
        Player::White => BitBoard::RANK_4 & center_files_bb,
        Player::Black => BitBoard::RANK_5 & center_files_bb,
    };
    value += (rank_four_center_bb & piece_bb).count_bits() as i32 * 20;

    // Not developing the center pawns is bad
    let rank_two_center_bb = match player {
        Player::White => BitBoard::RANK_2 & center_files_bb,
        Player::Black => BitBoard::RANK_7 & center_files_bb,
    };
    value += (rank_two_center_bb & piece_bb).count_bits() as i32 * -20;

    value
}

/// Evaluate the position of the knights.
fn player_knight_position(_board: &Board, piece_bb: BitBoard) -> i32 {
    let mut value = 0;

    // Avoid having knights on the borders
    value += score_position(piece_bb, BORDER_BB, -30);
    value += score_position(piece_bb, CORNER_BB, -20);
    // Move knights to the center
    value += score_position(piece_bb, CENTER_ONE_BB, 20);
    value += score_position(piece_bb, CENTER_RING_BB, 10);

    value
}

/// Evaluate the position of the bishops.
fn player_bishop_position(_board: &Board, piece_bb: BitBoard) -> i32 {
    let mut value = 0;

    // Avoid having bishops on the borders
    value += score_position(piece_bb, BORDER_BB, -15);
    value += score_position(piece_bb, CORNER_BB, -10);
    // Move bishops to the center
    value += score_position(piece_bb, CENTER_ONE_BB, 15);
    value += score_position(piece_bb, CENTER_RING_BB, 10);

    value
}

/// Evaluate the position of the rooks.
fn player_rook_position(_board: &Board, piece_bb: BitBoard, player: Player) -> i32 {
    let mut value = 0;

    // Being on the 7th rank is good
    let rank_seven_bb = match player {
        Player::White => BitBoard::RANK_7,
        Player::Black => BitBoard::RANK_2,
    };
    value += score_position(piece_bb, rank_seven_bb, 15);

    // Being in the center is good
    let center_bb = match player {
        Player::White => SQ::D1.to_bb() | SQ::E1.to_bb(),
        Player::Black => SQ::D8.to_bb() | SQ::E8.to_bb(),
    };
    value += score_position(piece_bb, center_bb, 10);

    // Avoid the left and right borders
    let border_bb = (BitBoard::FILE_A | BitBoard::FILE_H) ^ BORDER_BB;
    value += score_position(piece_bb, border_bb, -5);

    value
}

/// Evaluate the position of the queens.
fn player_queen_position(_board: &Board, piece_bb: BitBoard) -> i32 {
    let mut value = 0;

    // Avoid having queens on the borders
    value += score_position(piece_bb, BORDER_BB, -10);
    value += score_position(piece_bb, CORNER_BB, -10);
    // Move the queen to the center
    value += score_position(piece_bb, CENTER_TWO_BB, 5);

    value
}

/// The total piece position for the player.
fn player_piece_position(board: &Board, player: Player) -> i32 {
    player_king_position(board, board.piece_bb(player, PieceType::K), player)
        + player_pawn_position(board, board.piece_bb(player, PieceType::P), player)
        + player_knight_position(board, board.piece_bb(player, PieceType::N))
        + player_bishop_position(board, board.piece_bb(player, PieceType::B))
        + player_rook_position(board, board.piece_bb(player, PieceType::R), player)
        + player_queen_position(board, board.piece_bb(player, PieceType::Q))
}

fn player_threat_value(board: &Board, player: Player) -> i32 {
    let mut value = 0;

    let guards = player_guards(board, player);

    for guard in guards {
        let (_, origin_piece) = guard.origin;
        let (_, target_piece) = guard.target;

        if origin_piece.player() == target_piece.player() {
            value += 20;
        } else {
            value += get_piece_value(target_piece.type_of()) / 10;
        }
    }

    value
}

pub fn threat_value(board: &Board) -> i32 {
    let player_threat = player_threat_value(board, board.turn());
    let opponent_threat = player_threat_value(board, board.turn().other_player());

    player_threat - opponent_threat
}

/// The current positional value.
///
/// Returns a positive number if the current player has a positional advantage.
pub fn initial_positional_value(board: &Board) -> i32 {
    let player_pos = player_piece_position(board, board.turn());
    let opponent_pos = player_piece_position(board, board.turn().other_player());

    player_pos - opponent_pos
}

pub fn positional_piece_value(
    piece_type: PieceType,
    board: &Board,
    piece_bb: BitBoard,
    player: Player,
) -> i32 {
    match piece_type {
        PieceType::P => player_pawn_position(board, piece_bb, player),
        PieceType::N => player_knight_position(board, piece_bb),
        PieceType::B => player_bishop_position(board, piece_bb),
        PieceType::R => player_rook_position(board, piece_bb, player),
        PieceType::Q => player_queen_position(board, piece_bb),
        PieceType::K => player_king_position(board, piece_bb, player),
        _ => 0,
    }
}

/// The positional evaluation delta for a given move.
pub fn move_positional_value(old_board: &Board, mv: BitMove, new_board: &Board) -> i32 {
    let player = old_board.turn();
    let piece_type = old_board.piece_at_sq(mv.get_src()).type_of();

    let old_pos_eval = positional_piece_value(piece_type, old_board, mv.get_src().to_bb(), player);
    let new_pos_eval = positional_piece_value(piece_type, new_board, mv.get_dest().to_bb(), player);

    new_pos_eval - old_pos_eval
}

#[cfg(test)]
mod tests {
    use pleco::{BitBoard, SQ};

    use crate::stonefish::heuristic::positional_value::{
        BORDER_BB, CENTER_ONE_BB, CENTER_RING_BB, CENTER_TWO_BB, CORNER_BB,
    };

    #[test]
    fn should_calculate_center_one_bb() {
        let expected = SQ::D4.to_bb() | SQ::E4.to_bb() | SQ::D5.to_bb() | SQ::E5.to_bb();
        assert_eq!(CENTER_ONE_BB, expected);
    }

    #[test]
    fn should_calculate_border_bb() {
        let expected = BitBoard::FILE_A | BitBoard::FILE_H | BitBoard::RANK_1 | BitBoard::RANK_8;
        assert_eq!(BORDER_BB, expected);
    }

    #[test]
    fn should_calculate_corner_bb() {
        let expected = SQ::A1.to_bb() | SQ::A8.to_bb() | SQ::H1.to_bb() | SQ::H8.to_bb();
        assert_eq!(CORNER_BB, expected);
    }

    #[test]
    fn should_calculate_center_ring_bb() {
        let expected = SQ::C3.to_bb()
            | SQ::C4.to_bb()
            | SQ::C5.to_bb()
            | SQ::C6.to_bb()
            | SQ::D6.to_bb()
            | SQ::E6.to_bb()
            | SQ::F6.to_bb()
            | SQ::F5.to_bb()
            | SQ::F4.to_bb()
            | SQ::F3.to_bb()
            | SQ::E3.to_bb()
            | SQ::D3.to_bb();

        assert_eq!(CENTER_RING_BB, expected);
    }

    #[test]
    fn should_calculate_center_two_bb() {
        let expected = CENTER_ONE_BB | CENTER_RING_BB;

        assert_eq!(CENTER_TWO_BB, expected);
    }
}
