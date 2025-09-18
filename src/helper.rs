use crate::piece::CastlingRights;
use crate::position::Position;
use crate::bitboard::BitBoard;
use crate::position::{Pieces, Sides};

pub fn initialize_board() -> Position {
    let mut position = Position {
        bb_sides: [BitBoard(0), BitBoard(0)],
        bb_pieces: [[BitBoard(0); 6]; 2], 
        castling_rights: CastlingRights::new(),
    };

    // White pieces
    /* position.bb_pieces[Sides::WHITE][Pieces::PAWN] = BitBoard(
        (0..8).map(|i| 1u64 << (8 + i)).fold(0, |acc, v| acc | v),
    ); */
    position.bb_pieces[Sides::WHITE][Pieces::ROOK] =
        BitBoard((1 << 0) | (1 << 7));
    /* position.bb_pieces[Sides::WHITE][Pieces::KNIGHT] =
        BitBoard((1 << 1) | (1 << 6));
    position.bb_pieces[Sides::WHITE][Pieces::BISHOP] =
        BitBoard((1 << 2) | (1 << 5));
    position.bb_pieces[Sides::WHITE][Pieces::QUEEN] = BitBoard(1 << 3); */
    position.bb_pieces[Sides::WHITE][Pieces::KING] = BitBoard(1 << 4);

    // Black pieces
    /* position.bb_pieces[Sides::BLACK][Pieces::PAWN] = BitBoard(
        (0..8).map(|i| 1u64 << (48 + i)).fold(0, |acc, v| acc | v),
    ); */
    position.bb_pieces[Sides::BLACK][Pieces::ROOK] =
        BitBoard((1 << 56) | (1 << 63));
    /* position.bb_pieces[Sides::BLACK][Pieces::KNIGHT] =
        BitBoard((1 << 57) | (1 << 62));
    position.bb_pieces[Sides::BLACK][Pieces::BISHOP] =
        BitBoard((1 << 58) | (1 << 61));
    position.bb_pieces[Sides::BLACK][Pieces::QUEEN] = BitBoard(1 << 59); */
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
