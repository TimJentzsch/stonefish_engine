//! Evaluation of the positional value.
//!
//! Values inspired by https://www.chessprogramming.org/Simplified_Evaluation_Function
use pleco::{BitBoard, BitMove, Board, PieceType, Player, SQ};

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
// An array of all rank bitboards.
pub const RANK_BBS: [BitBoard; 8] = [
    BitBoard::RANK_1,
    BitBoard::RANK_2,
    BitBoard::RANK_3,
    BitBoard::RANK_4,
    BitBoard::RANK_5,
    BitBoard::RANK_6,
    BitBoard::RANK_7,
    BitBoard::RANK_8,
];

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
        println!("Endgame!!");
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
        value += score_position(piece_bb, castle_bb, 50);

        // Don't stand around in the center
        let center_bb = match player {
            Player::White => SQ::E1.to_bb() | SQ::D1.to_bb() | SQ::E2.to_bb() | SQ::D2.to_bb(),
            Player::Black => SQ::E8.to_bb() | SQ::D8.to_bb() | SQ::E7.to_bb() | SQ::D7.to_bb(),
        };
        value += score_position(piece_bb, center_bb, -20);

        value
    }
}

/// Get the bitboard of the given rank from the player's perspective.
fn get_player_rank_bb(rank: usize, player: Player) -> BitBoard {
    let index = if player == Player::White {
        rank - 1
    } else {
        8 - rank
    };

    RANK_BBS[index]
}

/// Evaluate the position of the pawns.
fn player_pawn_position(_board: &Board, piece_bb: BitBoard, player: Player) -> i32 {
    let mut value = 0;

    let center_files_bb = BitBoard::FILE_D | BitBoard::FILE_E;

    // Being close to promotion is good
    value += (get_player_rank_bb(7, player) & piece_bb).count_bits() as i32 * 70;
    value += (get_player_rank_bb(6, player) & piece_bb).count_bits() as i32 * 50;
    // Developing the center pawns is good
    value += (get_player_rank_bb(5, player) & center_files_bb & piece_bb).count_bits() as i32 * 40;
    value += (get_player_rank_bb(4, player) & center_files_bb & piece_bb).count_bits() as i32 * 35;
    // Not developing the center pawns is bad
    value += (get_player_rank_bb(2, player) & center_files_bb & piece_bb).count_bits() as i32 * -40;

    value
}

/// Evaluate the position of the knights.
fn player_knight_position(_board: &Board, piece_bb: BitBoard) -> i32 {
    let mut value = 0;

    // Avoid having knights on the borders
    value += score_position(piece_bb, BORDER_BB, -25);
    value += score_position(piece_bb, CORNER_BB, -20);
    // Move knights to the center
    value += score_position(piece_bb, CENTER_ONE_BB, 15);
    value += score_position(piece_bb, CENTER_RING_BB, 5);

    value
}

/// Evaluate the position of the bishops.
fn player_bishop_position(_board: &Board, piece_bb: BitBoard) -> i32 {
    let mut value = 0;

    // Avoid having bishops on the borders
    value += score_position(piece_bb, BORDER_BB, -25);
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
    value += score_position(piece_bb, rank_seven_bb, 25);

    // Being in the center is good
    let center_bb = match player {
        Player::White => SQ::D1.to_bb() | SQ::E1.to_bb(),
        Player::Black => SQ::D8.to_bb() | SQ::E8.to_bb(),
    };
    value += score_position(piece_bb, center_bb, 20);

    // Being in the center after castling is also good
    let center_bb = match player {
        Player::White => SQ::F1.to_bb(),
        Player::Black => SQ::F8.to_bb(),
    };
    value += score_position(piece_bb, center_bb, 10);

    // Avoid the left and right borders
    let border_bb = (BitBoard::FILE_A | BitBoard::FILE_H) ^ CORNER_BB;
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

pub fn threat_value(board: &Board) -> i32 {
    let mut value = 0;
    let piece_locations = board.get_piece_locations();

    for (sq, piece) in piece_locations {
        let piece_type = piece.type_of();
        let player = match piece.player() {
            Some(p) => p,
            None => continue,
        };

        if !piece_type.is_real() || player == board.turn() {
            continue;
        }

        let mut defenders = vec![];
        let mut attackers = vec![];

        // Determine all defenders and attackers of this piece
        for attacker_sq in board.attackers_to(sq, board.occupied()) {
            let other_piece = board.piece_at_sq(attacker_sq);
            let other_piece_type = other_piece.type_of();
            let other_player = match other_piece.player() {
                Some(p) => p,
                None => continue,
            };

            if !piece_type.is_real() {
                continue;
            }

            if player == other_player {
                defenders.push(other_piece_type);
            } else {
                attackers.push(other_piece_type);
            }
        }

        if defenders.len() == 0 && attackers.len() > 0 {
            value += get_piece_value(piece_type);
        }
    }

    value
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
    let src_sq = mv.get_src();
    let dest_sq = mv.get_dest();

    let old_piece = old_board.piece_at_sq(src_sq).type_of();
    // The new piece can be different (if promoting)
    let new_piece = new_board.piece_at_sq(dest_sq).type_of();

    let old_pos_eval = positional_piece_value(old_piece, old_board, src_sq.to_bb(), player);
    let new_pos_eval = positional_piece_value(new_piece, new_board, dest_sq.to_bb(), player);
    // We also need to consider the change of capturing an opponent's piece
    let capture_eval = if mv.is_capture() {
        let capture_piece = old_board.piece_at_sq(dest_sq).type_of();
        positional_piece_value(
            capture_piece,
            old_board,
            dest_sq.to_bb(),
            player.other_player(),
        )
    } else {
        0
    };

    new_pos_eval - old_pos_eval + capture_eval
}

#[cfg(test)]
mod tests {
    use pleco::{BitBoard, Board, PieceType, Player, SQ};

    use crate::stonefish::heuristic::positional_value::{
        initial_positional_value, player_king_position, player_knight_position,
        player_rook_position, threat_value, BORDER_BB, CENTER_ONE_BB, CENTER_RING_BB,
        CENTER_TWO_BB, CORNER_BB,
    };

    use super::player_pawn_position;

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

    #[test]
    fn should_calculate_player_pawn_position() {
        // A FEN string with the corresponding evaluation
        // The position should be symmetrical for both sides
        let parameters = [
            // No pawns pushed
            ("4k3/pppppppp/8/8/8/8/PPPPPPPP/4K3 w - - 0 1", -80),
            // Single pushed e pawn
            ("4k3/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/4K3 w - - 0 1", -40),
            // Double pushed e pawn
            ("4k3/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/4K3 w - - 0 1", -5),
            // Single pushed d pawn
            ("4k3/ppp1pppp/3p4/8/8/3P4/PPP1PPPP/4K3 w - - 0 1", -40),
            // Double pushed d pawn
            ("4k3/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/4K3 w - - 0 1", -5),
            // Single pushed both e and d pawn
            ("4k3/ppp2ppp/3pp3/8/8/3PP3/PPP2PPP/4K3 w - - 0 1", 0),
            // Double pushed both e and d pawn
            ("4k3/ppp2ppp/8/3pp3/3PP3/8/PPP2PPP/4K3 w - - 0 1", 70),
            // Almost promotion b pawn
            ("4k3/pPp2ppp/8/8/8/8/PpP2PPP/4K3 w - - 0 1", 70),
        ];

        for (fen, expected) in parameters {
            let board = Board::from_fen(fen).unwrap();

            let actual_white = player_pawn_position(
                &board,
                board.piece_bb(Player::White, PieceType::P),
                Player::White,
            );
            let actual_black = player_pawn_position(
                &board,
                board.piece_bb(Player::Black, PieceType::P),
                Player::Black,
            );

            assert_eq!(actual_white, expected, "Evaluation wrong for White: {fen}");
            assert_eq!(actual_black, expected, "Evaluation wrong for Black: {fen}");
        }
    }

    #[test]
    fn should_calculate_player_knight_position() {
        // A FEN string with the corresponding evaluation
        // The position should be symmetrical for both sides
        let parameters = [
            // Start position knights
            ("1n2k1n1/8/8/8/8/8/8/1N2K1N1 w - - 0 1", -50),
            // Single developed b knight
            ("4k3/8/2n5/8/8/2N5/8/4K3 w - - 0 1", 5),
            // b knight developed
            ("4k1n1/8/2n5/8/8/2N5/8/4K1N1 w - - 0 1", -20),
            // Single developed g knight
            ("4k3/8/5n2/8/8/5N2/8/4K3 w - - 0 1", 5),
            // g knight developed
            ("1n2k3/8/5n2/8/8/5N2/8/1N2K3 w - - 0 1", -20),
            // one corner knight
            ("n3k3/8/8/8/8/8/8/N3K3 w - - 0 1", -45),
        ];

        for (fen, expected) in parameters {
            let board = Board::from_fen(fen).unwrap();

            let actual_white =
                player_knight_position(&board, board.piece_bb(Player::White, PieceType::N));
            let actual_black =
                player_knight_position(&board, board.piece_bb(Player::Black, PieceType::N));

            assert_eq!(actual_white, expected, "Evaluation wrong for White: {fen}");
            assert_eq!(actual_black, expected, "Evaluation wrong for Black: {fen}");
        }
    }

    #[test]
    fn should_calculate_player_king_position() {
        // A FEN string with the corresponding evaluation
        // The position should be symmetrical for both sides
        let parameters = [
            // Start position king
            ("1q2k3/8/8/8/8/8/8/2Q1K3 w - - 0 1", -30),
            // Castled king short
            ("1q3rk1/8/8/8/8/8/8/2Q2RK1 w - - 0 1", 50),
            // Castled king long
            ("2kr2q1/8/8/8/8/8/8/2KR1Q2 w - - 0 1", 50),
        ];

        for (fen, expected) in parameters {
            let board = Board::from_fen(fen).unwrap();

            let actual_white = player_king_position(
                &board,
                board.piece_bb(Player::White, PieceType::K),
                Player::White,
            );
            let actual_black = player_king_position(
                &board,
                board.piece_bb(Player::Black, PieceType::K),
                Player::Black,
            );

            assert_eq!(actual_white, expected, "Evaluation wrong for White: {fen}");
            assert_eq!(actual_black, expected, "Evaluation wrong for Black: {fen}");
        }
    }

    #[test]
    fn should_calculate_player_rook_position() {
        // A FEN string with the corresponding evaluation
        // The position should be symmetrical for both sides
        let parameters = [
            // Start position h rook
            ("1q2k3/8/8/8/8/8/8/2Q1K3 w - - 0 1", 0),
            // Castled h rook
            ("5rk1/8/8/8/8/8/8/5RK1 w - - 0 1", 10),
            // Center h rook
            ("2kr4/8/8/8/8/8/8/2KR4 w - - 0 1", 20),
            // Start position a rook
            ("r3k3/8/8/8/8/8/8/R3K3 w - - 0 1", 0),
            // Castled a rook
            ("2kr4/8/8/8/8/8/8/2KR4 w - - 0 1", 20),
        ];

        for (fen, expected) in parameters {
            let board = Board::from_fen(fen).unwrap();

            let actual_white = player_rook_position(
                &board,
                board.piece_bb(Player::White, PieceType::R),
                Player::White,
            );
            let actual_black = player_rook_position(
                &board,
                board.piece_bb(Player::Black, PieceType::R),
                Player::Black,
            );

            assert_eq!(actual_white, expected, "Evaluation wrong for White: {fen}");
            assert_eq!(actual_black, expected, "Evaluation wrong for Black: {fen}");
        }
    }

    #[test]
    fn should_prefer_good_openings() {
        // The left side is the better opening, the right side the worse one
        let parameters = [
            (
                "e2e4 is better than b8c3",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 1",
            ),
            (
                "e2e4 is better than g1g3",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 1",
            ),
            (
                "e2e4 is better than e2e3",
                "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
                "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1",
            ),
            (
                "pure castling is better than walking the king",
                "1k1q4/8/8/8/8/8/8/3Q1RK1 w - - 0 1",
                "1k1q4/8/8/8/8/8/8/3Q2KR w - - 0 1",
            ),
            (
                "position castling is better than walking the king first",
                "rnbqk1nr/pp1p1pbp/4p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 b kq - 1 5",
                "rnbqk1nr/pp1p1pbp/4p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ1K1R b kq - 1 5",
            ),
            (
                "position castling is better than walking the king second",
                "r1bqk1nr/pp1p1pbp/2n1p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ2KR b kq - 3 6",
                "r1bqk1nr/pp1p1pbp/2n1p1p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQ1RK1 w kq - 2 6",
            ),
        ];

        for (name, fen_better, fen_worse) in parameters {
            let board_better = Board::from_fen(fen_better).unwrap();
            let board_worse = Board::from_fen(fen_worse).unwrap();

            let eval_better = initial_positional_value(&board_better);
            let eval_worse = initial_positional_value(&board_worse);

            assert!(
                eval_better > eval_worse,
                "{eval_better} <= {eval_worse} {name}"
            );
        }
    }

    #[test]
    fn test_threat_value() {
        let parameters = [
            (
                "unprotected white rook",
                "4k3/8/2r5/8/8/2R5/8/4K3 b - - 0 1",
                500,
            ),
            (
                "unprotected black rook",
                "4k3/8/2r5/8/8/2R5/8/4K3 w - - 0 1",
                500,
            ),
            (
                "protected white rook with bishop",
                "4k3/8/2r5/8/8/2R5/1B6/4K3 b - - 0 1",
                0,
            ),
            (
                "protected white rook with pawn",
                "4k3/8/2r5/8/8/2R5/1P6/4K3 b - - 0 1",
                0,
            ),
            (
                "protected black rook with bishop",
                "4k3/1b6/2r5/8/8/2R5/8/4K3 w - - 0 1",
                0,
            ),
            (
                "protected black rook with pawn",
                "4k3/1p6/2r5/8/8/2R5/8/4K3 w - - 0 1",
                0,
            ),
            (
                "bad opening",
                "rnbqkb1r/pp1ppppp/5n2/8/2p5/N7/PPPPPPPP/R1BQKBNR w KQkq - 0 4",
                100,
            ),
        ];

        for (name, fen, expected) in parameters {
            let board = Board::from_fen(fen).unwrap();
            let threat_value = threat_value(&board);

            assert_eq!(threat_value, expected, "{name}");
        }
    }
}
