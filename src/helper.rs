use crate::piece::CastlingRights;
use crate::position::Position;
use crate::bitboard::BitBoard;
use crate::position::{Pieces, Sides};

/// Initializes a chessboard to the standard starting position.
///
/// - Places all pawns, rooks, knights, bishops, queens, and kings
///   for both White and Black.
/// - Fills in the side masks (`bb_sides`) by OR-ing all piece bitboards.
/// - Castling rights are set to the default (none of the pieces have moved).
/// - No en passant target square is set initially.
///
/// Returns:
/// - A [`Position`] struct representing the standard starting chessboard.
pub fn initialize_board() -> Position {
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(0)],
        bb_pieces: [[BitBoard(0); 6]; 2], 
        castling_rights: CastlingRights::new(),
        en_passant: None,
    };

    // White pieces
    position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(
        (0..8).map(|i| 1u64 << (8 + i)).fold(0, |acc, v| acc | v),
    ); 
    position.bb_pieces[Sides::WHITE][Pieces::ROOK] =
        BitBoard((1 << 0) | (1 << 7));
    position.bb_pieces[Sides::WHITE][Pieces::KNIGHT] =
        BitBoard((1 << 1) | (1 << 6));
    position.bb_pieces[Sides::WHITE][Pieces::BISHOP] =
        BitBoard((1 << 2) | (1 << 5));
    position.bb_pieces[Sides::WHITE][Pieces::QUEEN] = BitBoard(1 << 3); 
    position.bb_pieces[Sides::WHITE][Pieces::KING] = BitBoard(1 << 4);

    // Black pieces
    position.bb_pieces[Sides::BLACK][Pieces::PAWN] = BitBoard(
        (0..8).map(|i| 1u64 << (48 + i)).fold(0, |acc, v| acc | v),
    );
    position.bb_pieces[Sides::BLACK][Pieces::ROOK] =
        BitBoard((1 << 56) | (1 << 63));
    position.bb_pieces[Sides::BLACK][Pieces::KNIGHT] =
        BitBoard((1 << 57) | (1 << 62));
    position.bb_pieces[Sides::BLACK][Pieces::BISHOP] =
        BitBoard((1 << 58) | (1 << 61));
    position.bb_pieces[Sides::BLACK][Pieces::QUEEN] = BitBoard(1 << 59); 
    position.bb_pieces[Sides::BLACK][Pieces::KING] = BitBoard(1 << 60); 

    // Fill side masks
    position.bb_sides[Sides::WHITE].0 = position.bb_pieces[Sides::WHITE]
        .iter()
        .fold(0u64, |acc, bb| acc | bb.0);
    position.bb_sides[Sides::BLACK].0 = position.bb_pieces[Sides::BLACK]
        .iter()
        .fold(0u64, |acc, bb| acc | bb.0);

    position
}

/// Converts a bit index (0..63) to chess notation, e.g., 0 -> "A1", 63 -> "H8"
pub fn index_to_square(index: u8) -> String {
    let file = (index % 8) as u8;
    let rank = (index / 8) as u8;
    let file_char = (b'A' + file) as char;
    let rank_char = (b'1' + rank) as char;
    format!("{}{}", file_char, rank_char)
}

/// Converts chess notation ("A1".."H8") to bit index (0..63)
pub fn square_to_index(square: &str) -> Option<u8> {
    if square.len() != 2 {
        return None;
    }
    let mut chars = square.chars();
    let file = chars.next()?.to_ascii_uppercase();
    let rank = chars.next()?;

    if !('A'..='H').contains(&file) || !('1'..='8').contains(&rank) {
        return None;
    }

    let file_idx = file as u8 - b'A';
    let rank_idx = rank as u8 - b'1';
    Some(rank_idx * 8 + file_idx)
}

/// Prints the current board state in a human-readable format for debugging.
///
/// - Iterates through all 64 squares and checks if they are occupied.
/// - If a piece is present, prints its side (`W` for White, `B` for Black)
///   and type (`P`, `N`, `B`, `R`, `Q`, `K`). (Note: Knight -> `N`)
/// - Squares with no pieces show their square name (e.g., `"A1"`).
/// - Colors pieces in the terminal (White = red, Black = green).
///
/// Example output (truncated):
/// ```text
/// BR BN BB BQ BK BB BN BR 
/// BP BP BP BP BP BP BP BP 
/// A7 B7 C7 D7 E7 F7 G7 H7 
/// ...
/// WP WP WP WP WP WP WP WP 
/// WR WN WB WQ WK WB WN WR 
/// ```
pub fn print_debug_board(position: &Position) {
    for row in (0..8).rev() {
        for col in 0..8 {
            let square: i32 = row * 8 + col;
            let mask = 1u64 << square;
            let mut output = index_to_square(square as u8);

            for side in [Sides::WHITE, Sides::BLACK] {
                for (i, bb) in position.bb_pieces[side].iter().enumerate() {
                    if (bb.0 & mask) != 0 {
                        let color_code = if side == Sides::WHITE {
                            "\x1b[31m"
                        } else {
                            "\x1b[32m"
                        }; // red / green
                        let piece_char = match i {
                            0 => "P",
                            1 => "N",
                            2 => "B",
                            3 => "R",
                            4 => "Q",
                            5 => "K",
                            _ => "?",
                        };
                        output = format!(
                            "{}{}{}\x1b[0m",
                            color_code,
                            if side == Sides::WHITE { "W" } else { "B" },
                            piece_char
                        );
                    }
                }
            }
            print!("{} ", output);
        }
        println!();
    }
    println!();
}
