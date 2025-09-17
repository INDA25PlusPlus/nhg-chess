//use crate::position::Position;
use crate::piece::{Piece, Color};
//use crate::moves::Move;

/*
- Pawn !
X Promotion: when it reaches the last rank (rank 8 for White, rank 1 for Black) become anything it wants ( queen, rook, bishop, or knight )
https://en.wikipedia.org/wiki/Promotion_(chess) 
- The new piece does not have to be a previously captured piece
- Promotion is mandatory when moving to the last rank; the pawn cannot remain as a pawn. 

En passant: capturing a pawn that just moved two squares. (??)

- King !
Castling: moves two squares left or right if rook + king haven’t moved, no check along the path.
*/

/// Check if a pawn move reaches its promotion rank (row).
pub fn is_pawn_promotion(to: u8, piece: Piece) -> bool {
    match piece {
        Piece::Pawn(Color::White) => to >= 56, // rank 8
        Piece::Pawn(Color::Black) => to < 8,   // rank 1
        _ => false,
    }
}

// from / to is needed for valid_pawn_moves; can this be cleaned?
pub fn valid_pawn_promotions(_from: u8, _to: u8, piece: Piece) -> Vec<Piece> {
    match piece {
        Piece::Pawn(Color::White) => vec![
            Piece::Queen(Color::White),
            Piece::Rook(Color::White),
            Piece::Bishop(Color::White),
            Piece::Knight(Color::White),
        ],
        Piece::Pawn(Color::Black) => vec![
            Piece::Queen(Color::Black),
            Piece::Rook(Color::Black),
            Piece::Bishop(Color::Black),
            Piece::Knight(Color::Black),
        ],
        _ => vec![],
    }
}