// position.rs

use crate::bitboard::BitBoard;
use crate::piece::{Piece, Color};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Position{
    //>>>>  NOTEEEE remove pub after testing
    /// Board for each side
    pub bb_sides: [BitBoard; 2],
    // BitBoards for all pieces and each side
    pub bb_pieces: [[BitBoard; 6]; 2],
}

pub struct Sides;
impl Sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

pub struct Pieces;
impl Pieces{
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

// example: let white_queens: BitBoard = position.bb_pieces[Sides::WHITE][Pieces::QUEEN]; 

// chess board stored as 2d array (? enum or bitboard) ; A->H, 1->8. returns 0 if no piece, returns associated string if piece (i.e. Q = queen)
// string expensive (boo), use enum: i.e. Piece::Queen(Color::White)
// how to keep track white/black?

/* bitmask functionality:
piece bitboard:         0b0001000000001
"spotlight"" for D4:    0b0001000000000
AND result:             0b0001000000000 =/= 0 -> square has a piece */

pub fn get_piece_at(position: &Position, square: u8) -> Option<Piece> {
    let spotlight = 1u64 << square; 
    println!("spotlight: {:064b}", spotlight);

    for side in [Sides::WHITE, Sides::BLACK] {
        println!("side:", side);
        for piece_type in 0..6 {
            println!("position in board: {:064b}", position.bb_pieces[side][piece_type].0);
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
