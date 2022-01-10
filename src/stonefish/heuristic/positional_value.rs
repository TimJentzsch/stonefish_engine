//! Evaluation of the positional value.
//!
//! Values inspired by https://www.chessprogramming.org/Simplified_Evaluation_Function
use pleco::{BitBoard, Board, Piece, PieceType, Player, SQ};

use super::material_value::get_piece_value;

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

/// The borders of the board.
#[inline]
fn get_border_bb() -> BitBoard {
    BitBoard::FILE_A | BitBoard::FILE_H | BitBoard::RANK_1 | BitBoard::RANK_8
}

/// The corners of the board.
#[inline]
fn get_corner_bb() -> BitBoard {
    SQ::A1.to_bb() | SQ::A8.to_bb() | SQ::H1.to_bb() | SQ::H8.to_bb()
}

/// The center of the board with radius one.
#[inline]
fn get_center_sq_one_bb() -> BitBoard {
    SQ::D4.to_bb() | SQ::E4.to_bb() | SQ::D5.to_bb() | SQ::E5.to_bb()
}

/// Get the center ring of the board with radius two.
#[inline]
fn get_center_ring_two_bb() -> BitBoard {
    (BitBoard::FILE_C | BitBoard::FILE_D | BitBoard::FILE_E | BitBoard::FILE_F)
        & (BitBoard::RANK_1
            | BitBoard::RANK_2
            | get_center_sq_one_bb()
            | BitBoard::RANK_7
            | BitBoard::RANK_8)
}

/// Get the center the board with radius two.
#[inline]
fn get_center_sq_two_bb() -> BitBoard {
    get_center_sq_one_bb() | get_center_ring_two_bb()
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
fn player_king_position(board: &Board, player: Player) -> i32 {
    let piece_bb = board.piece_bb(player, PieceType::K);

    if is_endgame(board) {
        let mut value = 0;

        // Stay away from the borders
        value += score_position(piece_bb, get_border_bb(), -30);
        value += score_position(piece_bb, get_corner_bb(), -20);

        // Go to the center
        value += score_position(piece_bb, get_center_sq_one_bb(), 40);
        value += score_position(piece_bb, get_center_ring_two_bb(), 20);

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
fn player_pawn_position(board: &Board, player: Player) -> i32 {
    let piece_bb = board.piece_bb(player, PieceType::P);
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
fn player_knight_position(board: &Board, player: Player) -> i32 {
    let piece_bb = board.piece_bb(player, PieceType::N);
    let mut value = 0;

    // Avoid having knights on the borders
    value += score_position(piece_bb, get_border_bb(), -30);
    value += score_position(piece_bb, get_corner_bb(), -20);
    // Move knights to the center
    value += score_position(piece_bb, get_center_sq_one_bb(), 20);
    value += score_position(piece_bb, get_center_ring_two_bb(), 10);

    value
}

/// Evaluate the position of the bishops.
fn player_bishop_position(board: &Board, player: Player) -> i32 {
    let piece_bb = board.piece_bb(player, PieceType::B);
    let mut value = 0;

    // Avoid having bishops on the borders
    value += score_position(piece_bb, get_border_bb(), -15);
    value += score_position(piece_bb, get_corner_bb(), -10);
    // Move bishops to the center
    value += score_position(piece_bb, get_center_sq_one_bb(), 15);
    value += score_position(piece_bb, get_center_ring_two_bb(), 10);

    value
}

/// Evaluate the position of the rooks.
fn player_rook_position(board: &Board, player: Player) -> i32 {
    let piece_bb = board.piece_bb(player, PieceType::R);
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
    let border_bb = (BitBoard::FILE_A | BitBoard::FILE_H) ^ get_border_bb();
    value += score_position(piece_bb, border_bb, -5);

    value
}

/// Evaluate the position of the queens.
fn player_queen_position(board: &Board, player: Player) -> i32 {
    let piece_bb = board.piece_bb(player, PieceType::Q);
    let mut value = 0;

    // Avoid having queens on the borders
    value += score_position(piece_bb, get_border_bb(), -10);
    value += score_position(piece_bb, get_corner_bb(), -10);
    // Move the queen to the center
    value += score_position(piece_bb, get_center_sq_two_bb(), 5);

    value
}

/// The total piece position for the player.
fn player_piece_position(board: &Board, player: Player) -> i32 {
    player_king_position(board, player)
        + player_pawn_position(board, player)
        + player_knight_position(board, player)
        + player_bishop_position(board, player)
        + player_rook_position(board, player)
        + player_queen_position(board, player)
}

/// Calculate the positional value for the current player.
fn player_positional_value(board: &Board, player: Player) -> i32 {
    let mut value = player_piece_position(board, player);

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

/// The current positional value.
///
/// Returns a positive number if the current player has a positional advantage.
pub fn positional_value(board: &Board) -> i32 {
    if board.in_check() {
        return -50;
    }

    let player_pos = player_positional_value(board, board.turn());
    let opponent_pos = player_positional_value(board, board.turn().other_player());

    player_pos - opponent_pos
}
