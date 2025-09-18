use crate::piece::{Piece, Color};
use crate::position::Position;
use crate::moves::{Move, piece_indexes};

/*
- Pawn !
X Promotion: when it reaches the last rank (rank 8 for White, rank 1 for Black) become anything it wants ( queen, rook, bishop, or knight )
https://en.wikipedia.org/wiki/Promotion_(chess) 
- The new piece does not have to be a previously captured piece
- Promotion is mandatory when moving to the last rank; the pawn cannot remain as a pawn. 

En passant: capturing a pawn that just moved two squares. (??)
- kräver ett "flow"
- använda flagga? 

- King !
Castling: moves two squares left or right if rook + king haven’t moved, no check along the path.
- Castling is permitted only if neither the king nor the rook has previously moved
- flagga för "has moved" för rook/king? 
- kräver flow ish
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

pub fn castling_moves(from: u8, piece: Piece, position: &Position) -> Vec<Move> {
    let mut moves = Vec::new();
    let cr = &position.castling_rights;
    let (own_index, enemy_index) = piece_indexes(piece);

    match piece.color() {
        Color::White => {
            if from == 4 { 
                // kingside
                if !cr.white_king_moved && !cr.white_kingside_rook_moved {
                    let empty = !(position.bb_sides[own_index].0 | position.bb_sides[enemy_index].0);
                    if (empty & (1<<5 | 1<<6)) == (1<<5 | 1<<6) {
                        moves.push(Move { from, to: 6, piece });
                    }
                }
                // queenside
                if !cr.white_king_moved && !cr.white_queenside_rook_moved {
                    let empty = !(position.bb_sides[own_index].0 | position.bb_sides[enemy_index].0);
                    if (empty & (1<<1 | 1<<2 | 1<<3)) == (1<<1 | 1<<2 | 1<<3) {
                        moves.push(Move { from, to: 2, piece });
                    }
                }
            }
        }
        Color::Black => {
            if from == 60 { 
                // kingside
                if !cr.black_king_moved && !cr.black_kingside_rook_moved {
                    let empty = !(position.bb_sides[own_index].0 | position.bb_sides[enemy_index].0);
                    if (empty & (1<<61 | 1<<62)) == (1<<61 | 1<<62) {
                        moves.push(Move { from, to: 62, piece });
                    }
                }
                // queenside
                if !cr.black_king_moved && !cr.black_queenside_rook_moved {
                    let empty = !(position.bb_sides[own_index].0 | position.bb_sides[enemy_index].0);
                    if (empty & (1<<57 | 1<<58 | 1<<59)) == (1<<57 | 1<<58 | 1<<59) {
                        moves.push(Move { from, to: 58, piece });
                    }
                }
            }
        }
    }

    moves
}