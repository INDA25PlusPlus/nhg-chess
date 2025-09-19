use crate::piece::{Piece, Color};
use crate::position::Position;
use crate::moves::{Move, piece_indexes};

/*
- Pawn !
X Promotion: when it reaches the last rank (rank 8 for White, rank 1 for Black) become anything it wants ( queen, rook, bishop, or knight )
https://en.wikipedia.org/wiki/Promotion_(chess) 
- The new piece does not have to be a previously captured piece
- Promotion is mandatory when moving to the last rank; the pawn cannot remain as a pawn. 

X En passant: capturing a pawn that just moved two squares. (??) 
https://www.chessprogramming.org/En_passant
https://www.chessprogramming.org/Forsyth-Edwards_Notation#Enpassanttargetsquare
- track one square at a time

- King 
X Castling: moves two squares left or right if rook + king haven’t moved, no check along the path.
- Castling is permitted only if neither the king nor the rook has previously moved
- flagga för "has moved" för rook/king? 
- kräver flow ish
*/

/// Checks whether a pawn move reaches its promotion rank.
///
/// # Arguments
///
/// * `to` — The destination square index (0–63).
/// * `piece` — The pawn being moved.
///
/// # Returns
///
/// * `true` if the pawn lands on its promotion rank (rank 8 for White, rank 1 for Black).
/// * `false` otherwise.
pub fn is_pawn_promotion(to: u8, piece: Piece) -> bool {
    match piece {
        Piece::Pawn(Color::White) => to >= 56, // rank 8
        Piece::Pawn(Color::Black) => to < 8,   // rank 1
        _ => false,
    }
}

/// Returns the possible promotion pieces for a pawn that reaches the last rank.
///
/// By chess rules, promotion is mandatory, and the pawn can be promoted to
/// a queen, rook, bishop, or knight of the same color.
///
/// # Arguments
///
/// * `piece` — The pawn being promoted.
///
/// # Returns
///
/// A `Vec<Piece>` containing the possible promotion outcomes.
pub fn valid_pawn_promotions(piece: Piece) -> Vec<Piece> {
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

/// Generates *pseudo-legal* castling moves for a king, if available, based on castling rights
/// and whether the squares between the king and rook are empty.
///
/// Castling is only possible if:
/// - The king and the involved rook have not previously moved.
/// - The path between the king and rook is empty.
///
/// # Arguments
///
/// * `from` — The square index of the king (should be `4` for White, `60` for Black).
/// * `piece` — The king piece (must be `Piece::King(Color::White|Black)`).
/// * `position` — The current position, including castling rights and occupancy.
///
/// # Returns
///
/// A vector of possible castling moves. May be empty if no castling is available.
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
                        moves.push(Move { from, to: 6, piece, promoted_from_pawn: false });
                    }
                }
                // queenside
                if !cr.white_king_moved && !cr.white_queenside_rook_moved {
                    let empty = !(position.bb_sides[own_index].0 | position.bb_sides[enemy_index].0);
                    if (empty & (1<<1 | 1<<2 | 1<<3)) == (1<<1 | 1<<2 | 1<<3) {
                        moves.push(Move { from, to: 2, piece, promoted_from_pawn: false });
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
                        moves.push(Move { from, to: 62, piece, promoted_from_pawn: false,});
                    }
                }
                // queenside
                if !cr.black_king_moved && !cr.black_queenside_rook_moved {
                    let empty = !(position.bb_sides[own_index].0 | position.bb_sides[enemy_index].0);
                    if (empty & (1<<57 | 1<<58 | 1<<59)) == (1<<57 | 1<<58 | 1<<59) {
                        moves.push(Move { from, to: 58, piece, promoted_from_pawn: false, });
                    }
                }
            }
        }
    }

    moves
}