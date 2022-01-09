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

/// Generate a bitboard for the board borders.
#[inline]
fn get_border_bb() -> BitBoard {
    BitBoard::FILE_A | BitBoard::FILE_H | BitBoard::RANK_1 | BitBoard::RANK_8
}

/// Counts the number of pieces at the borders of the board.
fn count_border_pieces(piece_bb: BitBoard) -> u8 {
    (get_border_bb() & piece_bb).count_bits()
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
    let king_bb = board.piece_bb(player, PieceType::K);

    if is_endgame(board) {
        let mut value = 0;

        // Stay away from the borders
        value += count_border_pieces(king_bb) as i32 * -40;

        // Go to the center
        let center_bb = SQ::D4.to_bb() | SQ::E4.to_bb() | SQ::D5.to_bb() | SQ::E5.to_bb();
        value += (king_bb & center_bb).count_bits() as i32 * 30;

        value
    } else {
        let mut value = 0;

        // Encourage castling
        let castle_bb = match player {
            Player::White => SQ::G1.to_bb() | SQ::C1.to_bb(),
            Player::Black => SQ::G8.to_bb() | SQ::C8.to_bb(),
        };
        value += (king_bb & castle_bb).count_bits() as i32 * 50;

        // Don't stand around in the center
        let center_bb = match player {
            Player::White => SQ::E1.to_bb() | SQ::D1.to_bb() | SQ::E2.to_bb() | SQ::D2.to_bb(),
            Player::Black => SQ::E8.to_bb() | SQ::D8.to_bb() | SQ::E7.to_bb() | SQ::D7.to_bb(),
        };
        value += (king_bb & center_bb).count_bits() as i32 * -20;

        value
    }
}

/// Evaluate the position of the knights.
fn player_knight_position(board: &Board, player: Player) -> i32 {
    let knight_bb = board.piece_bb(player, PieceType::N);

    // Avoid having knights on the borders
    count_border_pieces(knight_bb) as i32 * -30
}

/// Evaluate the position of the pawns.
fn player_pawn_position(board: &Board, player: Player) -> i32 {
    let pawn_bb = board.piece_bb(player, PieceType::P);
    let mut value = 0;

    // Being close to promotion is good
    let rank_seven_bb = match player {
        Player::White => BitBoard::RANK_7,
        Player::Black => BitBoard::RANK_2,
    };
    value += (rank_seven_bb & pawn_bb).count_bits() as i32 * 50;

    let rank_six_bb = match player {
        Player::White => BitBoard::RANK_6,
        Player::Black => BitBoard::RANK_3,
    };
    value += (rank_six_bb & pawn_bb).count_bits() as i32 * 30;

    let center_files_bb = BitBoard::FILE_D | BitBoard::FILE_E;

    // Developing the center pawns is good
    let rank_five_center_bb = match player {
        Player::White => BitBoard::RANK_5 & center_files_bb,
        Player::Black => BitBoard::RANK_4 & center_files_bb,
    };
    value += (rank_five_center_bb & pawn_bb).count_bits() as i32 * 25;

    let rank_four_center_bb = match player {
        Player::White => BitBoard::RANK_4 & center_files_bb,
        Player::Black => BitBoard::RANK_5 & center_files_bb,
    };
    value += (rank_four_center_bb & pawn_bb).count_bits() as i32 * 20;

    // Not developing the center pawns is bad
    let rank_two_center_bb = match player {
        Player::White => BitBoard::RANK_2 & center_files_bb,
        Player::Black => BitBoard::RANK_7 & center_files_bb,
    };
    value += (rank_two_center_bb & pawn_bb).count_bits() as i32 * -20;

    value
}

/// Evaluate the position of the bishops.
fn player_bishop_position(board: &Board, player: Player) -> i32 {
    let bishop_bb = board.piece_bb(player, PieceType::B);

    // Avoid having bishops on the borders
    count_border_pieces(bishop_bb) as i32 * -25
}

/// Evaluate the position of the rooks.
fn player_rook_position(board: &Board, player: Player) -> i32 {
    let rook_bb = board.piece_bb(player, PieceType::R);
    let mut value = 0;

    // Being on the 7th rank is good
    let rank_seven_bb = match player {
        Player::White => BitBoard::RANK_7,
        Player::Black => BitBoard::RANK_2,
    };
    value += (rank_seven_bb & rook_bb).count_bits() as i32 * 15;

    // Being in the center is good
    let center_bb = match player {
        Player::White => SQ::D1.to_bb() | SQ::E1.to_bb(),
        Player::Black => SQ::D8.to_bb() | SQ::E8.to_bb(),
    };
    value += (center_bb & rook_bb).count_bits() as i32 * 10;

    // Avoid the left and right borders
    let border_bb = (BitBoard::FILE_A | BitBoard::FILE_H) & (BitBoard::RANK_1 | BitBoard::RANK_8);
    value += (border_bb & rook_bb).count_bits() as i32 * -5;

    value
}

/// Evaluate the position of the queens.
fn player_queen_position(board: &Board, player: Player) -> i32 {
    let queen_bb = board.piece_bb(player, PieceType::Q);

    // Avoid having queens on the borders
    count_border_pieces(queen_bb) as i32 * -10
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
