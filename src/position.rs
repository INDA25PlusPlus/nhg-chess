use crate::bitboard::BitBoard;
use crate::piece::{Piece, Color};

// is copy needed? idk. added for debug

/* 
Bitboard set-up:
8 | 56 57 58 59 60 61 62 63
7 | 48 49 50 51 52 53 54 55
6 | 40 41 42 43 44 45 46 47
5 | 32 33 34 35 36 37 38 39
4 | 24 25 26 27 28 29 30 31
3 | 16 17 18 19 20 21 22 23
2 |  8  9 10 11 12 13 14 15
1 |  0  1  2  3  4  5  6  7
    a  b  c  d  e  f  g  h
*/

/// Represents the "depth" or "layers" of the bitboard. A "side" then is the color (white side, black side).
#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Position{
    //>>>>  NOTEEEE remove pub after testingg (???)

    /// Shows all the occupied positions given a color, irregardless of piece type. (A queen is represented the same as a pawn)
    pub bb_sides: [BitBoard; 2],
    /// Shows the positions of each piece-color combination, i.e. the location of White Rook (if any).
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

/*
Example Functionality (Bitmasking): 
Piece BitBoard:         0b0001000000001
"Spotlight"" for D4:    0b0001000000000
AND result:             0b0001000000000 =! 0    -> square has a piece 
*/

/// Gets the existence, type and color of a piece given a position expressed as a bit-index (0-63), where 0 is A1 and 63 is H8.
/// Expresses the desired bit-index as a hex (mask). The mask (called "spotlight") is compared to the BitBoards of the pieces for each color and type using AND
/// If the mask matches one of the boards (bb_pieces), it matches the equivalent BitBoard to associated Piece. 
/// 
/// Example Functionality (Bitmasking): \
/// Piece BitBoard:         0b0001000000001\
/// "Spotlight"" for D4:    0b0001000000000\
/// AND result:             0b0001000000000 =! 0    -> square has a piece 
pub fn get_piece_at(position: &Position, square: u8) -> Option<Piece> {
    let spotlight = 1u64 << square; 
    println!("spotlight: {:064b}", spotlight);

    for side in [Sides::WHITE, Sides::BLACK] {
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
