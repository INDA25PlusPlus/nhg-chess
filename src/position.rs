use crate::bitboard::BitBoard;
use crate::piece::{Piece, Color, CastlingRights};

// is copy needed? idk. added for debug

/// Represents the state of the chessboard using bitboards.
///
/// A [`Position`] stores which squares are occupied by which side and piece types,
/// along with castling rights and en passant information.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Position{
    /// Bitboards for all occupied squares, separated by side.
    ///
    /// Index `0`: White pieces, index `1`: Black pieces.  
    /// This does not differentiate by piece type (a pawn and queen are treated equally).
    pub bb_sides: [BitBoard; 2],
    /// Bitboards for each piece type, separated by side.
    ///
    /// Accessed as `bb_pieces[side][piece_type]`.  
    /// For example, `bb_pieces[Sides::WHITE][Pieces::ROOK]` gives the bitboard
    /// for White’s rooks.
    pub bb_pieces: [[BitBoard; 6]; 2],
    pub castling_rights: CastlingRights,
    /// The en passant target square, if available.
    ///
    /// If `Some(u8)`, it is the square index (0–63, where 0 = A1, 63 = H8).  
    /// If `None`, no en passant is available.
    pub en_passant: Option<u8>, // None or bit position
}
/// Constants representing the two sides in a chess game.
pub struct Sides;
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

/// Constants representing the piece types.
///
/// Indexed as `0..=5`, corresponding to pawn -> king.
pub struct Pieces;
impl Pieces{
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

/*
Example Functionality (Bitmasking): 
Piece BitBoard:         0b0001000000001
"Spotlight"" for D4:    0b0001000000000
AND result:             0b0001000000000 =! 0    -> square has a piece 
*/

/// Returns the piece (if any) at a given square in the [`Position`].
///
/// # Arguments
///
/// * `position` — The chess position to query.
/// * `square` — A square index in the range `0..=63`, where:
///   - `0` = A1  
///   - `63` = H8
///
/// # Returns
///
/// * `Some(Piece)` if a piece is found at the given square.
/// * `None` if the square is empty.
pub fn get_piece_at(position: &Position, square: u8) -> Option<Piece> {
    let spotlight = 1u64 << square; 
    //println!("spotlight: {:064b}", spotlight);

    for side in [Sides::WHITE, Sides::BLACK] {
        for piece_type in 0..6 {
            //println!("position in board: {:064b}", position.bb_pieces[side][piece_type].0);
            if (position.bb_pieces[side][piece_type].0 & spotlight) != 0 {
                let color = if side == Sides::WHITE {  
                    Color::White
                } else {
                    Color::Black
                };

                let piece = match piece_type {
                    Pieces::PAWN => Piece::Pawn(color),
                    Pieces::KNIGHT => Piece::Knight(color),
                    Pieces::BISHOP => Piece::Bishop(color),
                    Pieces::ROOK => Piece::Rook(color),
                    Pieces::QUEEN => Piece::Queen(color),
                    Pieces::KING => Piece::King(color),
                    _ => unreachable!(),
                };

                return Some(piece);
            }
        }
    }
    None
}
